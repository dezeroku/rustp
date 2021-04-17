use crate::ast::*;
use log;
use z3;

#[cfg(test)]
mod tests;

fn define_return_value(output: Type, return_value: Value) -> Command {
    match output {
        Type::Array(_, _) => Command::Binding(Binding::Assignment(
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
        Type::Tuple(_) => Command::Binding(Binding::Assignment(
            Variable::Named(String::from("return_value")),
            output,
            return_value,
            false,
        )),
        Type::Unit => Command::Noop,
    }
}

/// Return function with pre/postconditions properly wrapped, so it's ready to be proved
fn wrap_function(f: Function) -> Function {
    let mut to_prove = f.clone();

    let mut temp = f.content;

    // Set the noop as first, so the further generation works fine, even if assertion is first in the code
    temp.insert(0, Command::Noop);

    // Put the noop at the end so loops are in bounds, should be cleaned up in the end, similar to the push of noop above
    temp.push(Command::Noop);

    // Assign the value being returned to the return_value variable
    // TODO: maybe consider renaming it to ret'val or something like that?
    temp.push(define_return_value(f.output, f.return_value));

    // Set postcondition as last assert
    temp.push(Command::ProveControl(ProveControl::Assert(
        f.postcondition.clone(),
    )));

    to_prove.content = temp.clone();

    log::debug!("START TO PROVE COMMAND LIST:");
    for i in temp {
        log::debug!("{:?}", i);
    }
    log::debug!("END TO PROVE COMMAND LIST:");

    to_prove
}

/// Wrap asserts in {p}commands{q} triples, the {q} values are not yet expanded properly
fn create_triples(commands: Vec<Command>, precondition: Bool) -> Vec<(Bool, Vec<Command>, Bool)> {
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
                triples.push((precondition.clone(), code_till_now.clone(), a));
            }
            z => {
                code_till_now.push(z);
            }
        }
        log::debug!("{:?}", triples);
    }

    triples
}

/// Actually compute the "real" postcondition that should be proven, based on the code
/// The return type is (<p, comms, q>, prove_fail)
/// as some proving can already happen at this stage (loops)
fn calculate_triples(
    triples: Vec<(Bool, Vec<Command>, Bool)>,
) -> (Vec<(Bool, Vec<Command>, Bool)>, bool) {
    let mut triples_calculated = Vec::new();

    // TODO: Consider also listing the original post, to make user output easier
    for (p, mut _comms, mut q) in triples {
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
        for comm in comms.clone() {
            match comm.clone() {
                Command::ProveControl(_) => {}
                _ => {
                    log::trace!("{}", comm.clone());
                    let (_q, t) = comm.get_pre(q.clone(), p.clone());
                    if !t {
                        return (Vec::new(), false);
                    }
                    q = _q;
                    log::trace!("{}", q);
                }
            }
        }

        triples_calculated.push((p, comms, q));
    }

    // Invert array for proving - forwards
    let mut to_return = Vec::new();
    let mut check = true;
    while check {
        let t = triples_calculated.pop();
        match t {
            Some(x) => {
                to_return.push(x);
            }
            None => {
                check = false;
            }
        }
    }

    (to_return, true)
}

/// Prove the triples
fn prove_triples(to_prove: Vec<(Bool, Vec<Command>, Bool)>) -> bool {
    log::debug!("START TO PROVE FINAL LIST:");
    for (p, _, q) in to_prove.clone() {
        log::debug!("{} => {}", p, q);
    }
    log::debug!("END TO PROVE FINAL LIST:");

    // TODO: is this needed?
    // Isn't this kind of what we are proving separately for each case? As there's no need to
    // check that p -> q for every single command, as we control the q generation. Only assertions matter.
    //to_prove_vec_final.push((to_prove.precondition, Command::Noop, p_n));

    for (p, command, q) in to_prove {
        log::debug!("{} => [[{:?}]] => {}", p.clone(), command, q.clone());
        log::info!("{} => {}", p.clone(), q.clone());

        let mut cfg = z3::Config::new();
        cfg.set_model_generation(true);

        let ctx = z3::Context::new(&cfg);
        let t = z3::Solver::new(&ctx);

        t.assert(
            &p.clone()
                .as_bool(&ctx)
                .implies(&q.clone().as_bool(&ctx))
                .not(),
        );

        let f = t.check();
        log::debug!("{:?}", f);
        log::debug!("{:?}", t.get_model());
        let result = Some(f);

        match result {
            Some(z3::SatResult::Sat) => {
                log::debug!("Model: {:?}", t.get_model());
                println!("Failed to prove: {} => {}", p, q);
                return false;
            }
            Some(z3::SatResult::Unsat) => {
                log::debug!("Proven: {:?}", command);
            }
            _ => {
                panic!("Unknown result!")
            }
        }
    }

    true
}

