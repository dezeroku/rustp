extern crate nom;
use nom::IResult;

use crate::ast;
mod astp;
mod math;

pub fn block(input: &str) -> IResult<&str, Vec<ast::Command>> {
    astp::block(input)
}

// testing stuff
use nom::{
    branch::alt, bytes::complete::tag, bytes::complete::take_while1, character::complete::char,
    character::complete::multispace0, character::complete::newline, character::complete::space0,
    character::complete::space1, combinator::opt, multi::many0, sequence::tuple,
};

pub fn function(input: &str) -> IResult<&str, ast::Function> {
    // TODO: handle args instead of space1
    astp::function_unit(input)
}
