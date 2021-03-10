use crate::ast;
use std::collections::HashMap;

/// Infer types for all the bindings based on types of the r_value
pub fn simplify(program: ast::Program) -> ast::Program {
    // Find the bindings, which use ast::Type::Unknown
    // For these bindings, find a context (all the following commands, until the rebinding of same name)
    // Based on the context, find assignments and infer type based on r_value
    let mut result = Vec::new();
    let funcs = program.content.clone();
    for func in program.content {
        result.push(simplify_function(func, &funcs));
    }

    ast::Program { content: result }
}

fn find_type_val(
    val: &ast::Value,
    state: &HashMap<ast::Variable, ast::Type>,
    funcs: &Vec<ast::Function>,
) -> ast::Type {
    match val {
        ast::Value::Bool(_) => ast::Type::Bool,
        ast::Value::Expr(_) => ast::Type::I32,
        ast::Value::Variable(x) => match state.get(x) {
            Some(a) => a.clone(),
            None => ast::Type::Unknown, //panic!("Variable with unknown type assigned to the right"),
        },
        ast::Value::Array(v) => {
            let l = v.clone().len() as i32;
            for i in v {
                let t = find_type_val(i, state, funcs);
                if t != ast::Type::Unknown {
                    return ast::Type::Array(Box::new(t), l);
                }
            }
            panic!("Matching type not found")
        }
        ast::Value::Tuple(v) => {
            let mut types = Vec::new();
            for i in v {
                let t = find_type_val(i, state, funcs);
                types.push(t);
            }

            ast::Type::Tuple(types)
        }
        ast::Value::Dereference(v) => match &**v {
            ast::Value::Reference(x) => find_type_val(&*x, state, funcs),
            ast::Value::ReferenceMutable(x) => find_type_val(&*x, state, funcs),
            _ => panic!("Incorrect dereference"),
        },
        ast::Value::FunctionCall(name, _) => {
            for i in funcs {
                if &i.name == name {
                    return i.output.clone();
                }
            }
            panic!("Unknown function called")
        }
        ast::Value::Reference(v) => {
            let t = find_type_val(&*v, state, funcs);
            ast::Type::Reference(Box::new(t))
        }
        ast::Value::ReferenceMutable(v) => {
            let t = find_type_val(&*v, state, funcs);
            ast::Type::ReferenceMutable(Box::new(t))
        }
        _ => unimplemented!(),
    }
}

fn simplify_function_val(
    function: ast::Function,
    funcs: &Vec<ast::Function>,
    to_check: Vec<ast::Command>,
) -> ast::Function {
    let input = function.input.clone();

    let mut state: HashMap<ast::Variable, ast::Type> = HashMap::new();
    let mut result = Vec::new();

    for i in input {
        match i {
            ast::Binding::Declaration(name, t, _) => {
                state.insert(name, t);
            }
            _ => panic!("Is this correct function definition?"),
        }
    }

    for comm in function.content {
        if to_check.iter().any(|i| i.clone() == comm) {
            // rework the comm based on state
            match comm.clone() {
                ast::Command::Binding(a) => match a {
                    ast::Binding::Assignment(name, _, val, m) => {
                        let ty = find_type_val(&val, &state, funcs);
                        result.push(ast::Command::Binding(ast::Binding::Assignment(
                            name, ty, val, m,
                        )))
                    }
                    ast::Binding::Declaration(_, _, _) => {
                        // TODO: This will have to be solved using backwards iteration in another function (e.g. find_type_dec)
                        result.push(comm)
                    }
                    ast::Binding::Tuple(binds) => {
                        // TODO: Try to solve individually for each case
                        result.push(comm)
                    }
                },
                ast::Command::Block(a) => {
                    unimplemented!();
                }
                _ => panic!("This is not a binding"),
            }
        } else {
            match comm.clone() {
                ast::Command::Binding(a) => match a {
                    ast::Binding::Assignment(name, t, _, _) => {
                        state.insert(name, t);
                        result.push(comm)
                    }
                    ast::Binding::Declaration(name, t, _) => {
                        state.insert(name, t);
                        result.push(comm)
                    }
                    ast::Binding::Tuple(binds) => {
                        for i in binds {
                            match i {
                                ast::Command::Binding(ast::Binding::Assignment(name, t, _, _)) => {
                                    state.insert(name, t);
                                }
                                ast::Command::Binding(ast::Binding::Declaration(name, t, _)) => {
                                    state.insert(name, t);
                                }
                                _ => panic!("No nesting support"),
                            }
                        }
                        result.push(comm)
                    }
                },
                _ => result.push(comm),
            }
        }
    }
    ast::Function {
        name: function.name,
        content: result,
        input: function.input,
        output: function.output,
        precondition: function.precondition,
        postcondition: function.postcondition,
        return_value: function.return_value,
    }
}

