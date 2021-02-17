extern crate nom;

use crate::ast;

use nom::{
    branch::alt, bytes::complete::take_while1, character::complete::digit1,
    character::complete::one_of, character::complete::space0, character::is_digit,
    combinator::map_res, combinator::opt, map, named, sequence::tuple, take_while, IResult,
};
use std::str::FromStr;

fn digito(input: &str) -> IResult<&str, &str> {
    take_while1(|a| char::is_digit(a, 10))(input)
}

//fn expr(input: &str) -> IResult<&str, ast::Expr> {
//    match alt((expr, expr_number))(input) {
//        Ok(a) => {let (input_next, res) = a;
//        Err(e) => Err(e)
//    }
//    match t
//}

pub fn expr_expr(input: &str) -> IResult<&str, Box<ast::Expr>> {
    tuple((expr_number, space0, opcode, space0, expr_number))(input).map(|(next_input, res)| {
        let (a, _, op, _, b) = res;
        (next_input, Box::new(ast::Expr::Op(a, op, b)))
    })
}

fn expr_number(input: &str) -> IResult<&str, Box<ast::Expr>> {
    match number(input) {
        Ok(a) => {
            let (next_input, val) = a;
            Ok((next_input, Box::new(ast::Expr::Number(val))))
        }
        Err(e) => Err(e),
    }
}

fn opcode(input: &str) -> IResult<&str, ast::Opcode> {
    let t = one_of("*+-/")(input);
    match t {
        Ok(a) => {
            let (next_input, res) = a;
            Ok((
                next_input,
                match res {
                    '*' => ast::Opcode::Mul,
                    '+' => ast::Opcode::Add,
                    '-' => ast::Opcode::Sub,
                    '/' => ast::Opcode::Div,
                    _ => unimplemented!(),
                },
            ))
        }
        Err(a) => Err(a),
    }
}

#[test]
fn expr_expr1() {
    assert!(expr_expr("1 + 1").is_ok());
    assert!(expr_expr("13 - 4").is_ok());
    assert!(expr_expr("13-4").is_ok());
    assert!(expr_expr("13 / ").is_err());
}

#[test]
fn expr_expr2() {
    assert!(
        *expr_expr("12 * 3").unwrap().1
            == *Box::new(ast::Expr::Op(
                Box::new(ast::Expr::Number(12)),
                ast::Opcode::Mul,
                Box::new(ast::Expr::Number(3))
            ))
    );
}

#[test]
fn expr_number1() {
    assert!(expr_number("1").is_ok());
    assert!(*expr_number("13").unwrap().1 == *Box::new(ast::Expr::Number(13)));
}

#[test]
fn opcode1() {
    assert!(opcode("*").is_ok());
    assert!(opcode("-").is_ok());
    assert!(opcode("/").is_ok());
    assert!(opcode("+").unwrap().1 == ast::Opcode::Add);
    assert!(opcode("8").is_err());
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
