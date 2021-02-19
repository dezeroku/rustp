extern crate nom;
use nom::IResult;

use crate::ast;
mod math;

pub fn expr(input: &str) -> IResult<&str, Box<ast::Expr>> {
    math::expr(input)
}