fn simplify_function(function: ast::Function, funcs: &Vec<ast::Function>) -> ast::Function {
    let mut temp = function.content.clone();

    let mut last = Vec::new();

    let mut to_check = unknown_type_bindings(temp);

    let mut modified = function;
    while last != to_check {
        println!("CHECK: |{:?}|", to_check);
        last = to_check.clone();

        // TODO: run calls to simplify_function_dec in the same loop
        modified = simplify_function_val(modified, funcs, to_check);

        temp = modified.content.clone();
        to_check = unknown_type_bindings(temp);
    }
    modified
}

fn unknown_type_single(input: ast::Command) -> Option<ast::Command> {
    match input.clone() {
        ast::Command::Binding(a) => match a {
            ast::Binding::Assignment(_, t, _, _) => match t {
                ast::Type::Unknown => Some(input),
                _ => None,
            },
            ast::Binding::Declaration(_, t, _) => match t {
                ast::Type::Unknown => Some(input),
                _ => None,
            },
            ast::Binding::Tuple(binds) => {
                let mut temp = Vec::new();
                for i in binds {
                    match unknown_type_single(i) {
                        Some(x) => temp.push(x),
                        None => {}
                    }
                }
                if temp.len() > 0 {
                    Some(input)
                } else {
                    None
                }
            }
        },
        _ => None,
    }
}

