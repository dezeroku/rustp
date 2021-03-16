use crate::ast::*;
use std::collections::HashMap;

pub fn prove(input: Program) {
    // Create context for each command and try to prove it individually?
    // All that we have to prove are assertions, all the rest just modifies context.

    // For now just display everything here when it happens.
    println!("HAPPENS!");

    for func in input.content {
        let mut temp = func.content;
        if func.output != Type::Unit {
            temp.push(Command::Binding(Binding::Assignment(
                Variable::Named(String::from("return_value")),
                func.output,
                func.return_value,
                false,
            )));
        }

        temp.push(Command::ProveControl(ProveControl::Assert(
            func.postcondition,
        )));

        let con = create_context_func(temp, HashMap::new(), func.precondition);
        // Try to prove
        println!("{:?}", con);
    }
}

fn create_context_func(
    content: Vec<Command>,
    mut state: HashMap<Variable, Val>,
    known: Bool,
) -> Vec<Context> {
    let mut result = Vec::new();

    for comm in content {
        match comm {
            Command::ProveControl(_) => {
                result.push(Context {
                    command: comm,
                    state: state.clone(),
                    known: known.clone(),
                });
            }
            Command::Binding(Binding::Declaration(_, _, _)) => {
                // TODO:
            }
            Command::Binding(Binding::Assignment(name, t, v, _)) => {
                //result.push(Context {
                //    command: comm,
                //    state: state.clone(),
                //});

                state.insert(name, Val { v: v, t: t });
            }
            Command::Binding(Binding::Tuple(vec)) => {
                for dec in vec {
                    match dec {
                        Command::Binding(Binding::Declaration(name, _, _)) => {
                            // TODO:
                        }
                        Command::Binding(Binding::Assignment(name, t, v, _)) => {
                            state.insert(name, Val { v: v, t: t });
                        }
                        _ => {}
                    }
                }
            }
            Command::Block(Block::If(_, blocks, el)) => {
                // TODO:
                //let mut temp = blocks;
                //temp.push(el);

                //for block in temp {
                //    let mut state = definitions.clone();
                //    if !no_shadowing_logic(block, &mut state) {
                //        return false;
                //    }
                //}
            }
            Command::Block(Block::ForRange(iter, _, _, vec)) => {
                //let mut temp = definitions.clone();
                //if !no_shadowing_check(&mut temp, iter) {
                //    return false;
                //}

                //if !no_shadowing_logic(vec, &mut temp) {
                //    return false;
                //}
            }
            _ => {}
        }
    }

    return result;
}

#[derive(PartialEq, Clone, Debug)]
struct Context {
    command: Command,
    state: HashMap<Variable, Val>,
    known: Bool,
}

#[derive(PartialEq, Clone, Debug)]
struct Val {
    v: Value,
    t: Type,
}
