use crate::ast::*;
use crate::context;
use log;
use std::collections::HashMap;
use z3;

pub fn prove(input: Program) -> bool {
    // Create context for each command and try to prove it individually?
    // All that we have to prove are assertions, all the rest just modifies context.

    // For now just display everything here when it happens.
    for func in input.content.clone() {
        let f_name = func.name.clone();
        log::info!("Proving function: {}", f_name);

        let mut to_prove = func.clone();

        let mut temp = func.content.clone();

        // Set precondition as first assume
        temp.insert(
            0,
            Command::ProveControl(ProveControl::Assume(func.precondition)),
        );

        // TODO: uncomment when tuple type is handled in add_variable
        //if func.output.clone() != Type::Unit {
        //    temp.push(Command::Binding(Binding::Assignment(
        //        Variable::Named(String::from("return_value")),
        //        func.output.clone(),
        //        func.return_value.clone(),
        //        false,
        //    )));
        //}

        // Set postcondition as last assert
        temp.push(Command::ProveControl(ProveControl::Assert(
            func.postcondition.clone(),
        )));

        to_prove.content = temp.clone();

        let mut to_prove_vec = Vec::new();

        let mut check = true;
        while check {
            let t = temp.pop();
            match t {
                Some(x) => {
                    to_prove_vec.push(x);
                }
                None => {
                    check = false;
                }
            }
        }
        // TODO:
        // The whole function is {P} S {Q} (precondtion, code, postcondition)

        // for command in list of commands backwards:
        // get the P and Q for the command
        // check if this implies?
        // do the same for next command, but use P as Q

        let mut q = Bool::True;
        let mut p;
        for command in to_prove_vec {
            // Get p based on the command
            p = command.get_pre(q.clone());

            let mut cfg = z3::Config::new();
            cfg.set_model_generation(true);

            let ctx = z3::Context::new(&cfg);
            let t = z3::Solver::new(&ctx);

            t.assert(&p.clone().as_bool(&ctx).implies(&q.as_bool(&ctx)));

            let f = t.check();
            log::debug!("{:?}", f);
            log::debug!("{:?}", t.get_model());
            let result = Some(f);

            match result {
                Some(z3::SatResult::Unsat) => {
                    return false;
                }
                Some(z3::SatResult::Sat) => {
                    //log::info!("Proven: {}", command);
                    log::info!("Model: {:?}", t.get_model());
                }
                _ => {}
            }

            q = p;
        }
        //let con = context::get_context_func(to_prove, input.clone());
        //// Try to prove
        ////println!("{:?}", con);

        //for frame in con {
        //    log::debug!("{:?}", frame);
        //    let sat = prove_frame(frame.clone());
        //    match sat {
        //        None => {}
        //        Some(a) => {
        //            // A counter example was found - our check failed
        //            if a != z3::SatResult::Unsat {
        //                println!("Failed to prove: {}", frame.command);
        //                return false;
        //            }
        //        }
        //    }
        //}

        log::info!("Successfully proved function: {}", f_name);
    }

    //println!("START");
    //// Idea:
    //// declare constants for all identifiers that are there (variables)
    //// use asserts to assign values
    //// assert the assertion in the end
    //// try to check it

    //let mut cfg = z3::Config::new();
    //cfg.set_model_generation(true);
    ////let te: i32 = 20;
    //let ctx = z3::Context::new(&cfg);
    ////// TODO: use goal?
    //////let t = z3::Goal::new(&ctx, true, false, false);
    //////t.assert(&z3::ast::Ast());
    //let t = z3::Solver::new(&ctx);
    ////let b = z3::ast::Bool::new_const(&ctx, "b");
    ////let c = z3::ast::Bool::new_const(&ctx, "c");
    ////let d = z3::ast::Bool::new_const(&ctx, "d");
    //let x = z3::ast::Int::new_const(&ctx, "x");
    //let y = z3::ast::Int::new_const(&ctx, "y");
    ////t.assert(&z3::ast::Bool::and(&ctx, &[&b, &c]));
    ////t.assert(&z3::ast::Bool::and(&ctx, &[&c, &d]));
    //t.assert(&x.gt(&y));
    //let x = z3::ast::Int::new_const(&ctx, "x");
    //t.assert(&x.gt(&z3::ast::Int::from_i64(&ctx, 123)));
    //t.assert(&y.gt(&z3::ast::Int::from_i64(&ctx, 180)));
    ////t.assert(&y.gt(&z3::ast::Int::from_i64(&ctx, te.into())));
    //////t.assert(&z3::ast::Bool::not(&z3::ast::Bool::and(&ctx, &[&c, &d])));
    //////t.assert(&z3::ast::Bool::from_bool(&ctx, c == d));
    //let f = t.check();
    //println!("{:?}", f);
    //println!("{:?}", t.get_model());
    //////println!("{:?}", t.get_proof());
    //println!("DONE");
    true
}

