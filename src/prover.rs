use crate::ast::*;
use crate::context;
use std::collections::HashMap;
use z3;

pub fn prove(input: Program) {
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
            prove_frame(frame)
        }
    }

    println!("START");
    let mut cfg = z3::Config::new();
    cfg.set_model_generation(true);
    cfg.set_proof_generation(true);
    let ctx = z3::Context::new(&cfg);
    // TODO: use goal?
    //let t = z3::Goal::new(&ctx, true, false, false);
    //t.assert(&z3::ast::Ast());
    let t = z3::Solver::new(&ctx);
    let b = z3::ast::Bool::new_const(&ctx, "b");
    let c = z3::ast::Bool::new_const(&ctx, "c");
    let d = z3::ast::Bool::new_const(&ctx, "d");
    t.assert(&z3::ast::Bool::and(&ctx, &[&b, &c]));
    t.assert(&z3::ast::Bool::and(&ctx, &[&c, &d]));
    //t.assert(&z3::ast::Bool::not(&z3::ast::Bool::and(&ctx, &[&c, &d])));
    //t.assert(&z3::ast::Bool::from_bool(&ctx, c == d));
    let f = t.check();
    println!("{:?}", f);
    println!("{:?}", t.get_model());
    //println!("{:?}", t.get_proof());
    println!("DONE");
}

fn prove_frame(frame: context::Frame) {
    // convert to Z3 problem and run it
}
