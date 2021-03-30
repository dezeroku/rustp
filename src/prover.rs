use crate::ast::*;
use crate::context;
use std::collections::HashMap;
use z3;

pub fn prove(input: Program) -> bool {
    // Create context for each command and try to prove it individually?
    // All that we have to prove are assertions, all the rest just modifies context.

    // For now just display everything here when it happens.
    for func in input.content.clone() {
        let mut temp = func.content.clone();
        if func.output.clone() != Type::Unit {
            temp.push(Command::Binding(Binding::Assignment(
                Variable::Named(String::from("return_value")),
                func.output.clone(),
                func.return_value.clone(),
                false,
            )));
        }

        temp.push(Command::ProveControl(ProveControl::Assert(
            func.postcondition.clone(),
        )));

        let con = context::get_context_func(func, input.clone());
        // Try to prove
        //println!("{:?}", con);

        for frame in con {
            let sat = prove_frame(frame.clone());
            match sat {
                None => {}
                Some(a) => {
                    if a != z3::SatResult::Sat {
                        println!("Failed to prove: {}", frame.command);
                        return false;
                    }
                }
            }
        }
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

fn prove_frame(frame: context::Frame) -> Option<z3::SatResult> {
    let mut cfg = z3::Config::new();
    cfg.set_model_generation(true);

    let ctx = z3::Context::new(&cfg);
    let t = z3::Solver::new(&ctx);
    let needed = _prove_frame(frame, &ctx, &t);
    let result;
    if needed {
        let f = t.check();
        println!();
        println!("{:?}", f);
        println!();
        println!("{:?}", t.get_model());
        result = Some(f);
    } else {
        println!("Nothing to prove!");
        result = None;
    }
    println!();
    result
}

fn _prove_frame(frame: context::Frame, ctx: &z3::Context, sol: &z3::Solver) -> bool {
    // convert to Z3 problem and run it
    // for all the variables that we have, define them and assert their value
    //let mut vars_int = HashMap::new();
    let vars = frame.vars.clone();
    for context::Var { n, t, v } in vars {
        add_variable(n, t, v, &ctx, sol);
        // TODO: the add_variable should also handle setting the correct value based on input type
        // TODO: assign proper val
        // so we need to be able to convert value into z3 int/bool
        //let val = 1;
        //sol.assert(&var.ge(&z3::ast::Int::from_i64(&ctx, val)));
        //sol.assert(&var.le(&z3::ast::Int::from_i64(&ctx, val)));
        //vars_int.insert(name, var);
    }

    // TODO: run the assertion to provea
    // so we need to be able to convert bool into z3 bool
    let to_prove = frame.command;
    println!("{}", to_prove.clone());
    let (needed, to_prove_z3) = to_prove.as_bool(&ctx);
    sol.assert(&to_prove_z3);
    needed
}

fn add_variable(var: Variable, t: Type, v: Value, ctx: &z3::Context, sol: &z3::Solver) {
    match t {
        Type::I32 => match var.clone() {
            Variable::Named(name) => {
                //let t = z3::ast::Int::new_const(ctx, name);
                sol.assert(
                    &Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(var))),
                        Expr::Value(Box::new(v)),
                    )
                    .as_bool(ctx),
                );
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
                Variable::ArrayElem(name, ind) => unimplemented!(),
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
            Value::Bool(b) => panic!("Bool value used as an int"),
            Value::Variable(x) => match x {
                Variable::Named(name) => z3::ast::Int::new_const(ctx, name),
                Variable::Empty => unimplemented!(),
                Variable::ArrayElem(name, ind) => unimplemented!(),
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
    fn as_bool<'a>(self, ctx: &'a z3::Context) -> (bool, z3::ast::Bool<'a>) {
        match self {
            Command::ProveControl(a) => match a {
                ProveControl::Assert(b) => (true, b.as_bool(ctx)),
                ProveControl::Assume(b) => (true, b.as_bool(ctx)),
                ProveControl::LoopInvariant(b) => (true, b.as_bool(ctx)),
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
        };

        assert_eq!(prove_frame(frame), Some(z3::SatResult::Sat));
    }

    #[test]
    fn prove_frame2() {
        let com = Command::ProveControl(ProveControl::Assert(Bool::False));
        let frame = context::Frame {
            command: com,
            funcs: Vec::new(),
            vals: Vec::new(),
            vars: Vec::new(),
        };

        assert_eq!(prove_frame(frame), Some(z3::SatResult::Unsat));
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
        };

        assert_eq!(prove_frame(frame), Some(z3::SatResult::Sat));
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
        };

        assert_eq!(prove_frame(frame), Some(z3::SatResult::Unsat));
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
        };

        assert_eq!(prove_frame(frame), Some(z3::SatResult::Sat));
    }
}
