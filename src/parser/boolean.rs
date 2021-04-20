use crate::ast;
use crate::parser::astp;
use crate::parser::math;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::space0, character::complete::space1,
    multi::many0, sequence::tuple, IResult,
};

#[cfg(test)]
mod tests;

// concept: second answer to https://cs.stackexchange.com/questions/10558/grammar-for-describing-boolean-expressions-with-and-or-and-not
// concept: first answer to https://stackoverflow.com/questions/59508862/using-parser-combinator-to-parse-simple-math-expression
// addition is basically || and multiplication is &&

fn mult_expr_right(input: &str) -> IResult<&str, Box<ast::Bool>> {
    tuple((space0, and, space0, factor, space0))(input).and_then(|(next_input, x)| {
        let (_, _, _, b, _) = x;
        Ok((next_input, b))
    })
}

fn mult_expr(input: &str) -> IResult<&str, Box<ast::Bool>> {
    space0(input)
        .and_then(|(next_input, _)| factor(next_input))
        .and_then(|(next_input, a)| {
            let f = many0(mult_expr_right)(next_input);
            match f {
                Ok(x) => {
                    let (next_input, vect) = x;
                    let mut temp = a;
                    for item in vect {
                        let b = item;
                        temp = Box::new(ast::Bool::And(temp, b));
                    }
                    Ok((next_input, temp))
                }
                Err(_) => Ok((next_input, a)),
            }
        })
}

fn add_expr_right(input: &str) -> IResult<&str, Box<ast::Bool>> {
    tuple((space0, or, space0, mult_expr, space0))(input).and_then(|(next_input, x)| {
        let (_, _, _, b, _) = x;
        Ok((next_input, b))
    })
}

pub fn expr_val(input: &str) -> IResult<&str, ast::Value> {
    expr(input).and_then(|(next_input, res)| Ok((next_input, ast::Value::Bool(*res))))
}

pub fn expr(input: &str) -> IResult<&str, Box<ast::Bool>> {
    space0(input)
        .and_then(|(next_input, _)| mult_expr(next_input))
        .and_then(|(next_input, a)| {
            let f = many0(add_expr_right)(next_input);
            match f {
                Ok(x) => {
                    let (next_input, vect) = x;
                    let mut temp = a;
                    for item in vect {
                        let b = item;
                        temp = Box::new(ast::Bool::Or(temp, b));
                    }
                    Ok((next_input, temp))
                }
                Err(_) => Ok((next_input, a)),
            }
        })
}

fn factor(input: &str) -> IResult<&str, Box<ast::Bool>> {
    alt((
        forall,
        exists,
        factor_compare,
        factor_id,
        factor_not,
        factor_paren,
    ))(input)
}

fn expr_r_value(input: &str) -> IResult<&str, Box<ast::Bool>> {
    alt((astp::function_call, astp::variable_val))(input)
        .and_then(|(next_input, res)| Ok((next_input, Box::new(ast::Bool::Value(Box::new(res))))))
}

fn factor_id(input: &str) -> IResult<&str, Box<ast::Bool>> {
    alt((_true, _false, expr_r_value))(input)
}

fn factor_not(input: &str) -> IResult<&str, Box<ast::Bool>> {
    tuple((not, space0, factor))(input).map(|(next_input, res)| {
        let (_, _, a) = res;
        (next_input, Box::new(ast::Bool::Not(a)))
    })
}

fn factor_compare(input: &str) -> IResult<&str, Box<ast::Bool>> {
    alt((
        factor_compare_equal,
        factor_compare_not_equal,
        factor_compare_greater_equal,
        factor_compare_greater,
        factor_compare_smaller_equal,
        factor_compare_smaller,
    ))(input)
}

fn factor_compare_equal(input: &str) -> IResult<&str, Box<ast::Bool>> {
    tuple((math::expr, space0, tag("=="), space0, math::expr))(input).and_then(
        |(next_input, res)| {
            let (a, _, _, _, b) = res;
            Ok((next_input, Box::new(ast::Bool::Equal(*a, *b))))
        },
    )
}

fn factor_compare_not_equal(input: &str) -> IResult<&str, Box<ast::Bool>> {
    tuple((math::expr, space0, tag("!="), space0, math::expr))(input).and_then(
        |(next_input, res)| {
            let (a, _, _, _, b) = res;
            Ok((
                next_input,
                Box::new(ast::Bool::Not(Box::new(ast::Bool::Equal(*a, *b)))),
            ))
        },
    )
}

fn factor_compare_greater_equal(input: &str) -> IResult<&str, Box<ast::Bool>> {
    tuple((math::expr, space0, tag(">="), space0, math::expr))(input).and_then(
        |(next_input, res)| {
            let (a, _, _, _, b) = res;
            Ok((next_input, Box::new(ast::Bool::GreaterEqual(*a, *b))))
        },
    )
}
fn factor_compare_greater(input: &str) -> IResult<&str, Box<ast::Bool>> {
    tuple((math::expr, space0, tag(">"), space0, math::expr))(input).and_then(
        |(next_input, res)| {
            let (a, _, _, _, b) = res;
            Ok((next_input, Box::new(ast::Bool::GreaterThan(*a, *b))))
        },
    )
}

fn factor_compare_smaller_equal(input: &str) -> IResult<&str, Box<ast::Bool>> {
    tuple((math::expr, space0, tag("<="), space0, math::expr))(input).and_then(
        |(next_input, res)| {
            let (a, _, _, _, b) = res;
            Ok((next_input, Box::new(ast::Bool::LowerEqual(*a, *b))))
        },
    )
}
fn factor_compare_smaller(input: &str) -> IResult<&str, Box<ast::Bool>> {
    tuple((math::expr, space0, tag("<"), space0, math::expr))(input).and_then(
        |(next_input, res)| {
            let (a, _, _, _, b) = res;
            Ok((next_input, Box::new(ast::Bool::LowerThan(*a, *b))))
        },
    )
}

fn factor_paren(input: &str) -> IResult<&str, Box<ast::Bool>> {
    tuple((space0, tag("("), space0, expr, space0, tag(")"), space0))(input).map(
        |(next_input, res)| {
            let (_, _, _, a, _, _, _) = res;
            (next_input, a)
        },
    )
}

fn _true(input: &str) -> IResult<&str, Box<ast::Bool>> {
    tag("true")(input).and_then(|(next_input, _)| Ok((next_input, Box::new(ast::Bool::True))))
}

fn _false(input: &str) -> IResult<&str, Box<ast::Bool>> {
    tag("false")(input).and_then(|(next_input, _)| Ok((next_input, Box::new(ast::Bool::False))))
}

fn and(input: &str) -> IResult<&str, &str> {
    tag("&&")(input)
}

fn or(input: &str) -> IResult<&str, &str> {
    tag("||")(input)
}

fn not(input: &str) -> IResult<&str, &str> {
    tag("!")(input)
}

pub fn forall(input: &str) -> IResult<&str, Box<ast::Bool>> {
    tuple((tag("forall"), space1, astp::variable_single, space1, expr))(input).map(
        |(next_input, res)| {
            let (_, _, v, _, b) = res;
            (next_input, Box::new(ast::Bool::ForAll(v, b)))
        },
    )
}

pub fn exists(input: &str) -> IResult<&str, Box<ast::Bool>> {
    tuple((tag("exists"), space1, astp::variable_single, space1, expr))(input).map(
        |(next_input, res)| {
            let (_, _, v, _, b) = res;
            (next_input, Box::new(ast::Bool::Exists(v, b)))
        },
    )
}
