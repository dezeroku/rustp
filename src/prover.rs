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
        //temp.insert(
        //    0,
        //    Command::ProveControl(ProveControl::Assume(func.precondition)),
        //);

        // Set the noop as first, so the generation below works fine, even if assertion is first in the code
        temp.insert(0, Command::Noop);

        // TODO: uncomment when tuple type is handled in add_variable
        //if func.output.clone() != Type::Unit {
        //    temp.push(Command::Binding(Binding::Assignment(
        //        Variable::Named(String::from("return_value")),
        //        func.output.clone(),
        //        func.return_value.clone(),
        //        false,
        //    )));
        //}

        // Put the noop at the end so loops are in bounds, should be cleaned up in the end, similar to the push of noop above
        temp.push(Command::Noop);

        // Set postcondition as last assert
        temp.push(Command::ProveControl(ProveControl::Assert(
            func.postcondition.clone(),
        )));

        to_prove.content = temp.clone();
        log::debug!("START TO PROVE COMMAND LIST:");
        for i in temp.clone() {
            log::debug!("{:?}", i);
        }
        log::debug!("END TO PROVE COMMAND LIST:");

        let mut to_prove_vec = temp;

        // Invert the array for generation
        //let mut check = true;
        //while check {
        //    let t = temp.pop();
        //    match t {
        //        Some(x) => {
        //            to_prove_vec.push(x);
        //        }
        //        None => {
        //            check = false;
        //        }
        //    }
        //}
        // TODO:
        // The whole function is {P} S {Q} (precondtion, code, postcondition)

        // for command in list of commands backwards:
        // get the P and Q for the command
        // check if this implies?
        // do the same for next command, but use P as Q

        // first generate the list of stuff to prove
        // then in another loop, run the actual proving

        let mut to_prove_vec_temp = Vec::new();

        // Create a list of small contexts, that would be {precondition} bunch of code {assertion}

        let precondition = to_prove.precondition;
        let mut code_till_now = Vec::new();
        //let mut q = Bool::True;
        // Just an initializer
        //let mut p = Bool::True;

        for command in to_prove_vec {
            log::debug!("{:?}", command);
            match command.clone() {
                Command::ProveControl(x) => {
                    let a = match x {
                        ProveControl::Assert(z) => z,
                        ProveControl::Assume(z) => z,
                        ProveControl::LoopInvariant(z) => z,
                    };

                    // Is this really needed or does it make the context look worse?
                    code_till_now.push(command.clone());
                    to_prove_vec_temp.push((precondition.clone(), code_till_now.clone(), a));
                    //let (mut p, mut command, mut q) = to_prove_vec_temp.pop().unwrap();
                    //q = Bool::And(Box::new(q), Box::new(a));
                    //to_prove_vec_temp.push((p, command, q));
                }
                z => {
                    // Check the assertions, if something has to be added to current "real command" q

                    code_till_now.push(z);
                    // Get p based on the command
                    //p = z.get_pre(q.clone());

                    //to_prove_vec_temp.push((p.clone(), command, q.clone()));
                    //q = p.clone();
                }
            }
            log::debug!("{:?}", to_prove_vec_temp);
        }
        // Remember the p of first command to check if precondition implies it later on.
        //let p_n = p;

        // Work through to_prove_vec_temp and calculate the final posts based on the code

        let mut to_prove_vec_temp_calculated = Vec::new();

        // TODO: Consider also listing the original post, to make user output easier
        for (p, mut _comms, mut q) in to_prove_vec_temp {
            let mut comms = Vec::new();
            // Invert array for proving
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

            // Invert vector here, for proper postcondition expansion

            log::trace!("{}", q);
            for comm in comms.clone() {
                match comm.clone() {
                    Command::ProveControl(_) => {}
                    _ => {
                        log::trace!("{}", comm.clone());
                        q = comm.get_pre(q.clone());
                        log::trace!("{}", q);
                    }
                }
            }

            to_prove_vec_temp_calculated.push((p, comms, q));
        }

        let mut to_prove_vec_final = Vec::new();
        // Invert array for proving
        let mut check = true;
        while check {
            let t = to_prove_vec_temp_calculated.pop();
            match t {
                Some(x) => {
                    to_prove_vec_final.push(x);
                }
                None => {
                    check = false;
                }
            }
        }

        log::debug!("START TO PROVE FINAL LIST:");
        for i in to_prove_vec_final.clone() {
            log::debug!("{:?}", i);
        }
        log::debug!("END TO PROVE FINAL LIST:");

        //to_prove_vec_final.push((to_prove.precondition, Command::Noop, p_n));

        for (p, command, q) in to_prove_vec_final {
            log::debug!("{} => [[{:?}]] => {}", p.clone(), command, q.clone());

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
                    log::info!("Model: {:?}", t.get_model());
                    return false;
                }
                Some(z3::SatResult::Unsat) => {
                    log::info!("Proven: {:?}", command);
                }
                _ => {}
            }
        }

        log::info!("Successfully proved function: {}", f_name);
    }

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
            Command::Assignment(x) => x.get_pre(q),
            Command::ProveControl(x) => x.get_pre(q),
            Command::Block(x) => unimplemented!(),
            Command::Noop => q,
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

