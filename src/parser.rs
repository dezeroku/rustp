extern crate nom;
use nom::IResult;

use crate::ast;
mod astp;
mod boolean;
mod math;

pub fn program(input: &str) -> IResult<&str, ast::Program> {
    astp::program(input)
}
