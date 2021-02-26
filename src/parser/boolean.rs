use crate::ast;
use crate::parser::astp;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::space0, combinator::opt, multi::many0,
    sequence::tuple, IResult,
};

// concept: second answer to https://cs.stackexchange.com/questions/10558/grammar-for-describing-boolean-expressions-with-and-or-and-not
// concept: first answer to https://stackoverflow.com/questions/59508862/using-parser-combinator-to-parse-simple-math-expression
// addition is basically || and multiplication is &&

fn mult_expr_right(input: &str) -> IResult<&str, Box<ast::Bool>> {
    tuple((space0, and, space0, factor, space0))(input).and_then(|(next_input, x)| {
        let (_, _, _, b, _) = x;
        Ok((next_input, b))
    })
}

#[test]
fn mult_expr_right1() {
    assert!(mult_expr_right("&& true").unwrap().0 == "");
}

#[test]
fn mult_expr_right2() {
    assert!(mult_expr_right(" && true").unwrap().0 == "");
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

#[test]
fn expr1() {
    assert!(expr("true").is_ok());
    assert!(expr("true || false").is_ok());
    assert!(expr("true || false || true").unwrap().0 == "");
    assert!(
        expr("false || true").unwrap().1
            == Box::new(ast::Bool::Or(
                Box::new(ast::Bool::False),
                Box::new(ast::Bool::True)
            ))
    );
}

#[test]
fn expr2() {
    assert!(expr("!false && (a && b) || (!c) && true").unwrap().0 == "");
}

fn factor(input: &str) -> IResult<&str, Box<ast::Bool>> {
    alt((factor_id, factor_not, factor_paren))(input)
}

fn variable(input: &str) -> IResult<&str, Box<ast::Bool>> {
    astp::variable(input)
        .and_then(|(next_input, res)| Ok((next_input, Box::new(ast::Bool::Variable(res)))))
}

fn factor_id(input: &str) -> IResult<&str, Box<ast::Bool>> {
    alt((_true, _false, variable))(input)
}

#[test]
fn factor_id_1() {
    assert!(factor_id("true").unwrap().1 == Box::new(ast::Bool::True));
    assert!(factor_id("false").unwrap().1 == Box::new(ast::Bool::False));
}

fn factor_not(input: &str) -> IResult<&str, Box<ast::Bool>> {
    tuple((not, space0, factor))(input).map(|(next_input, res)| {
        let (_, _, a) = res;
        (next_input, Box::new(ast::Bool::Not(a)))
    })
}

#[test]
fn factor_not_1() {
    assert!(factor_not("!true").unwrap().1 == Box::new(ast::Bool::Not(Box::new(ast::Bool::True))));
}

fn factor_paren(input: &str) -> IResult<&str, Box<ast::Bool>> {
    tuple((space0, tag("("), space0, expr, space0, tag(")"), space0))(input).map(
        |(next_input, res)| {
            let (_, _, _, a, _, _, _) = res;
            (next_input, a)
        },
    )
}

#[test]
fn factor_paren_1() {
    assert!(
        factor_paren("(!true)").unwrap().1 == Box::new(ast::Bool::Not(Box::new(ast::Bool::True)))
    );
    assert!(
        factor_paren("(!a)").unwrap().1
            == Box::new(ast::Bool::Not(Box::new(ast::Bool::Variable(
                ast::Variable::Named("a".to_string())
            ))))
    );
}

fn _true(input: &str) -> IResult<&str, Box<ast::Bool>> {
    tag("true")(input).and_then(|(next_input, _)| Ok((next_input, Box::new(ast::Bool::True))))
}

#[test]
fn _true1() {
    assert!(_true("true").unwrap().0 == "");
    assert!(_true("false").is_err());
}

fn _false(input: &str) -> IResult<&str, Box<ast::Bool>> {
    tag("false")(input).and_then(|(next_input, _)| Ok((next_input, Box::new(ast::Bool::False))))
}

#[test]
fn _false1() {
    assert!(_false("false").unwrap().0 == "");
    assert!(_false("true").is_err());
}

fn and(input: &str) -> IResult<&str, &str> {
    tag("&&")(input)
}

#[test]
fn and1() {
    assert!(and("&&").unwrap().0 == "");
    assert!(and("&").is_err());
}

fn or(input: &str) -> IResult<&str, &str> {
    tag("||")(input)
}

#[test]
fn or1() {
    assert!(or("||").unwrap().0 == "");
    assert!(or("|").is_err());
}

fn not(input: &str) -> IResult<&str, &str> {
    tag("!")(input)
}

#[test]
fn not1() {
    assert!(not("!").unwrap().0 == "");
    assert!(not("?").is_err());
}
