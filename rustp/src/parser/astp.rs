use crate::ast;
use nom::{bytes::complete::take_while1, IResult};

pub fn variable(input: &str) -> IResult<&str, ast::Variable> {
    let l = |x: char| char::is_alphabetic(x) || '_' == x;

    take_while1(l)(input)
        .and_then(|(next_input, res)| Ok((next_input, ast::Variable::Named(res.to_string()))))
}

#[test]
fn variable1() {
    assert!(variable("1").is_err());
    assert!(variable("a").is_ok());
    assert!(variable("abc").unwrap().0 == "");
    assert!(variable("abc").unwrap().1 == ast::Variable::Named("abc".to_string()));
}

#[test]
fn variable2() {
    assert!(variable("_a_b").is_ok());
    assert!(variable("_a_b_c_").unwrap().0 == "");
    assert!(variable("_a_b_c_").unwrap().1 == ast::Variable::Named("_a_b_c_".to_string()));
}
