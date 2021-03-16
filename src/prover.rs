use crate::ast::*;
use std::collections::HashMap;

pub fn prove(input: Program) {
    // Create context for each command and try to prove it individually?
    // All that we have to prove are assertions, all the rest just modifies context.

    // For now just display everything here when it happens.
    println!("HAPPENS!");
}

struct Context {
    command: Command,
    state: HashMap<Variable, Val>,
}

struct Val {
    v: Value,
    t: Type,
}
