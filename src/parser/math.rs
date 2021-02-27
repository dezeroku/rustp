use crate::ast;

use crate::parser::astp;

use nom::{
    bytes::complete::take_while1, character::complete::char, character::complete::newline,
    character::complete::one_of, character::complete::space0, multi::many0, sequence::tuple,
    IResult,
};

use std::str::FromStr;

fn digito(input: &str) -> IResult<&str, &str> {
    take_while1(|a| char::is_digit(a, 10))(input)
}

fn multiple_expr(input: &str) -> IResult<&str, Vec<ast::Expr>> {
    let mut result = Vec::new();
    expr(input).and_then(|(next_input, res)| {
        result.push(*res);
        many0(tuple((newline, expr)))(next_input).map(|(next_input, res_vec)| {
            for item in res_vec {
                let (_, val) = item;
                result.push(*val);
            }
            (next_input, result)
        })
    })
}

// Concept: first answer to https://stackoverflow.com/questions/59508862/using-parser-combinator-to-parse-simple-math-expression

fn primary_expr(input: &str) -> IResult<&str, Box<ast::Expr>> {
    expr_number(input).or_else(|_| {
        expr_variable(input).or_else(|_| {
            char('(')(input)
                .and_then(|(next_input, _)| space0(next_input))
                .and_then(|(next_input, _)| expr(next_input))
                .and_then(|(next_input, res)| {
                    space0(next_input)
                        .and_then(|(next_input, _)| char(')')(next_input))
                        .and_then(|(next_input, _)| space0(next_input))
                        .and_then(|(next_input, _)| Ok((next_input, res)))
                })
        })
    })
}

fn mult_expr_right(input: &str) -> IResult<&str, (ast::Opcode, Box<ast::Expr>)> {
    tuple((space0, mult_or_divide, space0, primary_expr, space0))(input).and_then(
        |(next_input, x)| {
            let (_, op, _, b, _) = x;
            Ok((next_input, (op, b)))
        },
    )
}

fn mult_expr(input: &str) -> IResult<&str, Box<ast::Expr>> {
    space0(input)
        .and_then(|(next_input, _)| primary_expr(next_input))
        .and_then(|(next_input, a)| {
            let f = many0(mult_expr_right)(next_input);
            match f {
                Ok(x) => {
                    let (next_input, vect) = x;
                    let mut temp = a;
                    for item in vect {
                        let (op, b) = item;
                        temp = Box::new(ast::Expr::Op(temp, op, b));
                    }
                    Ok((next_input, temp))
                }
                Err(_) => Ok((next_input, a)),
            }
        })
}

fn add_expr_right(input: &str) -> IResult<&str, (ast::Opcode, Box<ast::Expr>)> {
    tuple((space0, add_or_subtract, space0, mult_expr, space0))(input).and_then(
        |(next_input, x)| {
            let (_, op, _, b, _) = x;
            Ok((next_input, (op, b)))
        },
    )
}

