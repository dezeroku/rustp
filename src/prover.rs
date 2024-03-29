use crate::ast::*;
use crate::set_nonmut as set;
use log;
use std::collections::HashSet;
use std::convert::TryInto;
use z3;

#[cfg(test)]
mod tests;

#[cfg(feature = "bounds_prove")]
mod bitvector;
#[cfg(feature = "bounds_prove")]
use bitvector::*;

#[cfg(not(feature = "bounds_prove"))]
mod int;
#[cfg(not(feature = "bounds_prove"))]
use int::*;

fn prove_block(precondition: Bool, code: Vec<Command>, postcondition: Bool) -> ProveBlock {
    ProveBlock {
        precondition: precondition.clone(),
        code: code,
        postcondition: postcondition.clone(),
        precondition_original: precondition,
        postcondition_original: postcondition,
    }
}

fn prove_impl(p: Bool, q: Bool) -> bool {
    let mut cfg = z3::Config::new();
    cfg.set_model_generation(true);

    let ctx = z3::Context::new(&cfg);
    let t = z3::Solver::new(&ctx);

    let (mut checks, _pre) = p.clone().as_bool(&ctx);
    let (checks_post, _post) = q.clone().as_bool(&ctx);
    checks.extend(checks_post);

    // Check overflows and underflows
    for i in checks {
        t.reset();
        t.assert(&_pre.implies(&i.check).not());
        let f = t.check();
        if f != z3::SatResult::Unsat {
            log::warn!("Bounds check warning: possible {} {}", i.name, i.object);
        }
    }
    t.reset();

    t.assert(&_pre.implies(&_post).not());

    let f = t.check();
    log::debug!("{:?}", f);
    log::debug!("{:?}", t.get_model());
    let result = Some(f);

    match result {
        Some(z3::SatResult::Sat) => {
            log::debug!("Model: {:?}", t.get_model());
            log::info!("Failed to prove: {} => {}", p, q);
            return false;
        }
        Some(z3::SatResult::Unsat) => {
            log::info!("Proven: {} => {}", p, q);
        }
        Some(z3::SatResult::Unknown) => {
            println!("Proving stopped!");
            return false;
        }
        _ => {
            panic!("Unknown result!")
        }
    }

    true
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Check<'a> {
    name: String,
    object: Expr,
    check: z3::ast::Bool<'a>,
}

#[derive(Clone, Debug)]
struct ProveBlock {
    precondition: Bool,
    code: Vec<Command>,
    postcondition: Bool,
    precondition_original: Bool,
    postcondition_original: Bool,
}

fn define_return_value(output: Type, return_value: Value) -> Command {
    match output {
        Type::Array(_, _) => Command::Binding(Binding::Assignment(
            Variable::Named(String::from("return_value")),
            output,
            return_value,
            false,
        )),
        Type::ArraySlice(_) => Command::Binding(Binding::Assignment(
            Variable::Named(String::from("return_value")),
            output,
            return_value,
            false,
        )),
        Type::Bool => Command::Binding(Binding::Assignment(
            Variable::Named(String::from("return_value")),
            output,
            return_value,
            false,
        )),
        Type::I32 => Command::Binding(Binding::Assignment(
            Variable::Named(String::from("return_value")),
            output,
            return_value,
            false,
        )),
        Type::Reference(_x) => {
            unimplemented!()
        }
        Type::ReferenceMutable(_x) => {
            unimplemented!()
        }
        Type::Tuple(_) => {
            //Command::Binding(Binding::Assignment(
            //Variable::Named(String::from("return_value")),
            //output,
            //return_value,
            //false,
            //))
            Command::Noop
        }
        Type::Unit => Command::Noop,
    }
}

/// Return function with some wrapping for e.g. return_value, so it's ready to be taken by ProveBlock
fn wrap_function(f: Function) -> Function {
    let mut to_prove = f.clone();

    let mut temp = f.content;

    // Set the noop as first command, so the further generation works fine, even if assertion is first in the code
    temp.insert(0, Command::Noop);

    // Assign the value being returned to the return_value variable
    temp.push(define_return_value(f.output, f.return_value));

    to_prove.content = temp.clone();

    log::debug!("START TO PROVE COMMAND LIST:");
    for i in temp {
        log::debug!("{:?}", i);
    }
    log::debug!("END TO PROVE COMMAND LIST:");

    to_prove
}

impl ProveBlock {
    /// Runs all the commands needed to just check whether the prove is successfull for this block (create, calculate, prove)
    fn simple_check(self) -> bool {
        let triples = self.create_triples();
        log::trace!("triples: {:?}", triples.clone());

        for t in triples {
            let mut final_res = true;
            let (to_check, ok) = t.calculate();
            if !ok {
                final_res = false;
            } else {
                if !to_check.prove() {
                    final_res = false;
                }
            }
            if !final_res {
                return false;
            }
        }
        true
    }