trait Provable {
    /// Find the P for {P} S {Q} to prove
    fn get_pre(self, q: Bool) -> Bool;
}

impl Provable for Command {
    fn get_pre(self, q: Bool) -> Bool {
        match self {
            Command::Binding(x) => x.get_pre(q),
            Command::Assignment(x) => unimplemented!(),
            Command::ProveControl(x) => x.get_pre(q),
            Command::Block(x) => unimplemented!(),
            Command::Noop => Bool::True,
        }
    }
}

impl Provable for Binding {
    fn get_pre(self, q: Bool) -> Bool {
        match self {
            Binding::Declaration(var, t, m) => Bool::True,

            Binding::Assignment(var, t, val, m) => {
                // Swap all `var` occurences with the `val` in the condition
                q.swap(var, val)
            }
            Binding::Tuple(vec) => {
                unimplemented!()
            }
        }
    }
}

impl Provable for ProveControl {
    fn get_pre(self, q: Bool) -> Bool {
        match self {
            ProveControl::Assert(a) => a,
            ProveControl::Assume(a) => a,
            ProveControl::LoopInvariant(a) => a,
        }
    }
}

fn prove_frame(frame: context::Frame) -> Option<z3::SatResult> {
    let mut cfg = z3::Config::new();
    cfg.set_model_generation(true);

    let ctx = z3::Context::new(&cfg);
    let t = z3::Solver::new(&ctx);
    let needed = _prove_frame(frame.clone(), &ctx, &t);
    let result;
    if needed {
        let f = t.check();
        log::debug!("{}", frame.command);
        log::debug!("{:?}", f);
        log::debug!("{:?}", t.get_model());
        result = Some(f);
        match result {
            Some(z3::SatResult::Sat) => {
                log::info!("Model: {:?}", t.get_model());
            }
            Some(z3::SatResult::Unsat) => {
                log::info!("Proven: {}", frame.command);
            }
            _ => {}
        }
    } else {
        //println!("Nothing to prove!");
        result = None;
    }
    result
}

fn _prove_frame(frame: context::Frame, ctx: &z3::Context, sol: &z3::Solver) -> bool {
    // convert to Z3 problem and run it
    // for all the variables that we have, define them and assert their value
    //let mut vars_int = HashMap::new();
    let vars = frame.vars.clone();
    for context::Var { n, t, v } in vars {
        add_variable(n, t, v, ctx, sol);
        // TODO: the add_variable should also handle setting the correct value based on input type
        // TODO: assign proper val
        // so we need to be able to convert value into z3 int/bool
        //let val = 1;
        //sol.assert(&var.ge(&z3::ast::Int::from_i64(&ctx, val)));
        //sol.assert(&var.le(&z3::ast::Int::from_i64(&ctx, val)));
        //vars_int.insert(name, var);
    }

    for assume in frame.assumes {
        log::debug!("Assume: {}", assume);
        sol.assert(&assume.as_bool(ctx));
    }

    // TODO: run the assertion to provea
    // so we need to be able to convert bool into z3 bool
    let to_prove = frame.command;
    let (needed, to_prove_z3) = to_prove.as_bool(ctx);
    sol.assert(&to_prove_z3);
    needed
}

