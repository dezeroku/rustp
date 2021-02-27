use crate::ast;
use crate::parser::boolean;
use crate::parser::math;

use nom::{
    branch::alt, bytes::complete::tag, bytes::complete::take_until, bytes::complete::take_while1,
    character::complete::char, character::complete::multispace0, character::complete::newline,
    character::complete::space0, character::complete::space1, combinator::not, combinator::opt,
    multi::many0, sequence::tuple, IResult,
};

fn function_input(input: &str) -> IResult<&str, ast::Binding> {
    tuple((
        space0,
        opt(tuple((tag("mut"), space1))),
        variable,
        space0,
        char(':'),
        space0,
        type_def,
        space0,
    ))(input)
    .and_then(|(next_input, res)| {
        let (_, m, v, _, _, _, t, _) = res;
        let mu = match m {
            Some(_) => true,
            None => false,
        };
        Ok((next_input, ast::Binding::Declaration(v, t, mu)))
    })
}

#[test]
fn function_input1() {
    assert!(
        function_input("a: i32").unwrap().1
            == ast::Binding::Declaration(
                ast::Variable::Named("a".to_string()),
                ast::Type::I32,
                false
            )
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
        false,
    ));
    t.push(ast::Binding::Declaration(
        ast::Variable::Named("b".to_string()),
        ast::Type::I32,
        false,
    ));
    t.push(ast::Binding::Declaration(
        ast::Variable::Named("c".to_string()),
        ast::Type::Bool,
        true,
    ));
    assert!(function_inputs("a: i32, b: i32, mut c: bool").unwrap().1 == t);
}

pub fn precondition(input: &str) -> IResult<&str, ast::Bool> {
    tuple((
        prove_start,
        tag("precondition"),
        space1,
        boolean::expr,
        newline,
    ))(input)
    .and_then(|(next_input, res)| {
        let (_, _, _, c, _) = res;
        Ok((next_input, *c))
    })
}

#[test]
fn precondition1() {
    assert!(precondition("//%precondition false\n").unwrap().1 == ast::Bool::False);
    assert!(
        precondition("//%precondition false && true\n").unwrap().1
            == ast::Bool::And(Box::new(ast::Bool::False), Box::new(ast::Bool::True))
    );
    assert!(
        precondition("//%precondition a == 143\n").unwrap().1
            == ast::Bool::Equal(
                ast::Expr::Variable(ast::Variable::Named("a".to_string())),
                ast::Expr::Number(143)
            )
    );
}

pub fn postcondition(input: &str) -> IResult<&str, ast::Bool> {
    tuple((
        prove_start,
        tag("postcondition"),
        space1,
        boolean::expr,
        newline,
    ))(input)
    .and_then(|(next_input, res)| {
        let (_, _, _, c, _) = res;
        Ok((next_input, *c))
    })
}

#[test]
fn postcondition1() {
    assert!(postcondition("//%postcondition false\n").unwrap().1 == ast::Bool::False);
    assert!(
        postcondition("//%postcondition false && true\n").unwrap().1
            == ast::Bool::And(Box::new(ast::Bool::False), Box::new(ast::Bool::True))
    );

    assert!(
        postcondition("//%postcondition a == 143\n").unwrap().1
            == ast::Bool::Equal(
                ast::Expr::Variable(ast::Variable::Named("a".to_string())),
                ast::Expr::Number(143)
            )
    );
}

pub fn function(input: &str) -> IResult<&str, ast::Function> {
    tuple((
        opt(precondition),
        multispace0,
        opt(postcondition),
        multispace0,
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
        let (pre, _, post, _, _, _, name, _, _, _, inputs, _, _, _, out, _, _, _, comms, _, _) =
            res;

        let pre = match pre {
            Some(a) => a,
            None => ast::Bool::True,
        };

        let post = match post {
            Some(a) => a,
            None => ast::Bool::True,
        };

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
                precondition: pre,
                postcondition: post,
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
        false,
    )));
    let a = function("fn a () {let a: i32 = 14;}").unwrap().1;

    let b = ast::Function {
        name: "a".to_string(),
        content: content,
        input: Vec::new(),
        output: ast::Type::Unit,
        precondition: ast::Bool::True,
        postcondition: ast::Bool::True,
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
        false,
    )));

    let mut input = Vec::new();
    input.push(ast::Binding::Declaration(
        ast::Variable::Named("c".to_string()),
        ast::Type::I32,
        false,
    ));
    input.push(ast::Binding::Declaration(
        ast::Variable::Named("d".to_string()),
        ast::Type::Bool,
        false,
    ));
    let a = function("fn a (c: i32, d: bool) -> bool {let a: i32 = 14;}")
        .unwrap()
        .1;

    let b = ast::Function {
        name: "a".to_string(),
        content: content,
        input: input,
        output: ast::Type::Bool,
        precondition: ast::Bool::True,
        postcondition: ast::Bool::True,
    };

    assert!(a == b);
}