pub fn expr(input: &str) -> IResult<&str, Box<ast::Expr>> {
    space0(input)
        .and_then(|(next_input, _)| mult_expr(next_input))
        .and_then(|(next_input, a)| {
            let f = many0(add_expr_right)(next_input);
            match f {
                Ok(x) => {
                    let (next_input, vect) = x;
                    let mut temp = a;
                    for item in vect {
                        let (op, b) = item;
                        temp = Box::new(ast::Expr::Op(temp, op, b));
                    }
                    Ok((next_input, temp))
                }
                Err(_) => Ok((next_input, a)),
            }
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

fn expr_variable(input: &str) -> IResult<&str, Box<ast::Expr>> {
    astp::variable(input)
        .and_then(|(next_input, res)| Ok((next_input, Box::new(ast::Expr::Variable(res)))))
}

fn mult_or_divide(input: &str) -> IResult<&str, ast::Opcode> {
    let t = one_of("*/")(input);
    match t {
        Ok(a) => {
            let (next_input, res) = a;
            Ok((
                next_input,
                match res {
                    '*' => ast::Opcode::Mul,
                    '/' => ast::Opcode::Div,
                    _ => unimplemented!(),
                },
            ))
        }
        Err(a) => Err(a),
    }
}

fn add_or_subtract(input: &str) -> IResult<&str, ast::Opcode> {
    let t = one_of("+-")(input);
    match t {
        Ok(a) => {
            let (next_input, res) = a;
            Ok((
                next_input,
                match res {
                    '+' => ast::Opcode::Add,
                    '-' => ast::Opcode::Sub,
                    _ => unimplemented!(),
                },
            ))
        }
        Err(a) => Err(a),
    }
}

#[test]
fn multiple_expr1() {
    assert!(multiple_expr("13 + 3\n 12 * 3").is_ok());
    assert!(multiple_expr("13 + 3\n 12 * 3").unwrap().0 == "");
}

#[test]
fn mult_expr_right1() {
    assert!(mult_expr_right("13").is_err());
    assert!(mult_expr_right("*13").is_ok());
    assert!(
        mult_expr_right("/13") == Ok(("", (ast::Opcode::Div, Box::new(ast::Expr::Number(13)))))
    );
}

#[test]
fn add_expr_right1() {
    assert!(add_expr_right("13").is_err());
    assert!(add_expr_right("-13").is_ok());
    assert!(add_expr_right("+13") == Ok(("", (ast::Opcode::Add, Box::new(ast::Expr::Number(13))))));
}

#[test]
fn mult_expr1() {
    assert!(
        mult_expr("13 * 4")
            == Ok((
                "",
                Box::new(ast::Expr::Op(
                    Box::new(ast::Expr::Number(13)),
                    ast::Opcode::Mul,
                    Box::new(ast::Expr::Number(4))
                ))
            ))
    );
}

#[test]
fn mult_expr2() {
    assert!(
        mult_expr(" 13 * 4")
            == Ok((
                "",
                Box::new(ast::Expr::Op(
                    Box::new(ast::Expr::Number(13)),
                    ast::Opcode::Mul,
                    Box::new(ast::Expr::Number(4))
                ))
            ))
    );
}

#[test]
fn mult_expr3() {
    assert!(
        mult_expr("13 * 4 ")
            == Ok((
                "",
                Box::new(ast::Expr::Op(
                    Box::new(ast::Expr::Number(13)),
                    ast::Opcode::Mul,
                    Box::new(ast::Expr::Number(4))
                ))
            ))
    );
}

#[test]
fn primary_expr1() {
    assert!(primary_expr("1").is_ok());
    assert!(primary_expr("3").is_ok());
    assert!(primary_expr("13") == Ok(("", Box::new(ast::Expr::Number(13)))));
    assert!(primary_expr("(3)").unwrap().0 == "");
    assert!(primary_expr("").is_err());
}

#[test]
fn primary_expr2() {
    assert!(primary_expr("1 + 1").is_ok());
    assert!(primary_expr("3 - 1").is_ok());
    assert!(primary_expr("1 / 1").is_ok());
    assert!(primary_expr("1 /") == Ok((" /", Box::new(ast::Expr::Number(1)))));
    assert!(
        *expr("12 * 3").unwrap().1
            == *Box::new(ast::Expr::Op(
                Box::new(ast::Expr::Number(12)),
                ast::Opcode::Mul,
                Box::new(ast::Expr::Number(3))
            ))
    );
}

#[test]
fn primary_expr3() {
    assert!(primary_expr("1 - 1 + 1").is_ok());
    assert!(primary_expr("1 * 2 -3").is_ok());
    assert!(primary_expr("1 + 1 - 3").is_ok());
}

#[test]
fn expr1() {
    assert!(expr("1 + 1").is_ok());
    assert!(expr("13 - 4").is_ok());
    assert!(expr("13-4").is_ok());
    assert!(expr("13 / ").unwrap().0 != "");
}

#[test]
fn expr2() {
    assert!(
        *expr("12 * 3").unwrap().1
            == *Box::new(ast::Expr::Op(
                Box::new(ast::Expr::Number(12)),
                ast::Opcode::Mul,
                Box::new(ast::Expr::Number(3))
            ))
    );
}

#[test]
fn expr3() {
    assert!(expr("13 * 4 + 2 / 4 - 8").unwrap().0 == "");
}

#[test]
fn expr4() {
    assert!(
        expr("1 + x").unwrap().1
            == Box::new(ast::Expr::Op(
                Box::new(ast::Expr::Number(1)),
                ast::Opcode::Add,
                Box::new(ast::Expr::Variable(ast::Variable::Named("x".to_string())))
            ))
    );
}

#[test]
fn expr_number1() {
    assert!(expr_number("1").is_ok());
    assert!(*expr_number("13").unwrap().1 == *Box::new(ast::Expr::Number(13)));
}

#[test]
fn mult_or_divide1() {
    assert!(mult_or_divide("*").is_ok());
    assert!(mult_or_divide("/").is_ok());
    assert!(mult_or_divide("+").is_err());
}

#[test]
fn add_or_subtract1() {
    assert!(add_or_subtract("+").is_ok());
    assert!(add_or_subtract("-").is_ok());
    assert!(add_or_subtract("/").is_err());
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

#[test]
fn number1() {
    assert!(number("1").is_ok());
    assert!(number("12").is_ok());
    assert!(number("194").is_ok());
    assert!(number("").is_err());
}