fn add_variable(var: Variable, t: Type, v: Value, ctx: &z3::Context, sol: &z3::Solver) {
    let unknown = match v.clone() {
        Value::Unknown => true,
        _ => false,
    };
    // TODO: check if this covers all the AST entries
    log::debug!("SET: {} = {:?}", var.clone(), v.clone());
    match t {
        Type::I32 => match var.clone() {
            Variable::Named(name) => {
                if unknown {
                    z3::ast::Int::new_const(ctx, name);
                } else {
                    sol.assert(
                        &Bool::Equal(
                            Expr::Value(Box::new(Value::Variable(var))),
                            Expr::Value(Box::new(v)),
                        )
                        .as_bool(ctx),
                    );
                }
            }
            Variable::ArrayElem(arr_name, index) => {
                // TODO: define indices properly, don't allow ALL the values
                // should we even define this array here instead of in the Type:Array?
                let t = z3::ast::Array::new_const(
                    ctx,
                    arr_name.clone(),
                    &z3::Sort::int(ctx),
                    &z3::Sort::int(ctx),
                );

                if unknown {
                } else {
                    let t = sol.assert(
                        &Bool::Equal(
                            Expr::Value(Box::new(Value::Variable(Variable::ArrayElem(
                                arr_name.clone(),
                                index,
                            )))),
                            Expr::Value(Box::new(v)),
                        )
                        .as_bool(ctx),
                    );
                }
            }

            _ => unimplemented!(),
        },
        Type::Bool => match var.clone() {
            // Use smart boolean operations to assign the newly created z3 variable same value that's assigned in the code
            Variable::Named(name) => {
                if unknown {
                    z3::ast::Bool::new_const(ctx, name);
                } else {
                    sol.assert(&z3::ast::Bool::or(
                        ctx,
                        &[&z3::ast::Bool::and(
                            ctx,
                            &[&z3::ast::Bool::new_const(ctx, name), &v.as_bool(ctx)],
                        )],
                    ));
                }
            }
            _ => unimplemented!(),
        },
        Type::Array(ty, _) => match var.clone() {
            Variable::Named(arr_name) => {
                let t = z3::ast::Array::new_const(
                    ctx,
                    arr_name.clone(),
                    &z3::Sort::int(ctx),
                    &z3::Sort::int(ctx),
                );

                if unknown {
                } else {
                    // Correctly assign specific values
                    match v {
                        Value::Array(a) => {
                            let mut i = 0;
                            for va in a {
                                match *ty {
                                    Type::I32 => sol.assert(
                                        &Bool::Equal(
                                            Expr::Value(Box::new(Value::Variable(
                                                Variable::ArrayElem(
                                                    arr_name.clone(),
                                                    Box::new(Value::Expr(Expr::Number(i))),
                                                ),
                                            ))),
                                            Expr::Value(Box::new(va)),
                                        )
                                        .as_bool(ctx),
                                    ),
                                    Type::Bool => {
                                        // TODO: base on the work above, but we don't have Equals implemented for Bool at the moment
                                    }
                                    // Tuple and array types do not have to be covered, however references have to
                                    _ => unimplemented!(),
                                }
                                i += 1;
                            }
                        }
                        _ => unimplemented!(),
                    }
                }
            }
            _ => unimplemented!(),
        },

        _ => unimplemented!(),
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
                Variable::Empty => unimplemented!(),
                Variable::ArrayElem(arr_name, ind) => {
                    let t = z3::ast::Array::new_const(
                        ctx,
                        arr_name,
                        &z3::Sort::int(ctx),
                        &z3::Sort::int(ctx),
                    );

                    t.select(&ind.as_int(ctx)).as_bool().unwrap()
                }
                Variable::TupleElem(name, ind) => unimplemented!(),
            },
            Value::Tuple(t) => unimplemented!(),
            Value::Array(a) => unimplemented!(),
            Value::FunctionCall(name, args) => unimplemented!(),
            Value::Reference(v) => unimplemented!(),
            Value::ReferenceMutable(v) => unimplemented!(),
            Value::Dereference(v) => unimplemented!(),
            Value::Unit => unimplemented!(),
            Value::Unknown => unimplemented!(),
        }
    }

    fn as_int<'a>(self, ctx: &'a z3::Context) -> z3::ast::Int<'a> {
        match self {
            Value::Expr(e) => e.as_int(ctx),
            Value::Bool(b) => panic!("Bool value ({}) used as an int", b),
            Value::Variable(x) => match x {
                Variable::Named(name) => z3::ast::Int::new_const(ctx, name),
                Variable::Empty => unimplemented!(),
                Variable::ArrayElem(arr_name, ind) => {
                    let t = z3::ast::Array::new_const(
                        ctx,
                        arr_name,
                        &z3::Sort::int(ctx),
                        &z3::Sort::int(ctx),
                    );

                    t.select(&ind.as_int(ctx)).as_int().unwrap()
                }

                Variable::TupleElem(name, ind) => unimplemented!(),
            },
            Value::Tuple(t) => unimplemented!(),
            Value::Array(a) => unimplemented!(),
            Value::FunctionCall(name, args) => unimplemented!(),
            Value::Reference(v) => unimplemented!(),
            Value::ReferenceMutable(v) => unimplemented!(),
            Value::Dereference(v) => unimplemented!(),
            Value::Unit => unimplemented!(),
            Value::Unknown => {
                // TODO: what this should be set to?
                // TODO: use the for_all quantifier properly, to test for all possible inputs, instead of trying to find the one that works?
                // It's mostly used for the function input params
                //let mut rng = rand::thread_rng();
                //z3::ast::Int::new_const(ctx, rng.gen::<i32>().to_string())
                z3::ast::Int::new_const(ctx, "_")
            }
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
                ProveControl::Assume(b) => {
                    // No need to prove these, we believe them without testing
                    // Just pas the information from that further on
                    //true, b.as_bool(ctx).not())
                    (false, z3::ast::Bool::from_bool(&ctx, true))
                }
                ProveControl::LoopInvariant(b) => (true, b.as_bool(ctx).not()),
            },
            _ => {
                // Nothing to prove here
                (false, z3::ast::Bool::from_bool(&ctx, true))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn prove_frame1() {
        let com = Command::ProveControl(ProveControl::Assert(Bool::True));
        let frame = context::Frame {
            command: com,
            funcs: Vec::new(),
            vals: Vec::new(),
            vars: Vec::new(),
            assumes: Vec::new(),
        };

        assert_eq!(prove_frame(frame), Some(z3::SatResult::Unsat));
    }

    #[test]
    fn prove_frame2() {
        let com = Command::ProveControl(ProveControl::Assert(Bool::False));
        let frame = context::Frame {
            command: com,
            funcs: Vec::new(),
            vals: Vec::new(),
            vars: Vec::new(),
            assumes: Vec::new(),
        };

        assert_eq!(prove_frame(frame), Some(z3::SatResult::Sat));
    }

    #[test]
    fn prove_frame3() {
        let com = Command::ProveControl(ProveControl::Assert(Bool::And(
            Box::new(Bool::True),
            Box::new(Bool::True),
        )));
        let frame = context::Frame {
            command: com,
            funcs: Vec::new(),
            vals: Vec::new(),
            vars: Vec::new(),
            assumes: Vec::new(),
        };

        assert_eq!(prove_frame(frame), Some(z3::SatResult::Unsat));
    }

    #[test]
    fn prove_frame4() {
        let com = Command::ProveControl(ProveControl::Assert(Bool::And(
            Box::new(Bool::True),
            Box::new(Bool::False),
        )));
        let frame = context::Frame {
            command: com,
            funcs: Vec::new(),
            vals: Vec::new(),
            vars: Vec::new(),
            assumes: Vec::new(),
        };

        assert_eq!(prove_frame(frame), Some(z3::SatResult::Sat));
    }

    #[test]
    fn prove_frame5() {
        let com = Command::ProveControl(ProveControl::Assert(Bool::Or(
            Box::new(Bool::True),
            Box::new(Bool::False),
        )));
        let frame = context::Frame {
            command: com,
            funcs: Vec::new(),
            vals: Vec::new(),
            vars: Vec::new(),
            assumes: Vec::new(),
        };

        assert_eq!(prove_frame(frame), Some(z3::SatResult::Unsat));
    }
}
