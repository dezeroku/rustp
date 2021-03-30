use crate::ast::*;
use itertools::Itertools;

/// Describe the state, as we enter the command (state BEFORE it's run)
/// Keep everything needed to exactly reproduce the command without the whole program at hand
#[derive(PartialEq, Clone, Debug, Hash, Eq)]
pub struct Frame {
    pub command: Command,
    pub funcs: Vec<Function>,
    pub vals: Vec<Val>,
    pub vars: Vec<Var>,
}

// Keeping track of the values may be helpful in the validator part
#[derive(PartialEq, Clone, Debug, Hash, Eq)]
pub struct Val {
    pub v: Value,
    pub t: Type,
}

#[derive(PartialEq, Clone, Debug, Hash, Eq)]
pub struct Var {
    pub n: Variable,
    pub t: Type,
    pub v: Value,
}

pub fn get_context_func(func: Function, program: Program) -> Vec<Frame> {
    let mut temp = program.content;
    if let Some(pos) = temp.iter().position(|x| *x == func) {
        temp.remove(pos);
    }

    // Remove content from functions, it's not needed anyway and makes the debug output clearer
    let mut funcs = Vec::new();
    for i in temp {
        let mut t = i;
        t.content = Vec::new();
        funcs.push(t);
    }

    let mut result = Vec::new();

    let mut vars = Vec::new();
    for v in func.input {
        match v {
            Binding::Declaration(name, t, _) => {
                vars.push(Var {
                    n: name,
                    t: t,
                    v: Value::Unknown,
                });
            }

            _ => {}
        }
    }

    let mut prev_frame = Frame {
        command: Command::Noop,
        funcs: funcs,
        vals: Vec::new(),
        vars: vars,
    };

    for command in func.content {
        let f = get_context_command(command.clone(), &prev_frame);
        let mut t = prev_frame.clone();
        t.command = command;
        result.push(t);
        prev_frame = f;
    }

    result
}

pub fn get_context_block(block: Vec<Command>, frame: &Frame) -> Vec<Frame> {
    let mut result = Vec::new();
    result
}

/// Return the context of state AFTER the command is run
/// Extract all the vals and vars that are declared in this command
pub fn get_context_command(command: Command, frame: &Frame) -> Frame {
    let funcs = frame.funcs.clone();
    let mut vals = frame.vals.clone();
    let mut vars = frame.vars.clone();

    let mut vals_temp = Vec::new();

    match command.clone() {
        // TODO: handle all the other commands
        // treat block as single commands for now, unpack it individually later on
        Command::ProveControl(ProveControl::Assert(a)) => {
            _get_vals_bool(a, &mut vals_temp);
        }
        Command::ProveControl(ProveControl::Assume(a)) => {
            _get_vals_bool(a, &mut vals_temp);
        }

        Command::ProveControl(ProveControl::LoopInvariant(a)) => {
            _get_vals_bool(a, &mut vals_temp);
        }

        Command::Binding(Binding::Declaration(n, t, _)) => {
            vars.push(Var {
                n: n,
                t: t,
                v: Value::Unknown,
            });
        }

        Command::Binding(Binding::Assignment(n, t, v, _)) => {
            _get_vals_val(v.clone(), &mut vals_temp, t.clone());
            vars.push(Var { n: n, t: t, v: v });
        }

        Command::Binding(Binding::Tuple(decs)) => {
            for i in decs {
                match i {
                    Command::Binding(Binding::Declaration(n, t, _)) => {
                        vars.push(Var {
                            n: n,
                            t: t,
                            v: Value::Unknown,
                        });
                    }

                    Command::Binding(Binding::Assignment(n, t, v, _)) => {
                        _get_vals_val(v.clone(), &mut vals_temp, t.clone());
                        vars.push(Var { n: n, t: t, v: v });
                    }
                    _ => {}
                }
            }
        }

        Command::Assignment(Assignment::Single(n, v)) => {
            // TODO: Try to find the variable in the list and get the type from there.
            // It should actually be enough to just find it and change its value, it already has to exist in the frame context.
            // Get the type from it and base the val searching on it
            let t = Type::Unknown;
            _get_vals_val(v, &mut vals_temp, t);
        }

        Command::Assignment(Assignment::Tuple(decs)) => {
            for i in decs {
                // TODO: reuse the code for Assignment::Single above when it's implemented
            }
        }

        Command::Block(a) => {
            // Just add the block and the previous frame, if user wants to check it in detail he can call the block specific function
        }
        Command::Noop => {}
    }

    // TODO: Use the vals_temp to extract new variables that could have occured.

    vals.append(&mut vals_temp);
    vals = vals.into_iter().unique().collect();

    let result = Frame {
        command: command,
        funcs: funcs,
        vals: vals,
        vars: vars,
    };

    result
}

