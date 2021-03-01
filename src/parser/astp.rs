use crate::ast;
use crate::parser::boolean;
use crate::parser::math;
use itertools::izip;

use nom::{
    branch::alt, bytes::complete::tag, bytes::complete::take_until, bytes::complete::take_while1,
    character::complete::char, character::complete::multispace0, character::complete::newline,
    character::complete::space0, character::complete::space1, combinator::not, combinator::opt,
    multi::many0, sequence::tuple, IResult,
};

static KEYWORDS: [&'static str; 7] = ["let", "true", "false", "&&", "||", "!", "_"];

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

fn command(input: &str) -> IResult<&str, ast::Command> {
    alt((
        binding,
        prove_control,
        assignment,
        assignment_tuple_unpack,
        if_else,
    ))(input)
}

fn assignment(input: &str) -> IResult<&str, ast::Command> {
    alt((
        tuple((
            space0,
            variable,
            space0,
            tag("="),
            space0,
            array_content,
            space0,
            tag(";"),
        )),
        tuple((
            space0,
            variable,
            space0,
            tag("="),
            space0,
            _tuple,
            space0,
            tag(";"),
        )),
        tuple((
            space0,
            variable,
            space0,
            tag("="),
            space0,
            boolean::expr_val,
            space0,
            tag(";"),
        )),
        tuple((
            space0,
            variable,
            space0,
            tag("="),
            space0,
            math::expr_val,
            space0,
            tag(";"),
        )),
    ))(input)
    .and_then(|(next_input, res)| {
        let (_, v, _, _, _, val, _, _) = res;
        Ok((next_input, ast::Command::Assignment(v, val)))
    })
}

fn assignment_tuple_unpack(input: &str) -> IResult<&str, ast::Command> {
    // TODO: properly handle mutability for each element
    // TODO: properly handle unpacking single value from right to multiple values on left
    tuple((
        space0,
        _tuple_unpack_left,
        space0,
        tuple((char('='), space0, _tuple, space0, char(';'))),
    ))(input)
    .and_then(|(next_input, x)| {
        let (_, v, _, tu) = x;
        if let (_, _, ast::Value::Tuple(exp), _, _) = tu {
            let mut result = Vec::new();
            for (var, val) in itertools::izip!(v, exp) {
                result.push(ast::Command::Assignment(var, val));
            }

            Ok((next_input, ast::Command::TupleAssignment(result)))
        } else {
            panic!("Incorrect case")
        }
    })
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

fn prove_start(input: &str) -> IResult<&str, &str> {
    tag("//%")(input)
}

pub fn function_name(input: &str) -> IResult<&str, &str> {
    let l = |x: char| char::is_alphabetic(x) || '_' == x;

    take_while1(l)(input).and_then(|(next_input, res)| Ok((next_input, res)))
}

fn variable_name(input: &str) -> IResult<&str, &str> {
    let l = |x: char| char::is_alphabetic(x) || '_' == x;

    take_while1(l)(input).and_then(|(next_input, res)| {
        // if matches then fail
        if !KEYWORDS.contains(&res) {
            Ok((next_input, res))
        } else {
            Err((nom::Err::Error(nom::error::Error::new(res, nom::error::ErrorKind::Tag))))
        }
    })
}

/// Defines everything what can be used on the right side of an assignment or binding.
pub fn r_value(input: &str) -> IResult<&str, ast::Value> {
    // TODO: this will be problematic due to possibility of boolean and math expr_val consuming same input, just in a different level
    // handle it somehow based on the length of input matched?
    alt((
        _tuple,
        function_call,
        variable_val,
        math::expr_val,
        boolean::expr_val,
    ))(input)
}

pub fn function_call(input: &str) -> IResult<&str, ast::Value> {
    tuple((
        function_name,
        space0,
        tag("("),
        space0,
        r_value,
        many0(tuple((space0, tag(","), space0, r_value))),
        space0,
        tag(")"),
    ))(input)
    .and_then(|(next_input, res)| {
        let (n, _, _, _, v, a, _, _) = res;
        let mut t = Vec::new();
        t.push(v);

        for item in a {
            let (_, _, _, i) = item;
            t.push(i);
        }

        Ok((next_input, ast::Value::FunctionCall(n.to_string(), t)))
    })
}

pub fn variable_val(input: &str) -> IResult<&str, ast::Value> {
    variable(input).and_then(|(next_input, res)| Ok((next_input, ast::Value::Variable(res))))
}

pub fn variable(input: &str) -> IResult<&str, ast::Variable> {
    alt((variable_array_elem, variable_single))(input)
}

fn variable_single(input: &str) -> IResult<&str, ast::Variable> {
    variable_name(input)
        .and_then(|(next_input, res)| Ok((next_input, ast::Variable::Named(res.to_string()))))
}

fn variable_array_elem(input: &str) -> IResult<&str, ast::Variable> {
    tuple((variable_name, char('['), r_value, char(']')))(input).and_then(
        |(next_input, (v, _, i, _))| {
            Ok((
                next_input,
                ast::Variable::ArrayElem(v.to_string(), Box::new(i)),
            ))
        },
    )
}

fn binding(input: &str) -> IResult<&str, ast::Command> {
    alt((
        binding_assignment_tuple_unpack,
        binding_assignment,
        binding_declaration,
    ))(input)
}

fn value(input: &str) -> IResult<&str, ast::Value> {
    alt((boolean::expr_val, math::expr_val))(input)
}

fn binding_assignment_tuple_unpack(input: &str) -> IResult<&str, ast::Command> {
    // TODO: properly handle mutability for each element
    tuple((
        space0,
        tag("let"),
        space1,
        _tuple_unpack_left,
        space0,
        opt(tuple((char(':'), space0, type_def, space0))),
        tuple((char('='), space0, _tuple, space0, char(';'))),
    ))(input)
    .and_then(|(next_input, x)| {
        let (_, _, _, v, _, t, tu) = x;
        if let (_, _, ast::Value::Tuple(exp), _, _) = tu {
            let mut result = Vec::new();
            match t {
                Some((_, _, ast::Type::Tuple(a), _)) => {
                    for (var, ty, val) in itertools::izip!(v, a, exp) {
                        result.push(ast::Binding::Assignment(var, ty, val, false));
                    }
                }
                None => {
                    for (var, val) in itertools::izip!(v, exp) {
                        result.push(ast::Binding::Assignment(
                            var,
                            ast::Type::Unknown,
                            val,
                            false,
                        ));
                    }
                }
                _ => panic!("Incorrect case"),
            }

            Ok((next_input, ast::Command::TupleBinding(result)))
        } else {
            panic!("Incorrect case")
        }
    })
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
            tuple((char('='), space0, array_content, space0, char(';'))),
            tuple((char('='), space0, _tuple, space0, char(';'))),
            tuple((char('='), space0, boolean::expr_val, space0, char(';'))),
            tuple((char('='), space0, math::expr_val, space0, char(';'))),
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

fn array_content(input: &str) -> IResult<&str, ast::Value> {
    tuple((
        char('['),
        space0,
        r_value,
        space0,
        many0(tuple((char(','), space0, r_value))),
        space0,
        char(']'),
    ))(input)
    .and_then(|(next_input, (_, _, v, _, r, _, _))| {
        let mut result = Vec::new();
        result.push(v);
        for (_, _, i) in r {
            result.push(i);
        }

        Ok((next_input, ast::Value::Array(result)))
    })
}

// TODO: _tuple_unpacking can work similarly, but return vector of values and handle _ properly
// single tuple assignment probably should be split into multiples assignments, each assigning a single value to a specific variable?
// In this way _ could just be skipped.

fn _tuple_empty_elem(input: &str) -> IResult<&str, ast::Variable> {
    tag("_")(input).and_then(|(next_input, res)| Ok((next_input, ast::Variable::Empty)))
}

fn _tuple_unpack_left(input: &str) -> IResult<&str, Vec<ast::Variable>> {
    tuple((
        tag("("),
        space0,
        alt((variable, _tuple_empty_elem)),
        space0,
        char(','),
        space0,
        opt(alt((variable, _tuple_empty_elem))),
        space0,
        many0(tuple((
            char(','),
            space0,
            alt((variable, _tuple_empty_elem)),
            space0,
        ))),
        space0,
        tag(")"),
    ))(input)
    .and_then(|(next_input, res)| {
        let (_, _, f, _, _, _, s, _, r, _, _) = res;
        let mut result = Vec::new();
        result.push(f);
        match s {
            Some(a) => result.push(a),
            None => {}
        }

        for item in r {
            let (_, _, t, _) = item;
            result.push(t);
        }

        Ok((next_input, result))
    })
}

fn _tuple_type(input: &str) -> IResult<&str, ast::Type> {
    tuple((
        tag("("),
        space0,
        type_def_single,
        space0,
        char(','),
        space0,
        opt(type_def_single),
        space0,
        many0(tuple((char(','), space0, type_def_single, space0))),
        space0,
        tag(")"),
    ))(input)
    .and_then(|(next_input, res)| {
        let (_, _, f, _, _, _, s, _, r, _, _) = res;
        let mut result = Vec::new();
        result.push(f);
        match s {
            Some(a) => result.push(a),
            None => {}
        }

        for item in r {
            let (_, _, t, _) = item;
            result.push(t);
        }

        Ok((next_input, ast::Type::Tuple(result)))
    })
}

fn _tuple(input: &str) -> IResult<&str, ast::Value> {
    tuple((
        tag("("),
        space0,
        r_value,
        space0,
        char(','),
        space0,
        opt(r_value),
        space0,
        many0(tuple((char(','), space0, r_value, space0))),
        space0,
        tag(")"),
    ))(input)
    .and_then(|(next_input, res)| {
        let (_, _, f, _, _, _, s, _, r, _, _) = res;
        let mut result = Vec::new();
        result.push(f);
        match s {
            Some(a) => result.push(a),
            None => {}
        }

        for item in r {
            let (_, _, t, _) = item;
            result.push(t);
        }

        Ok((next_input, ast::Value::Tuple(result)))
    })
}

fn array_type(input: &str) -> IResult<&str, ast::Type> {
    tuple((
        char('['),
        space0,
        type_def_single,
        space0,
        char(';'),
        space0,
        math::number,
        space0,
        char(']'),
    ))(input)
    .and_then(|(next_input, (_, _, t, _, _, _, i, _, _))| {
        Ok((next_input, ast::Type::Array(Box::new(t), i)))
    })
}

fn type_def(input: &str) -> IResult<&str, ast::Type> {
    alt((array_type, _tuple_type, type_def_single))(input)
}

fn type_def_single(input: &str) -> IResult<&str, ast::Type> {
    alt((type_def_bool, type_def_i32))(input)
}

fn type_def_bool(input: &str) -> IResult<&str, ast::Type> {
    tag("bool")(input).and_then(|(next_input, _)| Ok((next_input, ast::Type::Bool)))
}

fn type_def_i32(input: &str) -> IResult<&str, ast::Type> {
    tag("i32")(input).and_then(|(next_input, _)| Ok((next_input, ast::Type::I32)))
}

#[cfg(test)]
mod test {
    use super::*;

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
                    ast::Expr::Value(Box::new(ast::Value::Variable(ast::Variable::Named(
                        "a".to_string()
                    )))),
                    ast::Expr::Number(143)
                )
        );
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
                    ast::Expr::Value(Box::new(ast::Value::Variable(ast::Variable::Named(
                        "a".to_string()
                    )))),
                    ast::Expr::Number(143)
                )
        );
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
                    ast::Expr::Value(Box::new(ast::Value::Variable(ast::Variable::Named(
                        "a".to_string(),
                    )))),
                    ast::Expr::Number(12),
                )),
                Box::new(ast::Bool::False),
            ),
        };

        assert!(a == b);
    }

    #[test]
    fn block1() {
        assert!(block("let x = 1;").unwrap().0 == "");
        assert!(block("let x = 1;//%assert a < 143\n").unwrap().0 == "");
    }

    #[test]
    fn take_while_not_newline1() {
        assert!(take_while_not_newline("ababab\n").unwrap().0 == "\n");
        assert!(take_while_not_newline("ababab").unwrap().0 == "");
        assert!(take_while_not_newline("ababab").unwrap().1 == "ababab");
    }

    #[test]
    fn command1() {
        assert!(command("//%assert b == 143 % 4\n").unwrap().0 == "");
    }

    #[test]
    fn command2() {
        assert!(command("let a = 14;").unwrap().0 == "");
    }

    #[test]
    fn array_content1() {
        assert!(array_content("[1,]").is_err());
        assert!(array_content("[1,2]").unwrap().0 == "");
        let mut temp = Vec::new();
        temp.push(ast::Value::Expr(ast::Expr::Number(1)));
        temp.push(ast::Value::Expr(ast::Expr::Number(2)));
        temp.push(ast::Value::Expr(ast::Expr::Number(4)));

        assert!(array_content("[1,2, 4]").unwrap().1 == ast::Value::Array(temp));
    }

    #[test]
    fn array_type1() {
        assert!(array_type("[i32,]").is_err());
        assert!(array_type("[i32;]").is_err());
        assert!(array_type("[i32;4]").unwrap().1 == ast::Type::Array(Box::new(ast::Type::I32), 4));
        assert!(
            array_type("[bool;4]").unwrap().1 == ast::Type::Array(Box::new(ast::Type::Bool), 4)
        );
    }

    #[test]
    fn assignment1() {
        assert!(assignment("a = 12;").unwrap().0 == "");
        assert!(
            assignment("b = 12;").unwrap().1
                == ast::Command::Assignment(
                    ast::Variable::Named("b".to_string()),
                    ast::Value::Expr(ast::Expr::Number(12))
                )
        );
        assert!(
            assignment("b = true;").unwrap().1
                == ast::Command::Assignment(
                    ast::Variable::Named("b".to_string()),
                    ast::Value::Bool(ast::Bool::True)
                )
        );
        assert!(assignment("a = (12, false, a);").unwrap().0 == "");
    }

    #[test]
    fn assignment2() {
        assert!(assignment("a[1] = 12;").unwrap().0 == "");
        assert!(
            assignment("b[3] = 12;").unwrap().1
                == ast::Command::Assignment(
                    ast::Variable::ArrayElem(
                        "b".to_string(),
                        Box::new(ast::Value::Expr(ast::Expr::Number(3)))
                    ),
                    ast::Value::Expr(ast::Expr::Number(12))
                )
        );

        let mut temp = Vec::new();
        temp.push(ast::Value::Expr(ast::Expr::Number(12)));
        temp.push(ast::Value::Expr(ast::Expr::Number(3)));
        temp.push(ast::Value::Expr(ast::Expr::Number(1)));
        assert!(
            assignment("b = [12, 3, 1];").unwrap().1
                == ast::Command::Assignment(
                    ast::Variable::Named("b".to_string(),),
                    ast::Value::Array(temp)
                )
        );
    }

    #[test]
    fn assignment_tuple_unpack1() {
        assert!(assignment_tuple_unpack("(x,) = (12,);").unwrap().0 == "");
        assert!(assignment_tuple_unpack("(x,y) = (12, true);").unwrap().0 == "");
        assert!(
            assignment_tuple_unpack("(x,y, _) = (12, true, false);")
                .unwrap()
                .0
                == ""
        );
        let mut temp = Vec::new();
        temp.push(ast::Command::Assignment(
            ast::Variable::Named("x".to_string()),
            ast::Value::Expr(ast::Expr::Number(12)),
        ));
        temp.push(ast::Command::Assignment(
            ast::Variable::Named("y".to_string()),
            ast::Value::Bool(ast::Bool::True),
        ));
        temp.push(ast::Command::Assignment(
            ast::Variable::Empty,
            ast::Value::Bool(ast::Bool::False),
        ));

        assert!(
            assignment_tuple_unpack("(x,y, _) = (12, true, false);")
                .unwrap()
                .1
                == ast::Command::TupleAssignment(temp)
        );
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
            ast::Expr::Value(Box::new(ast::Value::Variable(ast::Variable::Named(
                "a".to_string(),
            )))),
            ast::Expr::Number(12),
        ));

        conds.push(ast::Bool::Equal(
            ast::Expr::Value(Box::new(ast::Value::Variable(ast::Variable::Named(
                "a".to_string(),
            )))),
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

    #[test]
    fn single_if1() {
        assert!(single_if("if a == 12 {let b = 3;}").unwrap().0 == "");
        assert!(
            single_if("if a == 12 {let b = 3;//%assert b == 3\n}")
                .unwrap()
                .0
                == ""
        );
    }

    #[test]
    fn assert1() {
        assert!(assert("//%assert 143 == 12\n").is_ok());
        assert!(assert("//%assert 143 - 4 < 2\n").unwrap().0 == "");
        assert!(assert("//%assert true\n").unwrap().0 == "");
    }

    #[test]
    fn assume1() {
        assert!(assume("//%assume 143 == 12\n").is_ok());
        assert!(assume("//%assume 143 - 4 < 2\n").unwrap().0 == "");
        assert!(assume("//%assume true\n").unwrap().0 == "");
    }

    #[test]
    fn loop_invariant1() {
        assert!(loop_invariant("//%loop_invariant 143 == 12\n").is_ok());
        assert!(loop_invariant("//%loop_invariant 143 - 4 < 2\n").unwrap().0 == "");
        assert!(loop_invariant("//%loop_invariant true\n").unwrap().0 == "");
    }

    #[test]
    fn prove_start1() {
        assert!(prove_start("//%").unwrap().0 == "");
        assert!(prove_start("/%").is_err());
    }

    #[test]
    fn function_call1() {
        assert!(function_call("abba(a, b)").unwrap().0 == "");
        assert!(function_call("abba(12, a, true)").unwrap().0 == "");
        assert!(function_call("xd(32, true)").unwrap().0 == "");
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

    #[test]
    fn variable3() {
        assert!(variable("true").is_err());
        assert!(variable("false").is_err());
    }

    #[test]
    fn variable_4() {
        assert!(variable("a[]").unwrap().0 == "[]");
        assert!(variable("a[1]").is_ok());
        assert!(variable("abc[a]").unwrap().0 == "");
        assert!(
            variable("abc[c]").unwrap().1
                == ast::Variable::ArrayElem(
                    "abc".to_string(),
                    Box::new(ast::Value::Variable(ast::Variable::Named("c".to_string())))
                )
        );
    }

    #[test]
    fn variable_array_elem1() {
        assert!(variable_array_elem("a[]").is_err());
        assert!(variable_array_elem("a[1]").is_ok());
        assert!(variable_array_elem("abc[a]").unwrap().0 == "");
        assert!(
            variable_array_elem("abc[c]").unwrap().1
                == ast::Variable::ArrayElem(
                    "abc".to_string(),
                    Box::new(ast::Value::Variable(ast::Variable::Named("c".to_string())))
                )
        );
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
        assert!(
            binding_assignment("let c: (i32, bool) = (12, false);")
                .unwrap()
                .0
                == ""
        );
    }

    #[test]
    fn binding_assigment2() {
        let mut temp = Vec::new();
        temp.push(ast::Value::Expr(ast::Expr::Number(1)));
        temp.push(ast::Value::Expr(ast::Expr::Number(2)));
        temp.push(ast::Value::Expr(ast::Expr::Number(4)));

        assert!(
            binding_assignment("let x = [1,2, 4];").unwrap().1
                == ast::Command::Binding(ast::Binding::Assignment(
                    ast::Variable::Named("x".to_string()),
                    ast::Type::Unknown,
                    ast::Value::Array(temp),
                    false
                ))
        );

        let mut temp = Vec::new();
        temp.push(ast::Value::Expr(ast::Expr::Number(1)));
        temp.push(ast::Value::Expr(ast::Expr::Number(2)));
        temp.push(ast::Value::Expr(ast::Expr::Number(4)));

        assert!(
            binding_assignment("let mut x = [1,2, 4];").unwrap().1
                == ast::Command::Binding(ast::Binding::Assignment(
                    ast::Variable::Named("x".to_string()),
                    ast::Type::Unknown,
                    ast::Value::Array(temp),
                    true
                ))
        );

        let mut temp = Vec::new();
        temp.push(ast::Value::Expr(ast::Expr::Number(1)));
        temp.push(ast::Value::Expr(ast::Expr::Number(2)));
        temp.push(ast::Value::Expr(ast::Expr::Number(4)));

        assert!(
            binding_assignment("let mut x = [1,2, 4];").unwrap().1
                == ast::Command::Binding(ast::Binding::Assignment(
                    ast::Variable::Named("x".to_string()),
                    ast::Type::Unknown,
                    ast::Value::Array(temp),
                    true
                ))
        );
    }

    #[test]
    fn binding_assigment3() {
        let mut temp = Vec::new();
        temp.push(ast::Value::Expr(ast::Expr::Number(1)));
        temp.push(ast::Value::Expr(ast::Expr::Number(2)));
        temp.push(ast::Value::Expr(ast::Expr::Number(4)));

        assert!(
            binding_assignment("let x: [i32; 3] = [1,2, 4];").unwrap().1
                == ast::Command::Binding(ast::Binding::Assignment(
                    ast::Variable::Named("x".to_string()),
                    ast::Type::Array(Box::new(ast::Type::I32), 3),
                    ast::Value::Array(temp),
                    false
                ))
        );
    }

    #[test]
    fn binding_assignment_tuple_unpack1() {
        assert!(
            binding_assignment_tuple_unpack("let (x,) = (12,);")
                .unwrap()
                .0
                == ""
        );
        assert!(
            binding_assignment_tuple_unpack("let (x,y) = (12, true);")
                .unwrap()
                .0
                == ""
        );
        assert!(
            binding_assignment_tuple_unpack("let (x,y, _) = (12, true, false);")
                .unwrap()
                .0
                == ""
        );
        let mut temp = Vec::new();
        temp.push(ast::Binding::Assignment(
            ast::Variable::Named("x".to_string()),
            ast::Type::Unknown,
            ast::Value::Expr(ast::Expr::Number(12)),
            false,
        ));
        temp.push(ast::Binding::Assignment(
            ast::Variable::Named("y".to_string()),
            ast::Type::Unknown,
            ast::Value::Bool(ast::Bool::True),
            false,
        ));
        temp.push(ast::Binding::Assignment(
            ast::Variable::Empty,
            ast::Type::Unknown,
            ast::Value::Bool(ast::Bool::False),
            false,
        ));

        assert!(
            binding_assignment_tuple_unpack("let (x,y, _) = (12, true, false);")
                .unwrap()
                .1
                == ast::Command::TupleBinding(temp)
        );
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
        assert!(binding_declaration("let c: (i32, bool);").unwrap().0 == "");
    }

    #[test]
    fn _tuple1() {
        let mut first = Vec::new();
        first.push(ast::Value::Variable(ast::Variable::Named("a".to_string())));

        let mut second = Vec::new();
        second.push(ast::Value::Expr(ast::Expr::Number(1)));

        let mut third = Vec::new();
        third.push(ast::Value::Expr(ast::Expr::Number(1)));
        third.push(ast::Value::Expr(ast::Expr::Number(2)));

        let mut fourth = Vec::new();
        fourth.push(ast::Value::Variable(ast::Variable::Named("a".to_string())));
        fourth.push(ast::Value::Variable(ast::Variable::Named("e".to_string())));

        let mut fifth = Vec::new();
        fifth.push(ast::Value::Variable(ast::Variable::Named("a".to_string())));
        fifth.push(ast::Value::Variable(ast::Variable::Named("e".to_string())));
        fifth.push(ast::Value::Expr(ast::Expr::Number(1)));

        let mut sixth = Vec::new();
        sixth.push(ast::Value::Expr(ast::Expr::Number(1)));
        sixth.push(ast::Value::Variable(ast::Variable::Named("a".to_string())));

        let mut seventh = Vec::new();
        seventh.push(ast::Value::Expr(ast::Expr::Number(1)));
        seventh.push(ast::Value::Expr(ast::Expr::Number(2)));
        seventh.push(ast::Value::Variable(ast::Variable::Named("a".to_string())));

        assert!(_tuple("(a)").is_err());
        assert!(_tuple("(a,)").unwrap().1 == ast::Value::Tuple(first));
        assert!(_tuple("(a, false)").unwrap().0 == "");
        assert!(_tuple("(1,)") == Ok(("", ast::Value::Tuple(second))));
        assert!(_tuple("(1,2)") == Ok(("", ast::Value::Tuple(third))));
        assert!(_tuple("(a, e)").unwrap().1 == ast::Value::Tuple(fourth));
        assert!(_tuple("(a, e, 1)").unwrap().1 == ast::Value::Tuple(fifth));
        assert!(_tuple("(1, a)").unwrap().1 == ast::Value::Tuple(sixth));
        assert!(_tuple("(1, 2, a)").unwrap().1 == ast::Value::Tuple(seventh));
    }

    #[test]
    fn _tuple_type1() {
        let mut t = Vec::new();
        t.push(ast::Type::I32);
        t.push(ast::Type::Bool);
        t.push(ast::Type::I32);

        assert!(_tuple_type("(i32)").is_err());
        assert!(_tuple_type("(i32,)").is_ok());
        assert!(_tuple_type("(i32, bool, i32)").unwrap().1 == ast::Type::Tuple(t));
    }

    #[test]
    fn _tuple_unpack_left1() {
        assert!(_tuple_unpack_left("(true)").is_err());
        assert!(_tuple_unpack_left("(t, a)").unwrap().0 == "");
        assert!(_tuple_unpack_left("(t, a, _)").unwrap().0 == "");
    }
}
