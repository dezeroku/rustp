use crate::ast::*;

mod tests;

static FORBIDDEN_DECS: [&'static str; 1] = ["return_value"];

pub fn validate(input: Program) -> bool {
    no_shadowing(input.clone()) && no_forbidden_decs(input.clone()) && no_undefined(input)
}

/// Check for using undefined variables in ProveCommands
fn no_undefined(input: Program) -> bool {
    let mut functions = Vec::new();
    for func in input.content.clone() {
        functions.push(func.name);
    }

    for func in input.content {
        let mut definitions = Vec::new();
        // Add function params as recognized names
        for i in func.clone().input {
            match i {
                // TODO: clean this up
                Binding::Declaration(name, _, _) => match name.clone() {
                    Variable::Named(x) => {
                        def_push(&mut definitions, name);
                        def_push(&mut definitions, Variable::Named(x + "'old"));
                    }
                    _ => unimplemented!(),
                },
                _ => panic!("Unsupported function parameter provided: {}", i),
            }
            //
        }

        if !no_undefined_func(func, definitions.clone(), functions.clone()) {
            return false;
        }
    }

    true
}

enum Namedec {
    Variable(Variable),
    Name(String),
}

/// Unpack the bool and get all the variables that are used in it
fn get_namedecs(b: Bool) -> Vec<Namedec> {
    let mut result = Vec::new();

    _get_namedecs_bool(b, &mut result);

    result
}

fn _get_namedecs_bool(z: Bool, mut decs: &mut Vec<Namedec>) {
    match z {
        Bool::And(a, b) => {
            _get_namedecs_bool(*a, &mut decs);
            _get_namedecs_bool(*b, &mut decs);
        }
        Bool::Or(a, b) => {
            _get_namedecs_bool(*a, &mut decs);
            _get_namedecs_bool(*b, &mut decs);
        }
        Bool::Not(a) => {
            _get_namedecs_bool(*a, &mut decs);
        }
        Bool::Value(a) => {
            _get_namedecs_val(*a, &mut decs);
        }
        Bool::Equal(a, b) => {
            _get_namedecs_expr(a, &mut decs);
            _get_namedecs_expr(b, &mut decs);
        }
        Bool::GreaterEqual(a, b) => {
            _get_namedecs_expr(a, &mut decs);
            _get_namedecs_expr(b, &mut decs);
        }
        Bool::LowerEqual(a, b) => {
            _get_namedecs_expr(a, &mut decs);
            _get_namedecs_expr(b, &mut decs);
        }
        Bool::GreaterThan(a, b) => {
            _get_namedecs_expr(a, &mut decs);
            _get_namedecs_expr(b, &mut decs);
        }
        Bool::LowerThan(a, b) => {
            _get_namedecs_expr(a, &mut decs);
            _get_namedecs_expr(b, &mut decs);
        }
        Bool::True => {}
        Bool::False => {}
    }
}

fn _get_namedecs_val(z: Value, mut decs: &mut Vec<Namedec>) {
    match z {
        Value::Expr(a) => {
            _get_namedecs_expr(a, &mut decs);
        }
        Value::Bool(a) => {
            _get_namedecs_bool(a, &mut decs);
        }
        Value::Variable(v) => {
            decs.push(Namedec::Variable(v));
        }
        Value::Tuple(a) => {
            for i in a {
                _get_namedecs_val(i, &mut decs);
            }
        }
        Value::Array(a) => {
            for i in a {
                _get_namedecs_val(i, &mut decs);
            }
        }
        Value::FunctionCall(name, a) => {
            decs.push(Namedec::Name(name));
            for i in a {
                _get_namedecs_val(i, &mut decs);
            }
        }
        Value::Dereference(a) => {
            _get_namedecs_val(*a, &mut decs);
        }
        Value::Reference(a) => {
            _get_namedecs_val(*a, &mut decs);
        }
        Value::ReferenceMutable(a) => {
            _get_namedecs_val(*a, &mut decs);
        }
        Value::Unit => {}
    }
}

fn _get_namedecs_expr(z: Expr, mut decs: &mut Vec<Namedec>) {
    match z {
        Expr::Number(_) => {}
        Expr::Op(a, _, b) => {
            _get_namedecs_expr(*a, &mut decs);
            _get_namedecs_expr(*b, &mut decs);
        }
        Expr::Value(a) => _get_namedecs_val(*a, &mut decs),
    }
}

fn def_push(definitions: &mut Vec<String>, val: Variable) {
    let name = match val {
        Variable::Named(a) => a,
        _ => return,
    };

    definitions.push(name);
}

fn no_undefined_check(
    definitions: &mut Vec<String>,
    functions: &Vec<String>,
    val: Namedec,
) -> bool {
    match val {
        Namedec::Variable(t) => {
            let name = match t {
                Variable::Named(a) => a,
                Variable::ArrayElem(a, _) => a,
                Variable::TupleElem(a, _) => a,
                _ => return true,
            };
            if !definitions.iter().any(|i| *i == name) {
                println!("Undefined variable used: {}", name);
                return false;
            }
        }
        Namedec::Name(name) => {
            if !functions.iter().any(|i| *i == name) {
                println!("Undefined function used: {}", name);
                return false;
            }
        }
    };

    true
}

fn no_undefined_logic(
    content: Vec<Command>,
    mut definitions: &mut Vec<String>,
    functions: &Vec<String>,
) -> bool {
    for comm in content {
        match comm {
            Command::ProveControl(ProveControl::Assert(a)) => {
                for i in get_namedecs(a) {
                    if !no_undefined_check(&mut definitions, &functions, i) {
                        return false;
                    }
                }
            }
            Command::ProveControl(ProveControl::Assume(a)) => {
                for i in get_namedecs(a) {
                    if !no_undefined_check(&mut definitions, &functions, i) {
                        return false;
                    }
                }
            }
            Command::Binding(Binding::Declaration(name, _, _)) => {
                def_push(&mut definitions, name);
            }
            Command::Binding(Binding::Assignment(name, _, _, _)) => {
                def_push(&mut definitions, name);
            }
            Command::Binding(Binding::Tuple(vec)) => {
                for dec in vec {
                    match dec {
                        Command::Binding(Binding::Declaration(name, _, _)) => {
                            def_push(&mut definitions, name);
                        }
                        Command::Binding(Binding::Assignment(name, _, _, _)) => {
                            def_push(&mut definitions, name);
                        }
                        _ => {}
                    }
                }
            }
            Command::Block(Block::If(_, blocks, el)) => {
                let mut temp = blocks;
                temp.push(el);

                for block in temp {
                    let mut state = definitions.clone();
                    if !no_undefined_logic(block, &mut state, &functions) {
                        return false;
                    }
                }
            }
            Command::Block(Block::ForRange(_, _, _, vec, a)) => {
                let mut temp = definitions.clone();

                if !no_undefined_logic(vec, &mut temp, &functions) {
                    return false;
                }

                for i in get_namedecs(a) {
                    if !no_undefined_check(&mut definitions, &functions, i) {
                        return false;
                    }
                }
            }
            Command::Block(Block::While(_, vec, a)) => {
                let mut temp = definitions.clone();

                if !no_undefined_logic(vec, &mut temp, &functions) {
                    return false;
                }

                for i in get_namedecs(a) {
                    if !no_undefined_check(&mut definitions, &functions, i) {
                        return false;
                    }
                }
            }

            _ => {}
        }
    }

    true
}

fn no_undefined_func(func: Function, mut definitions: Vec<String>, functions: Vec<String>) -> bool {
    no_undefined_logic(func.content, &mut definitions, &functions)
}

/// Check for declarations of reserved names in declarations (like e.g. return_value)
fn no_forbidden_decs(input: Program) -> bool {
    for func in input.content {
        if !no_forbidden_decs_func(func) {
            return false;
        }
    }

    true
}

fn no_forbidden_decs_check(val: Variable) -> bool {
    match val {
        Variable::Named(a) => {
            if !FORBIDDEN_DECS.contains(&a.as_str()) {
                if !a.ends_with("'old") {
                    true
                } else {
                    println!("'old variable used in binding: {}", a);
                    false
                }
            } else {
                println!("Keyword variable used in binding: {}", a);
                false
            }
        }
        _ => true,
    }
}

fn no_forbidden_decs_logic(content: Vec<Command>) -> bool {
    for comm in content {
        match comm {
            Command::Binding(Binding::Declaration(name, _, _)) => {
                if !no_forbidden_decs_check(name) {
                    return false;
                }
            }
            Command::Binding(Binding::Assignment(name, _, _, _)) => {
                if !no_forbidden_decs_check(name) {
                    return false;
                }
            }
            Command::Binding(Binding::Tuple(vec)) => {
                for dec in vec {
                    match dec {
                        Command::Binding(Binding::Declaration(name, _, _)) => {
                            if !no_forbidden_decs_check(name) {
                                return false;
                            }
                        }
                        Command::Binding(Binding::Assignment(name, _, _, _)) => {
                            if !no_forbidden_decs_check(name) {
                                return false;
                            }
                        }
                        _ => {}
                    }
                }
            }
            Command::Block(Block::If(_, blocks, el)) => {
                let mut temp = blocks;
                temp.push(el);

                for block in temp {
                    if !no_forbidden_decs_logic(block) {
                        return false;
                    }
                }
            }
            Command::Block(Block::ForRange(iter, _, _, vec, _)) => {
                if !no_forbidden_decs_check(iter) {
                    return false;
                }

                if !no_forbidden_decs_logic(vec) {
                    return false;
                }
            }
            Command::Block(Block::While(_, vec, _)) => {
                if !no_forbidden_decs_logic(vec) {
                    return false;
                }
            }

            _ => {}
        }
    }

    true
}

fn no_forbidden_decs_func(func: Function) -> bool {
    no_forbidden_decs_logic(func.content)
}

/// Check for overshadowing declarations (the ones that overwrite another)
fn no_shadowing(input: Program) -> bool {
    let mut definitions = Vec::new();
    for func in input.content.clone() {
        definitions.push(func.name);
    }

    for func in input.content {
        if !no_shadowing_func(func, definitions.clone()) {
            return false;
        }
    }

    true
}

fn no_shadowing_check(definitions: &mut Vec<String>, val: Variable) -> bool {
    let name = match val {
        Variable::Named(a) => a,
        _ => return true,
    };

    if definitions.iter().any(|i| *i == name) {
        println!("Variable redeclared: {}", name);
        return false;
    } else {
        definitions.push(name)
    }
    true
}

fn no_shadowing_logic(content: Vec<Command>, mut definitions: &mut Vec<String>) -> bool {
    for comm in content {
        match comm {
            Command::Binding(Binding::Declaration(name, _, _)) => {
                if !no_shadowing_check(&mut definitions, name) {
                    return false;
                }
            }
            Command::Binding(Binding::Assignment(name, _, _, _)) => {
                if !no_shadowing_check(&mut definitions, name) {
                    return false;
                }
            }
            Command::Binding(Binding::Tuple(vec)) => {
                for dec in vec {
                    match dec {
                        Command::Binding(Binding::Declaration(name, _, _)) => {
                            if !no_shadowing_check(&mut definitions, name) {
                                return false;
                            }
                        }
                        Command::Binding(Binding::Assignment(name, _, _, _)) => {
                            if !no_shadowing_check(&mut definitions, name) {
                                return false;
                            }
                        }
                        _ => {}
                    }
                }
            }
            Command::Block(Block::If(_, blocks, el)) => {
                let mut temp = blocks;
                temp.push(el);

                for block in temp {
                    let mut state = definitions.clone();
                    if !no_shadowing_logic(block, &mut state) {
                        return false;
                    }
                }
            }
            Command::Block(Block::ForRange(iter, _, _, vec, _)) => {
                let mut temp = definitions.clone();
                if !no_shadowing_check(&mut temp, iter) {
                    return false;
                }

                if !no_shadowing_logic(vec, &mut temp) {
                    return false;
                }
            }
            Command::Block(Block::While(_, vec, _)) => {
                let mut temp = definitions.clone();

                if !no_shadowing_logic(vec, &mut temp) {
                    return false;
                }
            }

            _ => {}
        }
    }

    true
}

fn no_shadowing_func(func: Function, mut definitions: Vec<String>) -> bool {
    no_shadowing_logic(func.content, &mut definitions)
}