/// Unpack the bool and get all the values that are used in it
fn get_vals_bool(b: Bool) -> Vec<Val> {
    let mut result = Vec::new();

    _get_vals_bool(b, &mut result);

    result
}

fn _get_vals_bool(z: Bool, mut decs: &mut Vec<Val>) {
    match z {
        Bool::And(a, b) => {
            _get_vals_bool(*a, &mut decs);
            _get_vals_bool(*b, &mut decs);
        }
        Bool::Or(a, b) => {
            _get_vals_bool(*a, &mut decs);
            _get_vals_bool(*b, &mut decs);
        }
        Bool::Not(a) => {
            _get_vals_bool(*a, &mut decs);
        }
        Bool::Value(a) => {
            _get_vals_val(*a, &mut decs, Type::Bool);
        }
        Bool::Equal(a, b) => {
            _get_vals_expr(a, &mut decs);
            _get_vals_expr(b, &mut decs);
        }
        Bool::GreaterEqual(a, b) => {
            _get_vals_expr(a, &mut decs);
            _get_vals_expr(b, &mut decs);
        }
        Bool::SmallerEqual(a, b) => {
            _get_vals_expr(a, &mut decs);
            _get_vals_expr(b, &mut decs);
        }
        Bool::Greater(a, b) => {
            _get_vals_expr(a, &mut decs);
            _get_vals_expr(b, &mut decs);
        }
        Bool::Smaller(a, b) => {
            _get_vals_expr(a, &mut decs);
            _get_vals_expr(b, &mut decs);
        }
        Bool::True => {}
        Bool::False => {}
    }
}

fn _get_vals_val(z: Value, mut decs: &mut Vec<Val>, t: Type) {
    match z.clone() {
        Value::Expr(a) => {
            decs.push(Val { v: z.clone(), t: t });
            _get_vals_expr(a, &mut decs);
        }
        Value::Bool(a) => {
            decs.push(Val { v: z.clone(), t: t });
            _get_vals_bool(a, &mut decs);
        }
        Value::Variable(v) => {
            decs.push(Val { v: z.clone(), t: t });
        }
        Value::Tuple(a) => {
            decs.push(Val { v: z.clone(), t: t });
            for i in a {
                _get_vals_val(i, &mut decs, Type::Unknown);
            }
        }
        Value::Array(a) => {
            decs.push(Val { v: z.clone(), t: t });
            for i in a {
                _get_vals_val(i, &mut decs, Type::Unknown);
            }
        }
        Value::FunctionCall(name, a) => {
            decs.push(Val { v: z.clone(), t: t });
            for i in a {
                _get_vals_val(i, &mut decs, Type::Unknown);
            }
        }
        Value::Dereference(a) => {
            decs.push(Val { v: z.clone(), t: t });
            _get_vals_val(*a, &mut decs, Type::Unknown);
        }
        Value::Reference(a) => {
            decs.push(Val { v: z.clone(), t: t });
            _get_vals_val(*a, &mut decs, Type::Unknown);
        }
        Value::ReferenceMutable(a) => {
            decs.push(Val { v: z.clone(), t: t });
            _get_vals_val(*a, &mut decs, Type::Unknown);
        }
        Value::Unit => {}
        Value::Unknown => {}
    }
}

fn _get_vals_expr(z: Expr, mut decs: &mut Vec<Val>) {
    match z {
        Expr::Number(_) => {}
        Expr::Op(a, _, b) => {
            _get_vals_expr(*a, &mut decs);
            _get_vals_expr(*b, &mut decs);
        }
        Expr::Value(a) => _get_vals_val(*a, &mut decs, Type::I32),
    }
}

// this function relies on the _the_vals_val returning ALL the meaningful vals that it contains recursively
fn _get_vars_val(z: Val, mut decs: &mut Vec<Var>) {
    match z.v.clone() {
        Value::Variable(v) => {
            decs.push(Var {
                n: v,
                t: z.t,
                v: Value::Unknown,
            });
        }
        _ => {}
    }
}
