extern crate nom;
use nom::IResult;

use crate::ast;
mod astp;
mod boolean;
mod math;

pub fn function(input: &str) -> IResult<&str, ast::Function> {
    astp::function(input)
}
