extern crate nom;
use nom::IResult;

use crate::ast;
mod astp;
mod math;

pub fn expr(input: &str) -> IResult<&str, Box<ast::Expr>> {
    math::expr(input)
}

// testing stuff

use nom::{
    bytes::complete::tag, character::complete::char, character::complete::space0, sequence::tuple,
};

pub fn binding_assignment(input: &str) -> IResult<&str, ast::Binding> {
    tuple((
        space0,
        tag("let "),
        space0,
        astp::variable,
        space0,
        char('='),
        space0,
        math::expr,
        space0,
        char(';'),
    ))(input)
    .and_then(|(next_input, x)| {
        // TODO: type
        let (_, _, _, v, _, _, _, exp, _, _) = x;
        Ok((
            next_input,
            ast::Binding::Assignment(v, ast::Type::I32, ast::Value::Expr(*exp)),
        ))
    })
}

#[test]
fn binding_assignment1() {
    assert!(binding_assignment("let x = 12;").unwrap().0 == "");
    assert!(binding_assignment("let x = 12 * 4;").unwrap().0 == "");
    assert!(binding_assignment("let y = 12 - 5 * 6;").unwrap().0 == "");
    assert!(binding_assignment("let z = 3").is_err());
    assert!(
        binding_assignment("let z = 3;").unwrap().1
            == ast::Binding::Assignment(
                ast::Variable::Named("z".to_string()),
                ast::Type::I32,
                ast::Value::Expr(ast::Expr::Number(3))
            )
    );
}
