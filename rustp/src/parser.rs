extern crate nom;
use nom::IResult;

use crate::ast;
mod astp;
mod math;

pub fn expr(input: &str) -> IResult<&str, Box<ast::Expr>> {
    math::expr(input)
}

// testing stuff

use nom::{bytes::complete::tag, character::complete::space0};

//fn binding_assignment(input: &str) -> IResult<&str, ast::Binding::Assignment> {
//    tuple((space0, tag("let "), space0, primary_expr, space0))(input).and_then(|(next_input, x)| {
//        let (_, op, _, b, _) = x;
//        Ok((next_input, (op, b)))
//    })
//}
