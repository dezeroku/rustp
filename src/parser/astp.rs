use crate::ast;
use crate::parser::boolean;
use crate::parser::math;

use nom::{
    branch::alt, bytes::complete::tag, bytes::complete::take_while1, character::complete::char,
    character::complete::multispace0, character::complete::newline, character::complete::space0,
    character::complete::space1, combinator::opt, multi::many0, sequence::tuple, IResult,
};

fn function_input(input: &str) -> IResult<&str, ast::Binding> {
    tuple((
        space0,
        variable,
        space0,
        char(':'),
        space0,
        type_def,
        space0,
    ))(input)
    .and_then(|(next_input, res)| {
        let (_, v, _, _, _, t, _) = res;
        Ok((next_input, ast::Binding::Declaration(v, t)))
    })
}

#[test]
fn function_input1() {
    assert!(
        function_input("a: i32").unwrap().1
            == ast::Binding::Declaration(ast::Variable::Named("a".to_string()), ast::Type::I32)
    );
}

fn function_inputs(input: &str) -> IResult<&str, Vec<ast::Binding>> {
    function_input(input)
        .and_then(|(next_input, res)| {
            let f = many0(tuple((char(','), function_input)))(next_input);
            let (next_input, rest) = match f {
                Ok((next_input, next)) => (next_input, next),
                Err(_) => (next_input, Vec::new()),
            };
            let mut content = Vec::new();
            content.push(res);

            for item in rest {
                let (_, i) = item;
                content.push(i);
            }

            Ok((next_input, content))
        })
        .or_else(|_| Ok((input, Vec::new())))
}

#[test]
fn function_inputs1() {
    let mut t = Vec::new();
    t.push(ast::Binding::Declaration(
        ast::Variable::Named("a".to_string()),
        ast::Type::I32,
    ));
    t.push(ast::Binding::Declaration(
        ast::Variable::Named("b".to_string()),
        ast::Type::I32,
    ));
    t.push(ast::Binding::Declaration(
        ast::Variable::Named("c".to_string()),
        ast::Type::Bool,
    ));
    assert!(function_inputs("a: i32, b: i32, c: bool").unwrap().1 == t);
}

pub fn function(input: &str) -> IResult<&str, ast::Function> {
    tuple((
        tag("fn"),
        space1,
        function_name,
        space1,
        tag("("),
        space0,
        function_inputs,
        space0,
        tag(")"),
        space0,
        opt(tuple((tag("->"), space0, type_def))),
        space0,
        tag("{"),
        space0,
        block,
        space0,
        tag("}"),
    ))(input)
    .and_then(|(next_input, res)| {
        let (_, _, name, _, _, _, inputs, _, _, _, out, _, _, _, comms, _, _) = res;
        let o = match out {
            Some((_, _, a)) => a,
            None => ast::Type::Unit,
        };
        Ok((
            next_input,
            ast::Function {
                name: name.to_string(),
                content: comms,
                input: inputs,
                output: o,
            },
        ))
    })
}

#[test]
fn function1() {
    assert!(function("fn a () {}").unwrap().0 == "");
    assert!(function("fn a ( ) {}").unwrap().0 == "");
    assert!(function("fn a ( ) { }").unwrap().0 == "");
    assert!(function("fn  a  ( ) { }").unwrap().0 == "");
    assert!(function("fn a () {let a = 14;}").unwrap().0 == "");
    assert!(function("fn a () {let a = 14; let c = 1 + b;}").unwrap().0 == "");
    let mut content = Vec::new();
    content.push(ast::Command::Binding(ast::Binding::Assignment(
        ast::Variable::Named("a".to_string()),
        ast::Type::I32,
        ast::Value::Expr(ast::Expr::Number(14)),
    )));
    let a = function("fn a () {let a: i32 = 14;}").unwrap().1;

    let b = ast::Function {
        name: "a".to_string(),
        content: content,
        input: Vec::new(),
        output: ast::Type::Unit,
    };

    assert!(a == b);
}