    /// Wrap asserts in {p}commands{q} triples, the {q} values are not yet expanded properly
    /// Returns multiple proveblocks from a single one
    fn create_triples(self) -> Vec<ProveBlock> {
        let ProveBlock {
            precondition,
            code: mut commands,
            postcondition,
            ..
        } = self;

        // Do this ugly asserting thing in single place (here)
        commands.push(Command::Noop);
        commands.push(Command::ProveControl(ProveControl::Assert(
            postcondition.clone(),
        )));

        log::trace!("OUR CODE - START");
        for i in commands.clone() {
            log::trace!("{}", i);
        }
        log::trace!("OUR CODE - END");

        let mut triples = Vec::new();

        let mut code_till_now = Vec::new();

        for command in commands {
            log::debug!("{:?}", command);
            match command.clone() {
                Command::ProveControl(x) => {
                    let a = match x {
                        ProveControl::Assert(z) => z,
                    };

                    code_till_now.push(command.clone());
                    triples.push(prove_block(
                        precondition.clone(),
                        code_till_now.clone(),
                        a.clone(),
                    ));
                }
                z => {
                    code_till_now.push(z);
                }
            }
            log::debug!("{:?}", triples);
        }

        log::trace!("{:?}", triples);

        triples
    }

    /// Actually compute the "real" postcondition that should be proven, based on the code
    /// The return type is (<p, comms, q>, prove_fail)
    /// as some proving can already happen at this stage (loops)
    fn calculate(self) -> (ProveBlock, bool) {
        let ProveBlock {
            precondition: p,
            code: mut _comms,
            postcondition: mut q,
            precondition_original: p_orig,
            postcondition_original: q_orig,
        } = self.clone();

        let comms_real = _comms.clone();

        let mut comms = Vec::new();
        // Invert array for proving - backwards
        let mut check = true;
        while check {
            let t = _comms.pop();
            match t {
                Some(x) => {
                    comms.push(x);
                }
                None => {
                    check = false;
                }
            }
        }
        // Invert vector here - backwards, for proper postcondition expansion
        log::trace!("{}", q);
        log::trace!("NEW TRIPLE");
        for comm in comms.clone() {
            match comm.clone() {
                Command::ProveControl(_) => {}
                _ => {
                    log::trace!("{}", comm.clone());
                    log::trace!("PRE BEFORE: {}", q.clone());
                    let (_q, t) = comm.get_pre(q.clone(), p.clone());
                    if !t {
                        return (self, false);
                    }

                    // new q is previous p
                    q = _q;
                    log::trace!("PRE AFTER: {}", q.clone());
                    log::trace!("{}", q);
                }
            }
        }

        (
            ProveBlock {
                precondition: p,
                code: comms_real,
                postcondition: q,
                precondition_original: p_orig,
                postcondition_original: q_orig,
            },
            true,
        )
    }

    /// Actually prove the triple (pre and post conditions should be calculated at this point)
    fn prove(self) -> bool {
        let ProveBlock {
            precondition: p,
            code: commands,
            postcondition: q,
            precondition_original: p_orig,
            postcondition_original: q_orig,
        } = self;

        log::debug!("START TO PROVE FINAL LIST:");
        log::trace!("{} => {:?} => {}", p.clone(), commands, q.clone());
        log::debug!("{} => {}", p, q);
        log::debug!("END TO PROVE FINAL LIST:");

        // Isn't this kind of what we are proving separately for each case? As there's no need to
        // check that p -> q for every single command, as we control the q generation. Only assertions matter.
        //to_prove_vec_final.push((to_prove.precondition, Command::Noop, p_n));

        log::debug!("{} => [[{:?}]] => {}", p.clone(), commands, q.clone());
        log::trace!("{} => [[{:?}]] => {}", p.clone(), commands, q.clone());
        log::info!("{} => {}", p.clone(), q.clone());

        let mut cfg = z3::Config::new();
        cfg.set_model_generation(true);

        let ctx = z3::Context::new(&cfg);
        let t = z3::Solver::new(&ctx);

        let (mut checks, _pre) = p.clone().as_bool(&ctx);
        let (checks_post, _post) = q.clone().as_bool(&ctx);
        checks.extend(checks_post);

        // Check overflows and underflows
        for i in checks {
            t.reset();
            t.assert(&_pre.implies(&i.check).not());
            let f = t.check();
            if f != z3::SatResult::Unsat {
                log::warn!("Bounds check warning: possible {} {}", i.name, i.object);
            }
        }
        t.reset();

        t.assert(&_pre.implies(&_post).not());

        let f = t.check();
        log::debug!("{:?}", f);
        log::debug!("{:?}", t.get_model());
        let result = Some(f);

        match result {
            Some(z3::SatResult::Sat) => {
                log::debug!("Model: {:?}", t.get_model());
                let mut temp = String::from("");
                for i in commands {
                    temp += &format!("{}\n", i).to_owned();
                }
                println!("Failed to prove: {} => {}", p, q);
                log::info!(
                    "Failed to prove: {} => {} with code:\n{}",
                    p_orig,
                    q_orig,
                    temp
                );
                log::debug!("Failed to prove: {} => {}", p, q);
                return false;
            }
            Some(z3::SatResult::Unsat) => {
                log::debug!("Proven: {:?}", commands);
            }
            Some(z3::SatResult::Unknown) => {
                println!("Proving stopped!");
                return false;
            }
            _ => {
                panic!("Unknown result!")
            }
        }

        true
    }
}