/// Prove the program provided as an input.
/// The funcs_to_prove vec may specify names of the functions to be proved, if empty all the functions are proved by default
pub fn prove(input: Program, funcs_to_prove: Vec<String>) -> bool {
    // Create context for each command and try to prove it individually?
    // All that we have to prove are assertions, all the rest just modifies context.

    // For now just display everything here when it happens.
    for func in input.content.clone() {
        let f_name = func.name.clone();
        if !funcs_to_prove.contains(&f_name) && !funcs_to_prove.is_empty() {
            log::debug!("Skipping function: {}", f_name);
            continue;
        } else {
            log::info!("Proving function: {}", f_name);
        }

        let wrapped_func = wrap_function(func);

        let triples = create_triples(wrapped_func.content, wrapped_func.precondition);

        let (calculated_triples, t) = calculate_triples(triples);
        if !t {
            log::info!("Failed to prove function: {}", f_name);
            return false;
        }

        let t = prove_triples(calculated_triples);

        if t {
            log::info!("Successfully proved function: {}", f_name);
        } else {
            log::info!("Failed to prove function: {}", f_name);
            return false;
        }
    }

    true
}

trait Provable {
    /// Find the P for {P} S {Q} to prove
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

            Binding::Assignment(var, _, val, _) => {
                // TODO: support array and tuple type correctly here
                // Seems that this should be actually handled based on the value type?

                // Swap all `var` occurences with the `val` in the condition
                match val.clone() {
                    Value::Bool(_) => (q.swap(var, val), true),
                    Value::Expr(_) => (q.swap(var, val), true),
                    Value::Array(vals) => {
                        let name = match var {
                            Variable::Named(x) => x,
                            _ => panic!("Incorrect assignment of array to different value than Variable::Named!")
                        };

                        // TODO: This approach does not handle variable based indexes at all.
                        // Reconsider it
                        let mut t = q;
                        let mut counter = 0;
                        for i in vals {
                            t = t.swap(
                                Variable::ArrayElem(
                                    name.clone(),
                                    Box::new(Value::Expr(Expr::Number(counter))),
                                ),
                                i,
                            );
                            counter += 1;
                        }

                        (t, true)
                    }
                    Value::Dereference(_) => unimplemented!(),
                    Value::Reference(_) => unimplemented!(),
                    Value::ReferenceMutable(_) => unimplemented!(),
                    Value::Tuple(_vals) => {
                        unimplemented!()
                    }
                    Value::Unit => (q, true),
                    Value::Variable(_) => {
                        // TODO: this may be not that easy actually?
                        // check it just to be sure
                        (q.swap(var, val), true)
                    }
                    Value::FunctionCall(_name, _args) => unimplemented!(),
                }
            }
            Binding::Tuple(_vec) => {
                unimplemented!()
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
            Assignment::Single(var, val) => (q.swap(var, val), true),
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

                // TODO: check the below implementation for any problems, it should be treated as a PoC for now

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
                    // TODO: Probably real precondition should be used instead of q here
                    // should it have some local context?
                    let triples = create_triples(c, q.clone());
                    log::trace!("triples: {:?}", triples.clone());
                    let (mut calculated_triples, t) = calculate_triples(triples);
                    if !t {
                        return (Bool::True, false);
                    }

                    log::trace!("calculated_triples: {:?}", calculated_triples.clone());

                    let (_, _, f) = calculated_triples.pop().unwrap();
                    let mut temp = f;
                    for (_, _, t) in calculated_triples {
                        temp = Bool::And(Box::new(temp), Box::new(t));
                        log::debug!("q AFTER: {}", temp);
                    }
                    ps.push(temp);
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
            Block::While(cond, mut comms, inv) => {
                // TODO: this is strong invariant, we don't look for it yet, but should be added
                let strong_inv = inv.clone();
                //Bool::And(Box::new(inv.clone()), Box::new(q.clone()));

                let pre = Bool::And(Box::new(strong_inv.clone()), Box::new(cond.clone()));

                // First check that the invariant works, so inv && cond -> inv
                // This is pretty stupid, but hey...
                comms.push(Command::Noop);
                comms.push(Command::ProveControl(ProveControl::Assert(inv.clone())));

                let triples = create_triples(comms.clone(), pre);
                log::trace!("triples: {:?}", triples.clone());
                let (calculated_triples, t) = calculate_triples(triples.clone());
                if !t {
                    return (Bool::True, false);
                }

                log::trace!("calculated_triples: {:?}", calculated_triples.clone());

                // Also check that not condition && invariant => post
                let pre_not = Bool::And(
                    Box::new(strong_inv.clone()),
                    Box::new(Bool::Not(Box::new(cond.clone()))),
                );

                // Remove last assert and change it with the proper post
                // These implications should be tested here it seems
                // TODO: is this ok?
                comms.pop();
                comms.push(Command::ProveControl(ProveControl::Assert(q.clone())));

                let triples_not = create_triples(comms.clone(), pre_not);
                log::trace!("triples_not: {:?}", triples_not.clone());
                let (calculated_triples_not, t) = calculate_triples(triples_not);
                if !t {
                    return (Bool::True, false);
                }
                log::trace!(
                    "calculated_triples_not: {:?}",
                    calculated_triples_not.clone()
                );

                if !prove_triples(calculated_triples) {
                    return (Bool::True, false);
                }

                if !prove_triples(calculated_triples_not) {
                    return (Bool::True, false);
                }

                // At this point, just return the real computed {p}
                //let final_list = calculated_triples;
                //final_list.extend(calculated_triples_not);

                // TODO: this is probably not going to be inv
                (inv, true)
            }
            Block::ForRange(_iter, _first, _last, _comms, _inv) => {
                unimplemented!()
            }
        }
    }
}

