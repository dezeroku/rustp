use crate::ast;

/// Infer types for all the bindings based on types of the r_value
pub fn simplify(program: ast::Program) -> ast::Program {
    // Find the bindings, which use ast::Type::Unknown
    // For these bindings, find a context (all the following commands, until the rebinding of same name)
    // Based on the context, find assignments and infer type based on r_value
    let mut result = Vec::new();
    for func in program.content {
        result.push(simplify_function(func));
    }

    ast::Program { content: result }
}

fn simplify_function(function: ast::Function) -> ast::Function {
    let to_check = unknown_type_bindings(function.content.iter().collect());
    function
}

fn unknown_type_single(input: &ast::Command) -> Option<&ast::Command> {
    match input {
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
                let temp: Vec<&ast::Command> = binds
                    .iter()
                    .map(|x| unknown_type_single(x))
                    .flatten()
                    .collect();
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

fn unknown_type_bindings(input: Vec<&ast::Command>) -> Vec<&ast::Command> {
    let mut to_return = Vec::new();
    for comm in input {
        match comm {
            ast::Command::Block(a) => {
                match a {
                    // TODO: handle this
                    _ => unimplemented!(), //ast::Block::If(_, comms, el) => {
                                           //    let t = comms.clone();
                                           //    t.push(el.clone());
                                           //    let t: usize = t
                                           //        .iter()
                                           //        .map(|x| unknown_type_bindings(x.clone().iter().collect()))
                                           //        .collect::<Vec<Vec<&ast::Command>>>()
                                           //        .iter()
                                           //        .map(|x| x.len())
                                           //        .collect::<Vec<usize>>()
                                           //        .iter()
                                           //        .sum();
                                           //    if t > 0 {
                                           //        to_return.push(comm);
                                           //    }
                                           //}
                                           //ast::Block::ForRange(_, _, _, comms) => {}
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
            unknown_type_single(&ast::Command::Binding(ast::Binding::Assignment(
                ast::Variable::Named(String::from("b")),
                ast::Type::Unknown,
                ast::Value::Expr(ast::Expr::Number(1)),
                false
            ))),
            Some(&ast::Command::Binding(ast::Binding::Assignment(
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
            unknown_type_single(&ast::Command::Binding(ast::Binding::Assignment(
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
            unknown_type_single(&ast::Command::Binding(ast::Binding::Tuple(temp.clone()))),
            Some(&ast::Command::Binding(ast::Binding::Tuple(temp)))
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
            unknown_type_single(&ast::Command::Binding(ast::Binding::Tuple(temp.clone()))),
            Some(&ast::Command::Binding(ast::Binding::Tuple(temp)))
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
            unknown_type_single(&ast::Command::Binding(ast::Binding::Tuple(temp))),
            None
        );
    }
}