#[test]
fn function3() {
    let mut content = Vec::new();
    content.push(ast::Command::Binding(ast::Binding::Assignment(
        ast::Variable::Named("a".to_string()),
        ast::Type::I32,
        ast::Value::Expr(ast::Expr::Number(14)),
        false,
    )));

    let mut input = Vec::new();
    input.push(ast::Binding::Declaration(
        ast::Variable::Named("c".to_string()),
        ast::Type::I32,
        false,
    ));
    input.push(ast::Binding::Declaration(
        ast::Variable::Named("d".to_string()),
        ast::Type::Bool,
        false,
    ));
    let a = function("//%precondition false && false\n//%postcondition (a == 12) && false\n fn a (c: i32, d: bool) -> bool {let a: i32 = 14;}")
        .unwrap()
        .1;

    let b = ast::Function {
        name: "a".to_string(),
        content: content,
        input: input,
        output: ast::Type::Bool,
        precondition: ast::Bool::And(Box::new(ast::Bool::False), Box::new(ast::Bool::False)),
        postcondition: ast::Bool::And(
            Box::new(ast::Bool::Equal(
                ast::Expr::Variable(ast::Variable::Named("a".to_string())),
                ast::Expr::Number(12),
            )),
            Box::new(ast::Bool::False),
        ),
    };

    assert!(a == b);
}

pub fn block(input: &str) -> IResult<&str, Vec<ast::Command>> {
    many0(tuple((
        comments,
        multispace0,
        command,
        multispace0,
        comments,
    )))(input)
    .and_then(|(next_input, res)| {
        let mut result = Vec::new();
        for i in res {
            let (_, _, temp, _, _) = i;
            result.push(temp)
        }
        Ok((next_input, result))
    })
}

