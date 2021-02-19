extern crate nom;
use nom::IResult;

use crate::ast;
mod astp;
mod math;

pub fn command(input: &str) -> IResult<&str, ast::Command> {
    astp::command(input)
}

// testing stuff
use nom::{
    branch::alt, bytes::complete::tag, bytes::complete::take_while1, character::complete::char,
    character::complete::newline, character::complete::space0, combinator::opt, sequence::tuple,
};
