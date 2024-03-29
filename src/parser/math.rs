use crate::ast;

use crate::parser::astp;

use nom::{
    branch::alt, bytes::complete::tag, bytes::complete::take_while1, character::complete::char,
    character::complete::one_of, character::complete::space0, multi::many0, sequence::tuple,
    IResult,
};

use std::str::FromStr;

#[cfg(test)]
mod tests;

fn digito(input: &str) -> IResult<&str, &str> {
    take_while1(|a| char::is_digit(a, 10))(input)
}

// Concept: first answer to https://stackoverflow.com/questions/59508862/using-parser-combinator-to-parse-simple-math-expression

fn primary_expr(input: &str) -> IResult<&str, Box<ast::Expr>> {
    expr_number(input).or_else(|_| {
        expr_r_value(input).or_else(|_| {
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
    tuple((space0, mult_or_divide_or_mod, space0, primary_expr, space0))(input).and_then(
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

pub fn expr_val(input: &str) -> IResult<&str, ast::Value> {
    expr(input).and_then(|(next_input, res)| Ok((next_input, ast::Value::Expr(*res))))
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
    alt((_expr_number_negative, _expr_number_positive))(input)
}

fn _expr_number_positive(input: &str) -> IResult<&str, Box<ast::Expr>> {
    match number(input) {
        Ok(a) => {
            let (next_input, val) = a;
            Ok((next_input, Box::new(ast::Expr::Number(val))))
        }
        Err(e) => Err(e),
    }
}

fn _expr_number_negative(input: &str) -> IResult<&str, Box<ast::Expr>> {
    tuple((tag("-"), number))(input)
        .and_then(|(next_input, (_, val))| Ok((next_input, Box::new(ast::Expr::Number(-val)))))
}

fn expr_r_value(input: &str) -> IResult<&str, Box<ast::Expr>> {
    alt((astp::function_call, astp::variable_val))(input)
        .and_then(|(next_input, res)| Ok((next_input, Box::new(ast::Expr::Value(Box::new(res))))))
}

fn mult_or_divide_or_mod(input: &str) -> IResult<&str, ast::Opcode> {
    let t = one_of("*/%")(input);
    match t {
        Ok(a) => {
            let (next_input, res) = a;
            Ok((
                next_input,
                match res {
                    '*' => ast::Opcode::Mul,
                    '/' => ast::Opcode::Div,
                    '%' => ast::Opcode::Rem,
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

pub fn number(input: &str) -> IResult<&str, i32> {
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
