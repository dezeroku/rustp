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

    true
    //println!("START");
    // Idea:
    // declare constants for all identifiers that are there (variables)
    // use asserts to assign values
    // assert the assertion in the end
    // try to check it

    //let mut cfg = z3::Config::new();
    //cfg.set_model_generation(true);
    //let te: i32 = 20;
    //let ctx = z3::Context::new(&cfg);
    //// TODO: use goal?
    ////let t = z3::Goal::new(&ctx, true, false, false);
    ////t.assert(&z3::ast::Ast());
    //let t = z3::Solver::new(&ctx);
    //let b = z3::ast::Bool::new_const(&ctx, "b");
    //let c = z3::ast::Bool::new_const(&ctx, "c");
    //let d = z3::ast::Bool::new_const(&ctx, "d");
    //let x = z3::ast::Int::new_const(&ctx, "x");
    //let y = z3::ast::Int::new_const(&ctx, "y");
    //t.assert(&z3::ast::Bool::and(&ctx, &[&b, &c]));
    //t.assert(&z3::ast::Bool::and(&ctx, &[&c, &d]));
    //t.assert(&x.gt(&y));
    //t.assert(&y.gt(&z3::ast::Int::from_i64(&ctx, te.into())));
    ////t.assert(&z3::ast::Bool::not(&z3::ast::Bool::and(&ctx, &[&c, &d])));
    ////t.assert(&z3::ast::Bool::from_bool(&ctx, c == d));
    //let f = t.check();
    //println!("{:?}", f);
    //println!("{:?}", t.get_model());
    ////println!("{:?}", t.get_proof());
    //println!("DONE");
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
    let mut vars_int = HashMap::new();
    let vars = frame.vars.clone();
    for context::Var { n, t, v } in vars {
        let (name, var) = add_variable(n, t, v, &ctx);
        // TODO: assign proper val
        // so we need to be able to convert value into z3 int/bool
        let val = 1;
        sol.assert(&var.ge(&z3::ast::Int::from_i64(&ctx, val)));
        sol.assert(&var.le(&z3::ast::Int::from_i64(&ctx, val)));
        vars_int.insert(name, var);
    }

    // TODO: run the assertion to provea
    // so we need to be able to convert bool into z3 bool
    let to_prove = frame.command;
    println!("{}", to_prove.clone());
    let (needed, to_prove_z3) = to_prove.as_bool(&ctx);
    sol.assert(&to_prove_z3);
    needed
}

fn add_variable(var: Variable, t: Type, v: Value, ctx: &z3::Context) -> (String, z3::ast::Int) {
    match t {
        Type::I32 => match var.clone() {
            Variable::Named(name) => {
                let t = z3::ast::Int::new_const(ctx, name.clone());
                (name, t)
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

trait Provable {
    fn as_bool<'a>(self, ctx: &'a z3::Context) -> z3::ast::Bool<'a>;
}

impl Provable for Bool {
    fn as_bool<'a>(self, ctx: &'a z3::Context) -> z3::ast::Bool<'a> {
        match self {
            Bool::True => (z3::ast::Bool::from_bool(&ctx, true)),
            Bool::False => (z3::ast::Bool::from_bool(&ctx, false)),
            Bool::And(_a, _b) => {
                let a: Bool = *_a;
                let b: Bool = *_b;
                z3::ast::Bool::and(ctx, &[&a.as_bool(ctx), &b.as_bool(ctx)])
            }
            _ => unimplemented!(),
        }
    }
}

//    And(Box<Bool>, Box<Bool>),
//    Or(Box<Bool>, Box<Bool>),
//    Not(Box<Bool>),
//    Value(Box<Value>),
//    True,
//    False,
//    Equal(Expr, Expr),
//    GreaterEqual(Expr, Expr),
//    SmallerEqual(Expr, Expr),
//    Greater(Expr, Expr),
//    Smaller(Expr, Expr),

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