fn unknown_type_bindings(input: Vec<ast::Command>) -> Vec<ast::Command> {
    let mut to_return = Vec::new();
    for comm in input {
        match comm.clone() {
            ast::Command::Block(a) => {
                match a {
                    // TODO: handle this
                    ast::Block::If(_, comms, el) => {
                        let mut t = comms.clone();
                        t.push(el.clone());
                        let l: usize = t
                            .iter()
                            .map(|x| {
                                let t = x.clone();
                                unknown_type_bindings(t)
                            })
                            .collect::<Vec<Vec<ast::Command>>>()
                            .iter()
                            .map(|y| y.len())
                            .collect::<Vec<usize>>()
                            .iter()
                            .sum();
                        if l > 0 {
                            to_return.push(comm);
                        }
                    }
                    ast::Block::ForRange(_, _, _, comms) => {
                        let f = comms.clone();
                        let t = unknown_type_bindings(f);
                        if t.len() > 0 {
                            to_return.push(comm);
                        }
                    }
                }
            }
            a => match unknown_type_single(a) {
                Some(x) => to_return.push(x),
                None => {}
            },
        }
    }

    to_return
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unknown_type_single1() {
        assert_eq!(
            unknown_type_single(ast::Command::Binding(ast::Binding::Assignment(
                ast::Variable::Named(String::from("b")),
                ast::Type::Unknown,
                ast::Value::Expr(ast::Expr::Number(1)),
                false
            ))),
            Some(ast::Command::Binding(ast::Binding::Assignment(
                ast::Variable::Named(String::from("b")),
                ast::Type::Unknown,
                ast::Value::Expr(ast::Expr::Number(1)),
                false
            )))
        );
    }

    #[test]
    fn unknown_type_single2() {
        assert_eq!(
            unknown_type_single(ast::Command::Binding(ast::Binding::Assignment(
                ast::Variable::Named(String::from("b")),
                ast::Type::I32,
                ast::Value::Expr(ast::Expr::Number(1)),
                false
            ))),
            None
        );
    }

    #[test]
    fn unknown_type_single3() {
        let mut temp = Vec::new();
        temp.push(ast::Command::Binding(ast::Binding::Declaration(
            ast::Variable::Named("a".to_string()),
            ast::Type::Unknown,
            false,
        )));
        temp.push(ast::Command::Binding(ast::Binding::Declaration(
            ast::Variable::Named("b".to_string()),
            ast::Type::Unknown,
            true,
        )));
        assert_eq!(
            unknown_type_single(ast::Command::Binding(ast::Binding::Tuple(temp.clone()))),
            Some(ast::Command::Binding(ast::Binding::Tuple(temp)))
        );
    }

    #[test]
    fn unknown_type_single4() {
        let mut temp = Vec::new();
        temp.push(ast::Command::Binding(ast::Binding::Declaration(
            ast::Variable::Named("a".to_string()),
            ast::Type::Unknown,
            false,
        )));
        temp.push(ast::Command::Binding(ast::Binding::Declaration(
            ast::Variable::Named("b".to_string()),
            ast::Type::I32,
            true,
        )));
        assert_eq!(
            unknown_type_single(ast::Command::Binding(ast::Binding::Tuple(temp.clone()))),
            Some(ast::Command::Binding(ast::Binding::Tuple(temp)))
        );
    }

    #[test]
    fn unknown_type_single5() {
        let mut temp = Vec::new();
        temp.push(ast::Command::Binding(ast::Binding::Declaration(
            ast::Variable::Named("a".to_string()),
            ast::Type::Bool,
            false,
        )));
        temp.push(ast::Command::Binding(ast::Binding::Declaration(
            ast::Variable::Named("b".to_string()),
            ast::Type::I32,
            true,
        )));
        assert_eq!(
            unknown_type_single(ast::Command::Binding(ast::Binding::Tuple(temp))),
            None
        );
    }

    #[test]
    fn unknown_type_bindings1() {
        let mut temp = Vec::new();
        temp.push(ast::Command::Binding(ast::Binding::Declaration(
            ast::Variable::Named("a".to_string()),
            ast::Type::Bool,
            false,
        )));
        temp.push(ast::Command::Binding(ast::Binding::Declaration(
            ast::Variable::Named("b".to_string()),
            ast::Type::I32,
            true,
        )));

        let mut f = Vec::new();
        f.push(ast::Command::Binding(ast::Binding::Tuple(temp)));

        let t = ast::Command::Block(ast::Block::ForRange(
            ast::Variable::Named("i".to_string()),
            ast::Value::Expr(ast::Expr::Number(1)),
            ast::Value::Expr(ast::Expr::Number(1)),
            f,
        ));

        let mut to_test = Vec::new();
        to_test.push(t);

        let check: Vec<ast::Command> = Vec::new();
        assert_eq!(unknown_type_bindings(to_test), check);
    }

    #[test]
    fn unknown_type_bindings2() {
        let mut temp = Vec::new();
        temp.push(ast::Command::Binding(ast::Binding::Declaration(
            ast::Variable::Named("a".to_string()),
            ast::Type::Unknown,
            false,
        )));
        temp.push(ast::Command::Binding(ast::Binding::Declaration(
            ast::Variable::Named("b".to_string()),
            ast::Type::I32,
            true,
        )));

        let mut f = Vec::new();
        f.push(ast::Command::Binding(ast::Binding::Tuple(temp)));

        let t = ast::Command::Block(ast::Block::ForRange(
            ast::Variable::Named("i".to_string()),
            ast::Value::Expr(ast::Expr::Number(1)),
            ast::Value::Expr(ast::Expr::Number(1)),
            f,
        ));

        let mut to_test = Vec::new();
        to_test.push(t);

        assert_eq!(unknown_type_bindings(to_test.clone()), to_test);
    }

    #[test]
    fn unknown_type_bindings3() {
        let mut temp = Vec::new();
        temp.push(ast::Command::Binding(ast::Binding::Declaration(
            ast::Variable::Named("a".to_string()),
            ast::Type::I32,
            false,
        )));
        temp.push(ast::Command::Binding(ast::Binding::Declaration(
            ast::Variable::Named("b".to_string()),
            ast::Type::I32,
            true,
        )));

        let mut f = Vec::new();
        f.push(ast::Command::Binding(ast::Binding::Tuple(temp)));

        let mut x = Vec::new();
        x.push(f.clone());

        let t = ast::Command::Block(ast::Block::If(Vec::new(), x.clone(), f));

        let mut to_test = Vec::new();
        to_test.push(t);

        let check: Vec<ast::Command> = Vec::new();
        assert_eq!(unknown_type_bindings(to_test), check);
    }

    #[test]
    fn unknown_type_bindings4() {
        let mut temp = Vec::new();
        temp.push(ast::Command::Binding(ast::Binding::Declaration(
            ast::Variable::Named("a".to_string()),
            ast::Type::Unknown,
            false,
        )));
        temp.push(ast::Command::Binding(ast::Binding::Declaration(
            ast::Variable::Named("b".to_string()),
            ast::Type::I32,
            true,
        )));

        let mut f = Vec::new();
        f.push(ast::Command::Binding(ast::Binding::Tuple(temp)));

        let mut x = Vec::new();
        x.push(f.clone());

        let t = ast::Command::Block(ast::Block::If(Vec::new(), x.clone(), f));

        let mut to_test = Vec::new();
        to_test.push(t);

        assert_eq!(unknown_type_bindings(to_test.clone()), to_test);
    }

    #[test]
    fn find_type_val1() {
        let state: HashMap<ast::Variable, ast::Type> = HashMap::new();
        let funcs = Vec::new();

        let val = ast::Value::Expr(ast::Expr::Number(12));

        assert_eq!(find_type_val(&val, &state, &funcs), ast::Type::I32)
    }

    #[test]
    fn find_type_val2() {
        let state: HashMap<ast::Variable, ast::Type> = HashMap::new();
        let funcs = Vec::new();

        let val = ast::Value::Bool(ast::Bool::True);

        assert_eq!(find_type_val(&val, &state, &funcs), ast::Type::Bool)
    }

    #[test]
    fn find_type_val3() {
        let mut state: HashMap<ast::Variable, ast::Type> = HashMap::new();
        let funcs = Vec::new();

        state.insert(ast::Variable::Named(String::from("y")), ast::Type::I32);

        let val = ast::Value::Variable(ast::Variable::Named(String::from("y")));

        assert_eq!(find_type_val(&val, &state, &funcs), ast::Type::I32)
    }

    #[test]
    fn find_type_val4() {
        let state: HashMap<ast::Variable, ast::Type> = HashMap::new();
        let funcs = Vec::new();

        let mut t = Vec::new();

        t.push(ast::Value::Expr(ast::Expr::Number(12)));
        t.push(ast::Value::Variable(ast::Variable::Named(String::from(
            "b",
        ))));
        let val = ast::Value::Array(t);

        assert_eq!(
            find_type_val(&val, &state, &funcs),
            ast::Type::Array(Box::new(ast::Type::I32), 2)
        )
    }

    #[test]
    fn find_type_val5() {
        let state: HashMap<ast::Variable, ast::Type> = HashMap::new();
        let funcs = Vec::new();

        let mut t = Vec::new();

        t.push(ast::Value::Variable(ast::Variable::Named(String::from(
            "b",
        ))));
        t.push(ast::Value::Expr(ast::Expr::Number(12)));
        let val = ast::Value::Array(t);

        assert_eq!(
            find_type_val(&val, &state, &funcs),
            ast::Type::Array(Box::new(ast::Type::I32), 2)
        )
    }

    #[test]
    fn find_type_val6() {
        let mut state: HashMap<ast::Variable, ast::Type> = HashMap::new();
        let funcs = Vec::new();

        state.insert(ast::Variable::Named(String::from("y")), ast::Type::Bool);

        let mut t = Vec::new();

        t.push(ast::Value::Expr(ast::Expr::Number(12)));
        t.push(ast::Value::Bool(ast::Bool::True));
        t.push(ast::Value::Variable(ast::Variable::Named(String::from(
            "y",
        ))));
        let val = ast::Value::Tuple(t);

        let mut types = Vec::new();
        types.push(ast::Type::I32);
        types.push(ast::Type::Bool);
        types.push(ast::Type::Bool);

        assert_eq!(find_type_val(&val, &state, &funcs), ast::Type::Tuple(types))
    }

    #[test]
    fn find_type_val7() {
        let mut state: HashMap<ast::Variable, ast::Type> = HashMap::new();
        let funcs = Vec::new();

        state.insert(
            ast::Variable::Named(String::from("y")),
            ast::Type::Reference(Box::new(ast::Type::Bool)),
        );
        let mut t = Vec::new();

        t.push(ast::Value::ReferenceMutable(Box::new(ast::Value::Expr(
            ast::Expr::Number(12),
        ))));
        t.push(ast::Value::Reference(Box::new(ast::Value::Bool(
            ast::Bool::True,
        ))));
        t.push(ast::Value::Variable(ast::Variable::Named(String::from(
            "y",
        ))));
        let val = ast::Value::Tuple(t);

        let mut types = Vec::new();
        types.push(ast::Type::ReferenceMutable(Box::new(ast::Type::I32)));
        types.push(ast::Type::Reference(Box::new(ast::Type::Bool)));
        types.push(ast::Type::Reference(Box::new(ast::Type::Bool)));

        assert_eq!(find_type_val(&val, &state, &funcs), ast::Type::Tuple(types))
    }

    #[test]
    fn find_type_val8() {
        let state: HashMap<ast::Variable, ast::Type> = HashMap::new();
        let funcs = Vec::new();

        let val = ast::Value::Dereference(Box::new(ast::Value::Reference(Box::new(
            ast::Value::Expr(ast::Expr::Number(12)),
        ))));

        assert_eq!(find_type_val(&val, &state, &funcs), ast::Type::I32)
    }

    #[test]
    fn find_type_val9() {
        let state: HashMap<ast::Variable, ast::Type> = HashMap::new();
        let mut funcs = Vec::new();

        let val = ast::Value::FunctionCall(String::from("test"), Vec::new());

        let f = ast::Function {
            name: String::from("test"),
            content: Vec::new(),
            input: Vec::new(),
            output: ast::Type::I32,
            precondition: ast::Bool::True,
            postcondition: ast::Bool::True,
            return_value: ast::Value::Unit,
        };

        funcs.push(f);

        assert_eq!(find_type_val(&val, &state, &funcs), ast::Type::I32)
    }

    #[test]
    fn simplify_function1() {
        let mut t = Vec::new();
        t.push(ast::Binding::Declaration(
            ast::Variable::Named(String::from("a")),
            ast::Type::I32,
            false,
        ));

        let mut content = Vec::new();
        content.push(ast::Command::Binding(ast::Binding::Assignment(
            ast::Variable::Named(String::from("x")),
            ast::Type::Unknown,
            ast::Value::Variable(ast::Variable::Named(String::from("a"))),
            false,
        )));

        let f = ast::Function {
            name: String::from("b"),
            content: content,
            input: t.clone(),
            output: ast::Type::Unit,
            precondition: ast::Bool::True,
            postcondition: ast::Bool::True,
            return_value: ast::Value::Unit,
        };

        let mut content2 = Vec::new();
        content2.push(ast::Command::Binding(ast::Binding::Assignment(
            ast::Variable::Named(String::from("x")),
            ast::Type::I32,
            ast::Value::Variable(ast::Variable::Named(String::from("a"))),
            false,
        )));

        let f2 = ast::Function {
            name: String::from("b"),
            content: content2,
            input: t.clone(),
            output: ast::Type::Unit,
            precondition: ast::Bool::True,
            postcondition: ast::Bool::True,
            return_value: ast::Value::Unit,
        };

        let funcs: &Vec<ast::Function> = &Vec::new();

        assert_eq!(simplify_function(f, &funcs), f2);
    }
}
