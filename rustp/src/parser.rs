extern crate nom;

use crate::ast;

use nom::{
    bytes::complete::take_while1, character::complete::digit1, character::is_digit,
    combinator::map_res, map, named, sequence::tuple, take_while, IResult,
};
use std::num::ParseIntError;
use std::str::FromStr;

fn digito(input: &str) -> IResult<&str, &str> {
    take_while1(|a| char::is_digit(a, 10))(input)
}

fn expr_number(input: &str) -> IResult<&str, ast::Expr> {
    match number(input) {
        Ok(a) => {
            let (next_input, val) = a;
            Ok((next_input, ast::Expr::Number(val)))
        }
        Err(e) => Err(e),
    }
}

fn number(input: &str) -> IResult<&str, i32> {
    let t = digito(input);
    match t {
        Ok(a) => {
            let (next_input, res) = a;
            // this unwrap may be a dirty hack :D
            // on the other hand this is only used when we are sure, that what we are converting is a number
            // so it may be safe?
            Ok((next_input, FromStr::from_str(res).unwrap()))
        }
        Err(a) => Err(a),
    }
}

//fn addition(input: &str) -> IResult<&str, ast::Expr> {
//    let t = tuple((number, number))(input).map(|(next_input, res)| {
//        let (a, b) = res;
//        (next_input, ast::Expr(a, b))
//    });
//}

#[test]
fn number1() {
    assert!(number("1").is_ok());
    assert!(number("12").is_ok());
    assert!(number("194").is_ok());
    assert!(number("").is_err());
}

pub fn testo() {
    println!("XD");
    println!("{}", number("194 a").unwrap().1);
}