/// Prove the program provided as an input.
/// The funcs_to_prove vec may specify names of the functions to be proved, if empty all the functions are proved by default
pub fn prove(input: Program, funcs_to_prove: Vec<String>) -> bool {
    for func in input.content.clone() {
        let f_name = func.name.clone();
        if !funcs_to_prove.contains(&f_name) && !funcs_to_prove.is_empty() {
            log::warn!("Skipping function: {}", f_name);
            continue;
        } else {
            log::warn!("Proving function: {}", f_name);
        }

        let wrapped_func = wrap_function(func);

        let to_prove = prove_block(
            wrapped_func.precondition,
            wrapped_func.content,
            wrapped_func.postcondition,
        );

        let triples = to_prove.create_triples();

        for i in triples {
            let final_res;
            let (current, temp_res) = i.calculate();

            if temp_res {
                final_res = current.prove();
            } else {
                final_res = temp_res;
            }

            if !final_res {
                log::warn!("Failed to prove function: {}", f_name);
                return false;
            }
        }
        log::warn!("Successfully proved function: {}", f_name);
    }

    true
}

trait Provable {
    /// Find the P for {P} S {Q} to prove
    /// It gets calculated starting from the {Q} and then moving backwards (due to the assignment axiom)
    fn get_pre(self, q: Bool, p: Bool) -> (Bool, bool);
}

impl Provable for Command {
    fn get_pre(self, q: Bool, p: Bool) -> (Bool, bool) {
        match self {
            Command::Binding(x) => x.get_pre(q, p),
            Command::Assignment(x) => x.get_pre(q, p),
            Command::ProveControl(x) => x.get_pre(q, p),
            Command::Block(x) => x.get_pre(q, p),
            Command::Noop => (q, true),
        }
    }
}

impl Provable for Binding {
    fn get_pre(self, q: Bool, _p: Bool) -> (Bool, bool) {
        match self {
            Binding::Declaration(_, _, _) => (q, true),

            Binding::Assignment(var, _, val, _) => match val.clone() {
                Value::Tuple(_) => {
                    panic! {"This is not supported, something went wrong!"}
                }
                _ => Assignment::Single(var, val).get_pre(q, _p),
            },
            Binding::Tuple(vec) => {
                let mut real_vec = Vec::new();
                for i in vec {
                    match i {
                        Command::Assignment(a) => {
                            real_vec.push(a);
                        }
                        _ => {
                            panic!("This is not supported, something went wrong!")
                        }
                    }
                }

                Assignment::Tuple(real_vec).get_pre(q, _p)
            }
        }
    }
}

impl Provable for Assignment {
    fn get_pre(self, q: Bool, p: Bool) -> (Bool, bool) {
        match self {
            Assignment::Tuple(vec) => {
                let mut t = q;
                for i in vec {
                    let (_t, l) = i.get_pre(t, p.clone());
                    if !l {
                        return (Bool::True, false);
                    }
                    t = _t;
                }
                (t, true)
            }

            Assignment::Single(var, val) => {
                // Swap all `var` occurences with the `val` in the condition
                // Supports only simple int, bool and arrayelem assignments for now
                match var.clone() {
                    Variable::Named(name) => match val.clone() {
                        Value::Array(vals) => {
                            // Bunch of individual assignments
                            let mut i = 0;
                            let mut temp = q;
                            for v in vals {
                                let tvar = Variable::ArrayElem(
                                    name.clone(),
                                    Box::new(Value::Expr(Expr::Number(i))),
                                );
                                let ass = Assignment::Single(tvar, v);
                                let (_temp, _) = ass.get_pre(temp, p.clone());
                                temp = _temp;
                                i += 1;
                            }
                            (temp, true)
                        }
                        _ => (q.swap(var, val), true),
                    },
                    Variable::TupleElem(_, _) => unimplemented!(),
                    Variable::Empty => (q, true),
                    Variable::ArrayElem(arr_name, index) => {
                        // This is conditional
                        // a[X] = Y
                        // for all a[Z] do:
                        //     if Z == X:
                        //         a[Z] = Y
                        //     else:
                        //         a[Z] = a[Z]

                        (q.index_swap(arr_name, *index, val.clone()), true)
                    }
                }
            }
        }
    }
}