fn comments(input: &str) -> IResult<&str, Vec<&str>> {
    many0(tuple((
        multispace0,
        alt((single_comment, multiline_comment)),
        multispace0,
    )))(input)
    .and_then(|(next_input, res)| {
        let mut t = Vec::new();
        for item in res {
            let (_, c, _) = item;
            t.push(c);
        }
        Ok((next_input, t))
    })
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

fn single_comment(input: &str) -> IResult<&str, &str> {
    not(command)(input).and_then(|(next_input, _)| {
        tuple((tag("//"), take_while_not_newline, opt(newline)))(next_input).and_then(
            |(next_input, res)| {
                let (_, c, _) = res;
                Ok((next_input, c))
            },
        )
    })
}

fn multiline_comment(input: &str) -> IResult<&str, &str> {
    not(command)(input).and_then(|(next_input, _)| {
        tuple((tag("/*"), take_until("*/"), tag("*/")))(next_input).and_then(|(next_input, res)| {
            let (_, c, _) = res;
            Ok((next_input, c))
        })
    })
}

#[test]
fn block1() {
    assert!(block("let x = 1;").unwrap().0 == "");
    assert!(block("let x = 1;//%assert a < 143\n").unwrap().0 == "");
}

fn command(input: &str) -> IResult<&str, ast::Command> {
    alt((binding, prove_control, if_else))(input)
}

#[test]
fn command1() {
    assert!(command("//%assert b == 143 % 4\n").unwrap().0 == "");
}

#[test]
fn command2() {
    assert!(command("let a = 14;").unwrap().0 == "");
}

fn if_else(input: &str) -> IResult<&str, ast::Command> {
    tuple((
        single_if,
        many0(tuple((space0, tag("else"), space0, single_if))),
        opt(tuple((
            space0,
            tag("else"),
            space0,
            tag("{"),
            space0,
            block,
            space0,
            tag("}"),
        ))),
    ))(input)
    .and_then(|(next_input, res)| {
        let (first, rest, e) = res;

        let mut conds = Vec::new();
        let mut comms = Vec::new();

        match first {
            ast::Command::Block(b) => match b {
                ast::Block::If(mut con, mut com, _) => {
                    conds.push(con.pop().unwrap());
                    comms.push(com.pop().unwrap());
                }
                _ => panic!("Not valid scenario"),
            },
            _ => panic!("Not valid scenario"),
        }

        for elem in rest {
            let (_, _, _, i) = elem;
            match i {
                ast::Command::Block(b) => match b {
                    ast::Block::If(mut con, mut com, _) => {
                        conds.push(con.pop().unwrap());
                        comms.push(com.pop().unwrap());
                    }
                    _ => panic!("Not valid scenario"),
                },
                _ => panic!("Not valid scenario"),
            }
        }

        let el = match e {
            Some((_, _, _, _, _, b, _, _)) => b,
            None => Vec::new(),
        };

        Ok((
            next_input,
            ast::Command::Block(ast::Block::If(conds, comms, el)),
        ))
    })
}

#[test]
fn if_else1() {
    assert!(if_else("if a == 12 {let b = 3;}").unwrap().0 == "");
    assert!(
        if_else("if a == 12 {let b = 3;//%assert b == 3\n}")
            .unwrap()
            .0
            == ""
    );
}

#[test]
fn if_else2() {
    assert!(
        if_else("if a == 12 {let b = 3;} else if a == 13 {let b = 4;} else {let b = 1;}")
            .unwrap()
            .0
            == ""
    );
}

#[test]
fn if_else3() {
    let a = if_else("if a == 12 {let b = 3;} else if a == 13 {let b = 4;} else {let b = 1;}")
        .unwrap()
        .1;

    let mut conds = Vec::new();
    let mut comms = Vec::new();
    let mut comms_1 = Vec::new();
    let mut comms_2 = Vec::new();

    let mut el = Vec::new();

    conds.push(ast::Bool::Equal(
        ast::Expr::Variable(ast::Variable::Named("a".to_string())),
        ast::Expr::Number(12),
    ));

    conds.push(ast::Bool::Equal(
        ast::Expr::Variable(ast::Variable::Named("a".to_string())),
        ast::Expr::Number(13),
    ));

    comms_1.push(ast::Command::Binding(ast::Binding::Assignment(
        ast::Variable::Named("b".to_string()),
        ast::Type::Unknown,
        ast::Value::Expr(ast::Expr::Number(3)),
        false,
    )));
    comms_2.push(ast::Command::Binding(ast::Binding::Assignment(
        ast::Variable::Named("b".to_string()),
        ast::Type::Unknown,
        ast::Value::Expr(ast::Expr::Number(4)),
        false,
    )));

    comms.push(comms_1);
    comms.push(comms_2);

    el.push(ast::Command::Binding(ast::Binding::Assignment(
        ast::Variable::Named("b".to_string()),
        ast::Type::Unknown,
        ast::Value::Expr(ast::Expr::Number(1)),
        false,
    )));

    let b = ast::Command::Block(ast::Block::If(conds, comms, el));

    assert!(a == b);
}

#[test]
fn if_else4() {
    assert!(if_else("if a == 14 {\nlet c = 3;\n}").unwrap().0 == "");
    assert!(
        if_else("if a == 14 {\nlet c = 3;\n} else if a == 13 {\n   let c = a + 43;\n}")
            .unwrap()
            .0
            == ""
    );
    assert!(if_else("if a == 14 {\nlet c = 3;\n} else if a == 13 {\n   let c = a + 43;\n} else {\n   let c = a + 123;\n}").unwrap().0 == "");
}

fn single_if(input: &str) -> IResult<&str, ast::Command> {
    tuple((
        tag("if"),
        space0,
        boolean::expr,
        space0,
        tag("{"),
        block,
        tag("}"),
        space0,
    ))(input)
    .and_then(|(next_input, res)| {
        let (_, _, c, _, _, b, _, _) = res;
        let mut conds = Vec::new();
        conds.push(*c);

        let mut comms = Vec::new();
        comms.push(b);

        let t = ast::Block::If(conds, comms, Vec::new());
        Ok((next_input, ast::Command::Block(t)))
    })
}

fn single_if1() {
    assert!(single_if("if a == 12 {let b = 3;}").unwrap().0 == "");
    assert!(
        single_if("if a == 12 {let b = 3;//%assert b == 3\n}")
            .unwrap()
            .0
            == ""
    );
}

fn prove_control(input: &str) -> IResult<&str, ast::Command> {
    alt((assert, assume, loop_invariant))(input)
}

fn assert(input: &str) -> IResult<&str, ast::Command> {
    tuple((prove_start, tag("assert"), space1, boolean::expr, newline))(input).map(
        |(next_input, res)| {
            let (_, _, _, a, _) = res;
            (
                next_input,
                ast::Command::ProveControl(ast::ProveControl::Assert(*a)),
            )
        },
    )
}

#[test]
fn assert1() {
    assert!(assert("//%assert 143 == 12\n").is_ok());
    assert!(assert("//%assert 143 - 4 < 2\n").unwrap().0 == "");
    assert!(assert("//%assert true\n").unwrap().0 == "");
}

fn assume(input: &str) -> IResult<&str, ast::Command> {
    tuple((prove_start, tag("assume"), space1, boolean::expr, newline))(input).map(
        |(next_input, res)| {
            let (_, _, _, a, _) = res;
            (
                next_input,
                ast::Command::ProveControl(ast::ProveControl::Assume(*a)),
            )
        },
    )
}

#[test]
fn assume1() {
    assert!(assume("//%assume 143 == 12\n").is_ok());
    assert!(assume("//%assume 143 - 4 < 2\n").unwrap().0 == "");
    assert!(assume("//%assume true\n").unwrap().0 == "");
}

fn loop_invariant(input: &str) -> IResult<&str, ast::Command> {
    tuple((
        prove_start,
        tag("loop_invariant"),
        space1,
        boolean::expr,
        newline,
    ))(input)
    .map(|(next_input, res)| {
        let (_, _, _, a, _) = res;
        (
            next_input,
            ast::Command::ProveControl(ast::ProveControl::LoopInvariant(*a)),
        )
    })
}

#[test]
fn loop_invariant1() {
    assert!(loop_invariant("//%loop_invariant 143 == 12\n").is_ok());
    assert!(loop_invariant("//%loop_invariant 143 - 4 < 2\n").unwrap().0 == "");
    assert!(loop_invariant("//%loop_invariant true\n").unwrap().0 == "");
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
    alt((value_bool, value_expr))(input)
}

fn value_bool(input: &str) -> IResult<&str, ast::Value> {
    boolean::expr(input).and_then(|(next_input, res)| Ok((next_input, ast::Value::Bool(*res))))
}

fn value_expr(input: &str) -> IResult<&str, ast::Value> {
    math::expr(input).and_then(|(next_input, res)| Ok((next_input, ast::Value::Expr(*res))))
}

fn binding_assignment(input: &str) -> IResult<&str, ast::Command> {
    tuple((
        space0,
        tag("let"),
        space1,
        opt(tuple((tag("mut"), space1))),
        variable,
        space0,
        opt(tuple((char(':'), space0, type_def, space0))),
        alt((
            tuple((char('='), space0, value_bool, space0, char(';'))),
            tuple((char('='), space0, value_expr, space0, char(';'))),
        )),
    ))(input)
    .and_then(|(next_input, x)| {
        let (_, _, _, m, v, _, t, tu) = x;
        let (_, _, exp, _, _) = tu;
        let mu = match m {
            Some(a) => true,
            None => false,
        };
        match t {
            Some((_, _, t, _)) => Ok((
                next_input,
                ast::Command::Binding(ast::Binding::Assignment(v, t, exp, mu)),
            )),
            None => Ok((
                next_input,
                ast::Command::Binding(ast::Binding::Assignment(v, ast::Type::Unknown, exp, mu)),
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
        binding_assignment("let mut z: i32 = 3;").unwrap().1
            == ast::Command::Binding(ast::Binding::Assignment(
                ast::Variable::Named("z".to_string()),
                ast::Type::I32,
                ast::Value::Expr(ast::Expr::Number(3)),
                true
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
                ast::Value::Expr(ast::Expr::Number(3)),
                false
            ))
    );
    assert!(
        binding_assignment("let z: bool = true;").unwrap().1
            == ast::Command::Binding(ast::Binding::Assignment(
                ast::Variable::Named("z".to_string()),
                ast::Type::Bool,
                ast::Value::Bool(ast::Bool::True),
                false
            ))
    );
    assert!(binding_assignment("let c = a + 43;").unwrap().0 == "");
}

fn binding_declaration(input: &str) -> IResult<&str, ast::Command> {
    tuple((
        space0,
        tag("let"),
        space1,
        opt(tuple((tag("mut"), space1))),
        variable,
        space0,
        opt(tuple((char(':'), space0, type_def, space0))),
        space0,
        char(';'),
    ))(input)
    .and_then(|(next_input, x)| {
        let (_, _, _, m, v, _, t, _, _) = x;
        let mu = match m {
            Some(a) => true,
            None => false,
        };
        match t {
            Some((_, _, t, _)) => Ok((
                next_input,
                ast::Command::Binding(ast::Binding::Declaration(v, t, mu)),
            )),
            None => Ok((
                next_input,
                ast::Command::Binding(ast::Binding::Declaration(v, ast::Type::Unknown, mu)),
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
                ast::Type::I32,
                false
            ))
    );
    assert!(
        binding_declaration("let mut z:bool;").unwrap().1
            == ast::Command::Binding(ast::Binding::Declaration(
                ast::Variable::Named("z".to_string()),
                ast::Type::Bool,
                true
            ))
    );
    assert!(
        binding_declaration("let x;").unwrap().1
            == ast::Command::Binding(ast::Binding::Declaration(
                ast::Variable::Named("x".to_string()),
                ast::Type::Unknown,
                false
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