trait ProvableValue {
    fn as_bool<'a>(self, ctx: &'a z3::Context) -> z3::ast::Bool<'a>;
    fn as_int<'a>(self, ctx: &'a z3::Context) -> z3::ast::Int<'a>;
}

impl ProvableValue for Value {
    fn as_bool<'a>(self, ctx: &'a z3::Context) -> z3::ast::Bool<'a> {
        match self {
            Value::Expr(e) => panic!("Bool value used as an int: {}", e),
            Value::Bool(b) => b.as_bool(ctx),
            Value::Variable(x) => match x {
                Variable::Named(name) => z3::ast::Bool::new_const(ctx, name),
                Variable::Empty => panic!("Empty variable tried to be used as a bool!"),
                Variable::ArrayElem(arr_name, ind) => {
                    let t = z3::ast::Array::new_const(
                        ctx,
                        arr_name,
                        &z3::Sort::int(ctx),
                        &z3::Sort::int(ctx),
                    );

                    t.select(&ind.as_int(ctx)).as_bool().unwrap()
                }
                Variable::TupleElem(_name, _ind) => unimplemented!(),
            },
            Value::Tuple(t) => panic!("Tuple {:?} tried to be used as a bool!", t),
            Value::Array(a) => panic!("Array {:?} tried to be used as a bool!", a),
            Value::FunctionCall(_name, _args) => unimplemented!(),
            Value::Reference(_v) => unimplemented!(),
            Value::ReferenceMutable(_v) => unimplemented!(),
            Value::Dereference(_v) => unimplemented!(),
            Value::Unit => unimplemented!(),
        }
    }

    fn as_int<'a>(self, ctx: &'a z3::Context) -> z3::ast::Int<'a> {
        match self {
            Value::Expr(e) => e.as_int(ctx),
            Value::Bool(b) => panic!("Bool value ({}) used as an int", b),
            Value::Variable(x) => match x {
                Variable::Named(name) => z3::ast::Int::new_const(ctx, name),
                Variable::Empty => panic!("Empty variable tried to be used as an int!"),
                Variable::ArrayElem(arr_name, ind) => {
                    let t = z3::ast::Array::new_const(
                        ctx,
                        arr_name,
                        &z3::Sort::int(ctx),
                        &z3::Sort::int(ctx),
                    );

                    t.select(&ind.as_int(ctx)).as_int().unwrap()
                }

                Variable::TupleElem(_name, _ind) => unimplemented!(),
            },
            Value::Tuple(t) => panic!("Tuple {:?} tried to be used as an intl!", t),
            Value::Array(a) => panic!("Array {:?} tried to be used as an int!", a),
            Value::FunctionCall(_name, _args) => unimplemented!(),
            Value::Reference(_v) => unimplemented!(),
            Value::ReferenceMutable(_v) => unimplemented!(),
            Value::Dereference(_v) => unimplemented!(),
            Value::Unit => unimplemented!(),
        }
    }
}

