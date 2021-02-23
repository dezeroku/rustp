use crate::ast;
use crate::parser::astp;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::space0, combinator::opt,
    sequence::tuple, IResult,
};

// concept: second answer to https://cs.stackexchange.com/questions/10558/grammar-for-describing-boolean-expressions-with-and-or-and-not

fn expr(input: &str) -> IResult<&str, Box<ast::Bool>> {
    tuple((space0, term, space0, opt(tuple((or, space0, term))), space0))(input).map(
        |(next_input, res)| {
            let (_, a, _, t, _) = res;
            match t {
                Some((_, _, b)) => (next_input, Box::new(ast::Bool::Or(a, b))),
                None => (next_input, a),
            }
        },
    )
}

#[test]
fn expr1() {
    assert!(expr("true").is_ok());
    assert!(expr("true || false").is_ok());
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
    println!("{:?}", expr("!false && (a && b) || (!c) && true"));
    assert!(expr("!false && (a && b) || (!c) && true").unwrap().0 == "");
}

fn term(input: &str) -> IResult<&str, Box<ast::Bool>> {
    tuple((
        space0,
        factor,
        space0,
        opt(tuple((and, space0, factor))),
        space0,
    ))(input)
    .map(|(next_input, res)| {
        let (_, a, _, t, _) = res;
        match t {
            Some((_, _, b)) => (next_input, Box::new(ast::Bool::And(a, b))),
            None => (next_input, a),
        }
    })
}

#[test]
fn term1() {
    assert!(term("true && false").is_ok());
    assert!(term("true").is_ok());
    assert!(
        term("false && true").unwrap().1
            == Box::new(ast::Bool::And(
                Box::new(ast::Bool::False),
                Box::new(ast::Bool::True)
            ))
    );
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
    tuple((space0, tag("("), space0, factor, space0, tag(")"), space0))(input).map(
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
    tag("true")(input).and_then(|(next_input, res)| Ok((next_input, Box::new(ast::Bool::True))))
}

#[test]
fn _true1() {
    assert!(_true("true").unwrap().0 == "");
    assert!(_true("false").is_err());
}

fn _false(input: &str) -> IResult<&str, Box<ast::Bool>> {
    tag("false")(input).and_then(|(next_input, res)| Ok((next_input, Box::new(ast::Bool::False))))
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