#[test]
fn function2() {
    assert!(function("fn a (a: i32) {}").unwrap().0 == "");
    assert!(function("fn a (b: bool, a:i32 ) {}").unwrap().0 == "");
    assert!(function("fn a ( ) -> i32{ }").unwrap().0 == "");
    assert!(function("fn  a  (b: bool ) -> bool{ }").unwrap().0 == "");
    let mut content = Vec::new();
    content.push(ast::Command::Binding(ast::Binding::Assignment(
        ast::Variable::Named("a".to_string()),
        ast::Type::I32,
        ast::Value::Expr(ast::Expr::Number(14)),
    )));

    let mut input = Vec::new();
    input.push(ast::Binding::Declaration(
        ast::Variable::Named("c".to_string()),
        ast::Type::I32,
    ));
    input.push(ast::Binding::Declaration(
        ast::Variable::Named("d".to_string()),
        ast::Type::Bool,
    ));
    let a = function("fn a (c: i32, d: bool) -> bool {let a: i32 = 14;}")
        .unwrap()
        .1;

    let b = ast::Function {
        name: "a".to_string(),
        content: content,
        input: input,
        output: ast::Type::Bool,
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
    alt((binding_assignment, binding_declaration))(input)
}

fn value(input: &str) -> IResult<&str, ast::Value> {
    boolean::expr(input)
        .and_then(|(next_input, res)| Ok((next_input, ast::Value::Bool(*res))))
        .or_else(|_| {
            math::expr(input).and_then(|(next_input, res)| Ok((next_input, ast::Value::Expr(*res))))
        })
}

fn binding_assignment(input: &str) -> IResult<&str, ast::Command> {
    tuple((
        space0,
        tag("let "),
        space0,
        variable,
        space0,
        opt(tuple((char(':'), space0, type_def, space0))),
        char('='),
        space0,
        value,
        space0,
        char(';'),
    ))(input)
    .and_then(|(next_input, x)| {
        let (_, _, _, v, _, t, _, _, exp, _, _) = x;
        match t {
            Some((_, _, t, _)) => Ok((
                next_input,
                ast::Command::Binding(ast::Binding::Assignment(v, t, exp)),
            )),
            None => Ok((
                next_input,
                ast::Command::Binding(ast::Binding::Assignment(v, ast::Type::Unknown, exp)),
            )),
        }
    })
}

#[test]
fn binding_assignment1() {
    assert!(binding_assignment("let x: i32 = 12;").unwrap().0 == "");
    assert!(binding_assignment("let x = 12 * 4;").unwrap().0 == "");
    assert!(binding_assignment("let y = 12 - 5 * 6;").unwrap().0 == "");
    assert!(binding_assignment("let z = 3").is_err());
    assert!(
        binding_assignment("let z: i32 = 3;").unwrap().1
            == ast::Command::Binding(ast::Binding::Assignment(
                ast::Variable::Named("z".to_string()),
                ast::Type::I32,
                ast::Value::Expr(ast::Expr::Number(3))
            ))
    );
    // This one will be validated later on?
    // Or can we just assume that the code we are getting is correct Rust?
    // TODO: run check with rustc before even starting to parse AST on our side.
    assert!(
        binding_assignment("let z: bool = 3;").unwrap().1
            == ast::Command::Binding(ast::Binding::Assignment(
                ast::Variable::Named("z".to_string()),
                ast::Type::Bool,
                ast::Value::Expr(ast::Expr::Number(3))
            ))
    );
    assert!(
        binding_assignment("let z: bool = true;").unwrap().1
            == ast::Command::Binding(ast::Binding::Assignment(
                ast::Variable::Named("z".to_string()),
                ast::Type::Bool,
                ast::Value::Bool(ast::Bool::True)
            ))
    );
}

fn binding_declaration(input: &str) -> IResult<&str, ast::Command> {
    tuple((
        space0,
        tag("let "),
        space0,
        variable,
        space0,
        opt(tuple((char(':'), space0, type_def, space0))),
        space0,
        char(';'),
    ))(input)
    .and_then(|(next_input, x)| {
        let (_, _, _, v, _, t, _, _) = x;
        match t {
            Some((_, _, t, _)) => Ok((
                next_input,
                ast::Command::Binding(ast::Binding::Declaration(v, t)),
            )),
            None => Ok((
                next_input,
                ast::Command::Binding(ast::Binding::Declaration(v, ast::Type::Unknown)),
            )),
        }
    })
}

#[test]
fn binding_declaration1() {
    assert!(binding_declaration("let x: i32;").unwrap().0 == "");
    assert!(binding_declaration("let x;").unwrap().0 == "");
    assert!(binding_declaration("let y: bool;").unwrap().0 == "");
    assert!(
        binding_declaration("let z:i32;").unwrap().1
            == ast::Command::Binding(ast::Binding::Declaration(
                ast::Variable::Named("z".to_string()),
                ast::Type::I32
            ))
    );
    assert!(
        binding_declaration("let z:bool;").unwrap().1
            == ast::Command::Binding(ast::Binding::Declaration(
                ast::Variable::Named("z".to_string()),
                ast::Type::Bool
            ))
    );
    assert!(
        binding_declaration("let x;").unwrap().1
            == ast::Command::Binding(ast::Binding::Declaration(
                ast::Variable::Named("x".to_string()),
                ast::Type::Unknown
            ))
    );
}

fn type_def(input: &str) -> IResult<&str, ast::Type> {
    // TODO: handle vector, tuple, etc.
    alt((type_def_bool, type_def_i32))(input)
}

fn type_def_bool(input: &str) -> IResult<&str, ast::Type> {
    tag("bool")(input).and_then(|(next_input, _)| Ok((next_input, ast::Type::Bool)))
}

fn type_def_i32(input: &str) -> IResult<&str, ast::Type> {
    tag("i32")(input).and_then(|(next_input, _)| Ok((next_input, ast::Type::I32)))
}