impl Provable for Assignment {
    fn get_pre(self, q: Bool) -> Bool {
        match self {
            Assignment::Tuple(vec) => {
                let mut t = q;
                for i in vec {
                    t = i.get_pre(t);
                }
                t
            }
            Assignment::Single(var, val) => q.swap(var, val),
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
    fn prove_empty1() {
        assert!(prove(Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::True,
                postcondition: Bool::True,
                return_value: Value::Unit
            }]
        }));
    }

    #[test]
    fn prove_1() {
        assert!(prove(Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![
                    Command::Binding(Binding::Assignment(
                        Variable::Named(String::from("x")),
                        Type::I32,
                        Value::Expr(Expr::Number(1)),
                        false
                    )),
                    Command::ProveControl(ProveControl::Assert(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Number(1)
                    )))
                ],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::True,
                postcondition: Bool::True,
                return_value: Value::Unit
            }]
        }));
    }

    #[test]
    fn prove_2() {
        assert!(prove(Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![
                    Command::Binding(Binding::Assignment(
                        Variable::Named(String::from("x")),
                        Type::I32,
                        Value::Expr(Expr::Number(1)),
                        false
                    )),
                    Command::ProveControl(ProveControl::Assert(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Number(1)
                    ))),
                    Command::Binding(Binding::Assignment(
                        Variable::Named(String::from("x")),
                        Type::I32,
                        Value::Expr(Expr::Number(2)),
                        false
                    )),
                    Command::ProveControl(ProveControl::Assert(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Number(2)
                    )))
                ],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::True,
                postcondition: Bool::True,
                return_value: Value::Unit
            }]
        }));
    }

    #[test]
    fn prove_3() {
        assert!(prove(Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![
                    Command::Binding(Binding::Assignment(
                        Variable::Named(String::from("x")),
                        Type::I32,
                        Value::Expr(Expr::Number(1)),
                        false
                    )),
                    Command::ProveControl(ProveControl::Assert(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Number(1)
                    ))),
                    Command::Binding(Binding::Assignment(
                        Variable::Named(String::from("x")),
                        Type::I32,
                        Value::Expr(Expr::Number(2)),
                        false
                    )),
                    Command::ProveControl(ProveControl::Assert(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Number(2)
                    ))),
                    Command::Binding(Binding::Assignment(
                        Variable::Named(String::from("x")),
                        Type::I32,
                        Value::Expr(Expr::Op(
                            Box::new(Expr::Value(Box::new(Value::Variable(Variable::Named(
                                String::from("x")
                            ))))),
                            Opcode::Add,
                            Box::new(Expr::Number(3))
                        )),
                        false
                    )),
                    Command::ProveControl(ProveControl::Assert(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Number(5)
                    ))),
                    Command::Binding(Binding::Assignment(
                        Variable::Named(String::from("x")),
                        Type::I32,
                        Value::Expr(Expr::Number(7)),
                        false
                    )),
                    Command::ProveControl(ProveControl::Assert(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Number(7)
                    ))),
                ],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::True,
                postcondition: Bool::True,
                return_value: Value::Unit
            }]
        }));
    }

    #[test]
    fn prove_fail_1() {
        assert!(!prove(Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![
                    Command::Binding(Binding::Assignment(
                        Variable::Named(String::from("x")),
                        Type::I32,
                        Value::Expr(Expr::Number(1)),
                        false
                    )),
                    Command::ProveControl(ProveControl::Assert(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Number(2)
                    )))
                ],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::True,
                postcondition: Bool::True,
                return_value: Value::Unit
            }]
        }));
    }

    #[test]
    fn prove_postcondition1() {
        assert!(prove(Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![Command::Binding(Binding::Assignment(
                    Variable::Named(String::from("x")),
                    Type::I32,
                    Value::Expr(Expr::Number(1)),
                    false
                ))],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::True,
                postcondition: Bool::Equal(
                    Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                        "x"
                    ))))),
                    Expr::Number(1)
                ),
                return_value: Value::Unit
            }]
        }));
    }

    #[test]
    fn prove_postcondition_fail1() {
        assert!(prove(Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![Command::Binding(Binding::Assignment(
                    Variable::Named(String::from("x")),
                    Type::I32,
                    Value::Expr(Expr::Number(1)),
                    false
                ))],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::True,
                postcondition: Bool::Equal(
                    Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                        "x"
                    ))))),
                    Expr::Number(2)
                ),
                return_value: Value::Unit
            }]
        }));
    }
}
