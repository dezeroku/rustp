use crate::ast;
use crate::parser::math;

use nom::{
    branch::alt, bytes::complete::tag, bytes::complete::take_while1, character::complete::char,
    character::complete::multispace0, character::complete::newline, character::complete::space0,
    character::complete::space1, combinator::opt, multi::many0, sequence::tuple, IResult,
};

pub fn function_unit(input: &str) -> IResult<&str, ast::Function> {
    tuple((
        tag("fn"),
        space1,
        function_name,
        space1,
        tag("("),
        space0,
        tag(")"),
        space0,
        tag("{"),
        space0,
        block,
        space0,
        tag("}"),
    ))(input)
    .and_then(|(next_input, res)| {
        let (_, _, name, _, _, _, _, _, _, _, comms, _, _) = res;
        Ok((
            next_input,
            ast::Function {
                name: name.to_string(),
                content: comms,
                input: Vec::new(),
                output: ast::Type::Unit,
            },
        ))
    })
}

#[test]
fn function_unit1() {
    assert!(function_unit("fn a () {}").unwrap().0 == "");
    assert!(function_unit("fn a ( ) {}").unwrap().0 == "");
    assert!(function_unit("fn a ( ) { }").unwrap().0 == "");
    assert!(function_unit("fn  a  ( ) { }").unwrap().0 == "");
    assert!(function_unit("fn a () {let a = 14;}").unwrap().0 == "");
    assert!(
        function_unit("fn a () {let a = 14; let c = 1 + b;}")
            .unwrap()
            .0
            == ""
    );
    let mut content = Vec::new();
    content.push(ast::Command::Binding(ast::Binding::Assignment(
        ast::Variable::Named("a".to_string()),
        ast::Type::I32,
        ast::Value::Expr(ast::Expr::Number(14)),
    )));
    let a = function_unit("fn a () {let a = 14;}").unwrap().1;

    let b = ast::Function {
        name: "a".to_string(),
        content: content,
        input: Vec::new(),
        output: ast::Type::Unit,
    };

    assert!(a == b);
}

pub fn block(input: &str) -> IResult<&str, Vec<ast::Command>> {
    many0(tuple((multispace0, command, multispace0)))(input).and_then(|(next_input, res)| {
        let mut result = Vec::new();
        for i in res {
            let (_, temp, _) = i;
            result.push(temp)
        }
        Ok((next_input, result))
    })
}

#[test]
fn block1() {
    assert!(block("let x = 1;").unwrap().0 == "");
    assert!(block("let x = 1;//%assert 143").unwrap().0 == "");
}

fn command(input: &str) -> IResult<&str, ast::Command> {
    alt((binding, assert))(input)
}

#[test]
fn command1() {
    assert!(command("//%assert 143\n").unwrap().0 == "");
}

#[test]
fn command2() {
    assert!(command("let a = 14;").unwrap().0 == "");
}

fn take_while_not_newline(input: &str) -> IResult<&str, &str> {
    take_while1(|x| x != '\n')(input)
}

#[test]
fn take_while_not_newline1() {
    assert!(take_while_not_newline("ababab\n").unwrap().0 == "\n");
    assert!(take_while_not_newline("ababab").unwrap().0 == "");
    assert!(take_while_not_newline("ababab").unwrap().1 == "ababab");
}

fn assert(input: &str) -> IResult<&str, ast::Command> {
    tuple((
        prove_start,
        tag("assert"),
        take_while_not_newline,
        opt(newline),
    ))(input)
    .map(|(next_input, res)| {
        let (_, _, a, _) = res;
        (
            next_input,
            ast::Command::ProveControl(ast::ProveControl::Assert(a.to_string())),
        )
    })
    //prove_start(input).and_then(|(next_input, _)| {
    //    tag("assert")(next_input).and_then(|(next_input, _)| {
    //        // If t fails, there's no newline, get whole line as assert
    //        // If t is ok, there's a newline somewhere, get only parsed content as assert.
    //        let t = take_while_not_newline(next_input).unwrap();
    //        // TODO: get the last newline if exists
    //        Ok((t.0, ast::ProveControl::Assert(t.1.to_string())))
    //        //match t {
    //        //    Ok((next_input, res)) => {
    //        //        //let next_input = newline(next_input).unwrap().0;
    //        //        // TODO: result the newline that's left
    //        //        Ok((next_input, ast::ProveControl::Assert(res.to_string())))
    //        //    }
    //        //    Err(a) => Ok(("", ast::ProveControl::Assert(next_input.to_string()))),
    //        //}
    //    })
    //})
}

#[test]
fn assert1() {
    assert!(assert("//%assert 143").is_ok());
}

#[test]
fn assert2() {
    assert!(assert("//%assert 143").unwrap().0 == "");
}

#[test]
fn assert3() {
    assert!(assert("//%assert 143\n").unwrap().0 == "");
}

fn prove_start(input: &str) -> IResult<&str, &str> {
    tag("//%")(input)
}

#[test]
fn prove_start1() {
    assert!(prove_start("//%").unwrap().0 == "");
    assert!(prove_start("/%").is_err());
}

pub fn function_name(input: &str) -> IResult<&str, &str> {
    let l = |x: char| char::is_alphabetic(x) || '_' == x;

    take_while1(l)(input).and_then(|(next_input, res)| Ok((next_input, res)))
}

fn variable_name(input: &str) -> IResult<&str, &str> {
    let l = |x: char| char::is_alphabetic(x) || '_' == x;

    take_while1(l)(input).and_then(|(next_input, res)| Ok((next_input, res)))
}

// TODO:
// probably this has to be split to r_variable, so function calls also could be used on right
// sight of the assignment
pub fn variable(input: &str) -> IResult<&str, ast::Variable> {
    variable_name(input)
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

fn binding(input: &str) -> IResult<&str, ast::Command> {
    // TODO: binding without assignment case
    binding_assignment(input)
}

fn binding_assignment(input: &str) -> IResult<&str, ast::Command> {
    tuple((
        space0,
        tag("let "),
        space0,
        variable,
        space0,
        char('='),
        space0,
        math::expr,
        space0,
        char(';'),
    ))(input)
    .and_then(|(next_input, x)| {
        // TODO: type
        let (_, _, _, v, _, _, _, exp, _, _) = x;
        Ok((
            next_input,
            ast::Command::Binding(ast::Binding::Assignment(
                v,
                ast::Type::I32,
                ast::Value::Expr(*exp),
            )),
        ))
    })
}

#[test]
fn binding_assignment1() {
    assert!(binding_assignment("let x = 12;").unwrap().0 == "");
    assert!(binding_assignment("let x = 12 * 4;").unwrap().0 == "");
    assert!(binding_assignment("let y = 12 - 5 * 6;").unwrap().0 == "");
    assert!(binding_assignment("let z = 3").is_err());
    assert!(
        binding_assignment("let z = 3;").unwrap().1
            == ast::Command::Binding(ast::Binding::Assignment(
                ast::Variable::Named("z".to_string()),
                ast::Type::I32,
                ast::Value::Expr(ast::Expr::Number(3))
            ))
    );
}