trait ProvableBool {
    fn as_bool<'a>(self, ctx: &'a z3::Context) -> z3::ast::Bool<'a>;
}

trait ProvableInt {
    fn as_int<'a>(self, ctx: &'a z3::Context) -> z3::ast::Int<'a>;
}

impl ProvableInt for Expr {
    fn as_int<'a>(self, ctx: &'a z3::Context) -> z3::ast::Int<'a> {
        match self {
            Expr::Number(a) => z3::ast::Int::from_i64(ctx, a.into()),
            Expr::Op(a, op, b) => match op {
                Opcode::Add => z3::ast::Int::add(ctx, &[&a.as_int(ctx), &b.as_int(ctx)]),
                Opcode::Sub => z3::ast::Int::sub(ctx, &[&a.as_int(ctx), &b.as_int(ctx)]),
                Opcode::Mul => z3::ast::Int::mul(ctx, &[&a.as_int(ctx), &b.as_int(ctx)]),
                Opcode::Div => a.as_int(ctx).div(&b.as_int(ctx)),
                Opcode::Rem => a.as_int(ctx).rem(&b.as_int(ctx)),
            },
            Expr::Value(a) => a.as_int(ctx),
        }
    }
}

impl ProvableBool for Bool {
    fn as_bool<'a>(self, ctx: &'a z3::Context) -> z3::ast::Bool<'a> {
        match self {
            Bool::True => (z3::ast::Bool::from_bool(&ctx, true)),
            Bool::False => (z3::ast::Bool::from_bool(&ctx, false)),
            Bool::And(_a, _b) => {
                let a: Bool = *_a;
                let b: Bool = *_b;
                z3::ast::Bool::and(ctx, &[&a.as_bool(ctx), &b.as_bool(ctx)])
            }
            Bool::Or(_a, _b) => {
                let a: Bool = *_a;
                let b: Bool = *_b;
                z3::ast::Bool::or(ctx, &[&a.as_bool(ctx), &b.as_bool(ctx)])
            }
            Bool::Not(a) => z3::ast::Bool::not(&a.as_bool(ctx)),
            Bool::Equal(a, b) => z3::ast::Bool::and(
                ctx,
                &[
                    &a.clone().as_int(ctx).ge(&b.clone().as_int(ctx)),
                    &a.as_int(ctx).le(&b.as_int(ctx)),
                ],
            ),
            Bool::GreaterEqual(a, b) => a.as_int(ctx).ge(&b.as_int(ctx)),
            Bool::LowerEqual(a, b) => a.as_int(ctx).le(&b.as_int(ctx)),
            Bool::GreaterThan(a, b) => a.as_int(ctx).gt(&b.as_int(ctx)),
            Bool::LowerThan(a, b) => a.as_int(ctx).lt(&b.as_int(ctx)),
            Bool::Value(v) => v.as_bool(ctx),
        }
    }
}

trait ProvableCommand {
    fn as_bool<'a>(self, ctx: &'a z3::Context) -> (bool, z3::ast::Bool<'a>);
}

impl ProvableCommand for Command {
    // Concept: Try to prove not (negate everything) to try to find if there exists an incorrect mapping?
    fn as_bool<'a>(self, ctx: &'a z3::Context) -> (bool, z3::ast::Bool<'a>) {
        match self {
            Command::ProveControl(a) => match a {
                // We are trying to find COUNTER example here.
                // So if we get sat, then it means that the assertion is actually incorrect
                ProveControl::Assert(b) => {
                    let x = b.as_bool(ctx).not();
                    log::debug!("ASSERT: {}", x);
                    (true, x)
                }
            },
            _ => {
                // Nothing to prove here
                (false, z3::ast::Bool::from_bool(&ctx, true))
            }
        }
    }
}