impl Provable for ProveControl {
    fn get_pre(self, _q: Bool, _p: Bool) -> (Bool, bool) {
        match self {
            ProveControl::Assert(a) => (a, true),
        }
    }
}

impl Provable for Block {
    fn get_pre(self, q: Bool, _p: Bool) -> (Bool, bool) {
        match self {
            Block::If(mut ifs, mut comms, el) => {
                // Calculate p for all the possible choices
                // Then it's (p1 && cond1) || (!p1 && p2 && cond2) || (!p1 && !p2 && p3 && cond3) || ... || (!p1 && !p2 && !p3 && .. && !p3 && p_el)

                // Handle the else case
                comms.push(el);
                ifs.push(Bool::True);

                // The code generated conditions
                let mut ps = Vec::new();
                for mut c in comms {
                    // This is pretty stupid, but hey...
                    c.push(Command::Noop);
                    c.push(Command::ProveControl(ProveControl::Assert(q.clone())));

                    log::trace!("q BEFORE: {}", q.clone());
                    log::trace!("c: {:?}", c.clone());
                    // should it have some local context?
                    let (temp, ok) = prove_block(q.clone(), c, q.clone()).calculate();

                    if !ok {
                        return (Bool::True, false);
                    }

                    ps.push(temp.postcondition);
                }

                // The ifs generated conditions
                let mut ifs_combined = Vec::new();
                for i in ifs {
                    let mut temp = i;
                    for l in ifs_combined.clone() {
                        temp = Bool::And(Box::new(temp), Box::new(Bool::Not(Box::new(l))));
                    }
                    ifs_combined.push(temp);
                }

                // Sanity check
                assert!(ifs_combined.len() == ps.len());

                // Merge the two
                let mut merged = Vec::new();
                while !ifs_combined.is_empty() {
                    let t = ps.pop().unwrap();
                    let i = ifs_combined.pop().unwrap();

                    merged.push((i, t));
                }

                // Create final bool
                let (i, p) = merged.pop().unwrap();
                let mut to_return = Bool::And(Box::new(i), Box::new(p));
                for (i, p) in merged {
                    log::debug!("{}", i);
                    log::debug!("{}", p);
                    to_return = Bool::Or(
                        Box::new(to_return),
                        Box::new(Bool::And(Box::new(i), Box::new(p))),
                    );
                }

                (to_return, true)
            }
            Block::While(cond, comms, inv, var) => {
                let strong_inv = inv.clone();

                if var == Expr::Number(0) {
                    log::warn!("No loop variant provided for loop with invariant: {}", inv);
                } else {
                    // Prove the loop variant
                    // {cond && inv && t == z} code {t < z}
                    let old_var = Expr::Value(Box::new(Value::Variable(Variable::Named(
                        String::from("__variant_old"),
                    ))));

                    let var_pre = Bool::And(
                        Box::new(cond.clone()),
                        Box::new(Bool::And(
                            Box::new(strong_inv.clone()),
                            Box::new(Bool::Equal(var.clone(), old_var.clone())),
                        )),
                    );
                    let var_post = Bool::LowerThan(var.clone(), old_var);

                    let var_prove = prove_block(var_pre, comms.clone(), var_post);

                    if !var_prove.simple_check() {
                        return (Bool::True, false);
                    }

                    // Now let's prove that invariant implies that variant >= 0
                    let var_post = Bool::GreaterEqual(var.clone(), Expr::Number(0));
                    if !prove_impl(strong_inv.clone(), var_post) {
                        return (Bool::True, false);
                    }
                }

                let pre = Bool::And(Box::new(strong_inv.clone()), Box::new(cond.clone()));

                // First check that the invariant works
                // inv && cond -> inv
                let inv_prove = prove_block(pre, comms.clone(), strong_inv.clone());

                if !inv_prove.simple_check() {
                    return (Bool::True, false);
                }

                // Also check the real thing, so if we're out of the loop then it means that post is achieved
                // not condition && invariant => post
                let pre_not = Bool::And(
                    Box::new(Bool::Not(Box::new(cond.clone()))),
                    Box::new(strong_inv.clone()),
                );

                //let real_prove = prove_block(pre_not, comms.clone(), q.clone());

                //if !real_prove.simple_check() {
                //    return (Bool::True, false);
                //}

                if !prove_impl(pre_not, q.clone()) {
                    return (Bool::True, false);
                }

                // At this point, just return the real computed {p}
                (strong_inv, true)
            }
            Block::ForRange(_iter, _first, _last, _comms, _inv) => {
                unimplemented!()
            }
        }
    }
}
