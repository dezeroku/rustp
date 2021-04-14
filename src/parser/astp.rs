use crate::ast;
use crate::parser::boolean;
use crate::parser::math;

use nom::{
    branch::alt, bytes::complete::tag, bytes::complete::take_until, bytes::complete::take_while1,
    character::complete::char, character::complete::multispace0, character::complete::newline,
    character::complete::space0, character::complete::space1, combinator::not, combinator::opt,
    multi::many0, multi::many1, sequence::tuple, IResult,
};

static KEYWORDS: [&'static str; 7] = ["let", "true", "false", "&&", "||", "!", "_"];

pub fn program(input: &str) -> IResult<&str, ast::Program> {
    many1(function)(input)
        .and_then(|(next_input, res)| Ok((next_input, ast::Program { content: res })))
}

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

fn precondition(input: &str) -> IResult<&str, ast::Bool> {
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

fn postcondition(input: &str) -> IResult<&str, ast::Bool> {
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

fn function(input: &str) -> IResult<&str, ast::Function> {
    tuple((
        //many0(tuple((multispace0, comments, multispace0))),
        opt(precondition),
        multispace0,
        opt(postcondition),
        multispace0,
        tag("fn"),
        space1,
        function_name,
        space0,
        tuple((tag("("), space0, function_inputs, space0, tag(")"))),
        space0,
        opt(tuple((tag("->"), space0, type_def))),
        space0,
        tag("{"),
        space0,
        block,
        multispace0,
        opt(r_value),
        multispace0,
        tag("}"),
        multispace0,
    ))(input)
    .and_then(|(next_input, res)| {
        let (
            //   _,
            pre,
            _,
            post,
            _,
            _,
            _,
            name,
            _,
            (_, _, inputs, _, _),
            _,
            out,
            _,
            _,
            _,
            comms,
            _,
            ret,
            _,
            _,
            _,
        ) = res;

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

        let ret_val = match ret {
            Some(a) => a,
            None => ast::Value::Unit,
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
                return_value: ret_val,
            },
        ))
    })
}

fn block(input: &str) -> IResult<&str, Vec<ast::Command>> {
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
        alt((debug_comment, single_comment, multiline_comment)),
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

fn debug_comment(input: &str) -> IResult<&str, &str> {
    not(command)(input).and_then(|(next_input, _)| {
        tuple((
            prove_start,
            tag("debug"),
            newline,
            take_while_not_newline,
            opt(newline),
        ))(next_input)
        .and_then(|(next_input, res)| {
            let (_, _, _, c, _) = res;
            Ok((next_input, c))
        })
    })
}

fn single_comment(input: &str) -> IResult<&str, &str> {
    not(command)(input).and_then(|(next_input, _)| {
        not(prove_start)(input).and_then(|(next_input, _)| {
            tuple((tag("//"), take_while_not_newline, opt(newline)))(next_input).and_then(
                |(next_input, res)| {
                    let (_, c, _) = res;
                    Ok((next_input, c))
                },
            )
        })
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
        if_else,
        while_parse,
        for_parse,
    ))(input)
}

fn assignment(input: &str) -> IResult<&str, ast::Command> {
    alt((
        assignment_tuple_unpack,
        assignment_tuple_single,
        assignment_single,
    ))(input)
}

fn assignment_single(input: &str) -> IResult<&str, ast::Command> {
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
            tuple_values,
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
        tuple((
            space0,
            variable,
            space0,
            tag("="),
            space0,
            reference,
            space0,
            tag(";"),
        )),
        tuple((
            space0,
            variable,
            space0,
            tag("="),
            space0,
            reference_mut,
            space0,
            tag(";"),
        )),
        tuple((
            space0,
            variable,
            space0,
            tag("="),
            space0,
            dereference,
            space0,
            tag(";"),
        )),
    ))(input)
    .and_then(|(next_input, res)| {
        let (_, v, _, _, _, val, _, _) = res;
        Ok((
            next_input,
            ast::Command::Assignment(ast::Assignment::Single(v, val)),
        ))
    })
}

fn assignment_tuple_unpack(input: &str) -> IResult<&str, ast::Command> {
    tuple((
        space0,
        tuple_unpack_left,
        space0,
        tuple((char('='), space0, tuple_values, space0, char(';'))),
    ))(input)
    .and_then(|(next_input, x)| {
        let (_, v, _, tu) = x;
        if let (_, _, ast::Value::Tuple(exp), _, _) = tu {
            let mut result = Vec::new();
            for (var, val) in itertools::izip!(v, exp) {
                result.push(ast::Assignment::Single(var, val));
            }

            Ok((
                next_input,
                ast::Command::Assignment(ast::Assignment::Tuple(result)),
            ))
        } else {
            panic!("Incorrect case")
        }
    })
}

fn assignment_tuple_single(input: &str) -> IResult<&str, ast::Command> {
    tuple((
        space0,
        tuple_unpack_left,
        space0,
        tuple((char('='), space0, variable_single, space0, char(';'))),
    ))(input)
    .and_then(|(next_input, x)| {
        let (_, v, _, tu) = x;
        if let (_, _, ast::Variable::Named(name), _, _) = tu {
            let mut result = Vec::new();
            let mut indexes = Vec::new();
            for i in 0..v.len() {
                indexes.push(i as i32);
            }
            for (var, i) in itertools::izip!(v, indexes) {
                result.push(ast::Assignment::Single(
                    var,
                    ast::Value::Variable(ast::Variable::TupleElem(
                        name.clone(),
                        Box::new(ast::Value::Expr(ast::Expr::Number(i))),
                    )),
                ));
            }

            Ok((
                next_input,
                ast::Command::Assignment(ast::Assignment::Tuple(result)),
            ))
        } else {
            panic!("Incorrect case")
        }
    })
}

fn for_parse(input: &str) -> IResult<&str, ast::Command> {
    tuple((
        tag("for"),
        space1,
        variable_single,
        space1,
        tag("in"),
        space1,
        r_value,
        space0,
        tag(".."),
        space0,
        r_value,
        space0,
        tag("{"),
        multispace0,
        block,
        multispace0,
        tag("}"),
    ))(input)
    .and_then(
        |(next_input, (_, _, iter, _, _, _, start, _, _, _, end, _, _, _, comms, _, _))| {
            Ok((
                next_input,
                ast::Command::Block(ast::Block::ForRange(iter, start, end, comms)),
            ))
        },
    )
}

fn while_parse(input: &str) -> IResult<&str, ast::Command> {
    tuple((
        tag("while"),
        space1,
        boolean::expr,
        space0,
        tag("{"),
        multispace0,
        block,
        multispace0,
        tag("}"),
    ))(input)
    .and_then(|(next_input, (_, _, c, _, _, _, comms, _, _))| {
        Ok((
            next_input,
            ast::Command::Block(ast::Block::While(*c, comms)),
        ))
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

fn function_name(input: &str) -> IResult<&str, &str> {
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
            Err(nom::Err::Error(nom::error::Error::new(
                res,
                nom::error::ErrorKind::Tag,
            )))
        }
    })
}

/// Defines everything what can be used on the right side of an assignment or binding.
fn r_value(input: &str) -> IResult<&str, ast::Value> {
    // TODO: this will be problematic due to possibility of boolean and math expr_val consuming same input, just in a different level
    // handle it somehow based on the length of input matched?
    alt((
        dereference,
        tuple_values,
        function_call,
        variable_val,
        math::expr_val,
        boolean::expr_val,
        reference_mut,
        reference,
    ))(input)
}

fn reference(input: &str) -> IResult<&str, ast::Value> {
    tuple((tag("&"), space0, r_value))(input)
        .and_then(|(next_input, (_, _, r))| Ok((next_input, ast::Value::Reference(Box::new(r)))))
}

fn reference_mut(input: &str) -> IResult<&str, ast::Value> {
    tuple((tag("&mut"), space1, r_value))(input).and_then(|(next_input, (_, _, r))| {
        Ok((next_input, ast::Value::ReferenceMutable(Box::new(r))))
    })
}

fn dereference(input: &str) -> IResult<&str, ast::Value> {
    tuple((tag("*"), space0, variable_val))(input)
        .and_then(|(next_input, (_, _, r))| Ok((next_input, ast::Value::Dereference(Box::new(r)))))
}

pub fn function_call(input: &str) -> IResult<&str, ast::Value> {
    tuple((
        function_name,
        space0,
        tag("("),
        space0,
        opt(tuple((
            r_value,
            many0(tuple((space0, tag(","), space0, r_value))),
        ))),
        space0,
        tag(")"),
    ))(input)
    .and_then(|(next_input, res)| {
        let (n, _, _, _, x, _, _) = res;
        match x {
            Some((v, a)) => {
                let mut t = Vec::new();
                t.push(v);

                for item in a {
                    let (_, _, _, i) = item;
                    t.push(i);
                }

                Ok((next_input, ast::Value::FunctionCall(n.to_string(), t)))
            }
            None => Ok((
                next_input,
                ast::Value::FunctionCall(n.to_string(), Vec::new()),
            )),
        }
    })
}

pub fn variable_val(input: &str) -> IResult<&str, ast::Value> {
    variable(input).and_then(|(next_input, res)| Ok((next_input, ast::Value::Variable(res))))
}

fn variable(input: &str) -> IResult<&str, ast::Variable> {
    alt((variable_tuple_elem, variable_array_elem, variable_single))(input)
}

fn variable_single(input: &str) -> IResult<&str, ast::Variable> {
    variable_name(input)
        .and_then(|(next_input, res)| Ok((next_input, ast::Variable::Named(res.to_string()))))
}

fn variable_array_elem(input: &str) -> IResult<&str, ast::Variable> {
    tuple((variable_name, char('['), space0, r_value, space0, char(']')))(input).and_then(
        |(next_input, (v, _, _, i, _, _))| {
            Ok((
                next_input,
                ast::Variable::ArrayElem(v.to_string(), Box::new(i)),
            ))
        },
    )
}

fn variable_tuple_elem(input: &str) -> IResult<&str, ast::Variable> {
    tuple((variable_name, char('.'), r_value))(input).and_then(|(next_input, (v, _, i))| {
        Ok((
            next_input,
            ast::Variable::TupleElem(v.to_string(), Box::new(i)),
        ))
    })
}

fn binding(input: &str) -> IResult<&str, ast::Command> {
    alt((
        binding_assignment_tuple_multiple,
        binding_assignment_tuple_single,
        binding_assignment,
        binding_declaration_tuple,
        binding_declaration,
    ))(input)
}

fn binding_assignment_tuple_single(input: &str) -> IResult<&str, ast::Command> {
    tuple((
        space0,
        tag("let"),
        space1,
        tuple((
            tag("("),
            space0,
            opt(tuple((tag("mut"), space1))),
            alt((variable, tuple_empty_elem)),
            space0,
            char(','),
            space0,
            opt(tuple((
                opt(tuple((tag("mut"), space1))),
                alt((variable, tuple_empty_elem)),
            ))),
            space0,
            many0(tuple((
                char(','),
                space0,
                opt(tuple((tag("mut"), space1))),
                alt((variable, tuple_empty_elem)),
                space0,
            ))),
            space0,
            tag(")"),
        )),
        space0,
        tuple((char(':'), space0, tuple_type, space0)),
        space0,
        tuple((char('='), space0, variable_single, space0, char(';'))),
    ))(input)
    .and_then(
        |(
            next_input,
            (_, _, _, (_, _, fm, f, _, _, _, second, _, rest, _, _), _, t, _, (_, _, temp, _, _)),
        )| {
            let name = match temp {
                ast::Variable::Named(x) => x,
                _ => panic!("Incorrect case"),
            };
            let mut vars = Vec::new();
            let mut muts = Vec::new();
            vars.push(f);
            muts.push(fm);

            match second {
                Some((sm, s)) => {
                    vars.push(s);
                    muts.push(sm);
                }
                None => {}
            }

            for (_, _, rm, r, _) in rest {
                vars.push(r);
                muts.push(rm);
            }

            let types = match t {
                (_, _, ast::Type::Tuple(x), _) => x,
                _ => {
                    panic!("Incorrect case")
                }
            };

            let mut result = Vec::new();
            let mut vals = Vec::new();
            for i in 0..vars.len() {
                vals.push(i as i32);
            }

            for (m, v, t, val) in itertools::izip!(muts, vars, types, vals) {
                let mu = match m {
                    Some(_) => true,
                    None => false,
                };
                result.push(ast::Command::Binding(ast::Binding::Assignment(
                    v,
                    t,
                    ast::Value::Variable(ast::Variable::TupleElem(
                        name.clone(),
                        Box::new(ast::Value::Expr(ast::Expr::Number(val))),
                    )),
                    mu,
                )));
            }

            Ok((
                next_input,
                ast::Command::Binding(ast::Binding::Tuple(result)),
            ))
        },
    )
}

fn binding_assignment_tuple_multiple(input: &str) -> IResult<&str, ast::Command> {
    tuple((
        space0,
        tag("let"),
        space1,
        tuple((
            tag("("),
            space0,
            opt(tuple((tag("mut"), space1))),
            alt((variable, tuple_empty_elem)),
            space0,
            char(','),
            space0,
            opt(tuple((
                opt(tuple((tag("mut"), space1))),
                alt((variable, tuple_empty_elem)),
            ))),
            space0,
            many0(tuple((
                char(','),
                space0,
                opt(tuple((tag("mut"), space1))),
                alt((variable, tuple_empty_elem)),
                space0,
            ))),
            space0,
            tag(")"),
        )),
        space0,
        tuple((char(':'), space0, tuple_type, space0)),
        space0,
        tuple((char('='), space0, tuple_values, space0, char(';'))),
    ))(input)
    .and_then(
        |(
            next_input,
            (_, _, _, (_, _, fm, f, _, _, _, second, _, rest, _, _), _, t, _, (_, _, temp, _, _)),
        )| {
            let vals = match temp {
                ast::Value::Tuple(x) => x,
                _ => panic!("Incorrect case"),
            };
            let mut vars = Vec::new();
            let mut muts = Vec::new();
            vars.push(f);
            muts.push(fm);

            match second {
                Some((sm, s)) => {
                    vars.push(s);
                    muts.push(sm);
                }
                None => {}
            }

            for (_, _, rm, r, _) in rest {
                vars.push(r);
                muts.push(rm);
            }

            let types = match t {
                (_, _, ast::Type::Tuple(x), _) => x,
                _ => {
                    panic!("Incorrect case")
                }
            };

            let mut result = Vec::new();
            for (m, v, t, val) in itertools::izip!(muts, vars, types, vals) {
                let mu = match m {
                    Some(_) => true,
                    None => false,
                };
                result.push(ast::Command::Binding(ast::Binding::Assignment(
                    v, t, val, mu,
                )));
            }

            Ok((
                next_input,
                ast::Command::Binding(ast::Binding::Tuple(result)),
            ))
        },
    )
}

fn binding_assignment(input: &str) -> IResult<&str, ast::Command> {
    // TODO: IMPORTANT: reference and reference mutable are not real types, handle these properly somehow
    tuple((
        space0,
        tag("let"),
        space1,
        opt(tuple((tag("mut"), space1))),
        variable,
        space0,
        alt((
            tuple((
                tuple((char(':'), space0, array_type, space0)),
                tuple((char('='), space0, array_content, space0, char(';'))),
            )),
            tuple((
                tuple((char(':'), space0, tuple_type, space0)),
                tuple((char('='), space0, tuple_values, space0, char(';'))),
            )),
            tuple((
                tuple((char(':'), space0, type_def_bool, space0)),
                tuple((char('='), space0, boolean::expr_val, space0, char(';'))),
            )),
            tuple((
                tuple((char(':'), space0, type_def_i32, space0)),
                tuple((char('='), space0, math::expr_val, space0, char(';'))),
            )),
            //tuple((
            //    tuple((char(':'), space0, type_def_reference, space0)),
            //    tuple((char('='), space0, reference, space0, char(';'))),
            //)),
            //tuple((
            //    tuple((char(':'), space0, type_def_reference_mut, space0)),
            //    tuple((char('='), space0, reference_mut, space0, char(';'))),
            //)),
            //tuple((
            //    tuple((char(':'), space0, type_def_single, space0)),
            //    tuple((char('='), space0, dereference, space0, char(';'))),
            //)),
        )),
    ))(input)
    .and_then(|(next_input, x)| {
        let (_, _, _, m, v, _, ((_, _, t, _), (_, _, exp, _, _))) = x;
        let mu = match m {
            Some(_) => true,
            None => false,
        };
        Ok((
            next_input,
            ast::Command::Binding(ast::Binding::Assignment(v, t, exp, mu)),
        ))
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
        tuple((char(':'), space0, type_def, space0)),
        space0,
        char(';'),
    ))(input)
    .and_then(|(next_input, x)| {
        let (_, _, _, m, v, _, (_, _, t, _), _, _) = x;
        let mu = match m {
            Some(_) => true,
            None => false,
        };
        Ok((
            next_input,
            ast::Command::Binding(ast::Binding::Declaration(v, t, mu)),
        ))
    })
}

fn binding_declaration_tuple(input: &str) -> IResult<&str, ast::Command> {
    tuple((
        space0,
        tag("let"),
        space1,
        tuple((
            tag("("),
            space0,
            opt(tuple((tag("mut"), space1))),
            alt((variable, tuple_empty_elem)),
            space0,
            char(','),
            space0,
            opt(tuple((
                opt(tuple((tag("mut"), space1))),
                alt((variable, tuple_empty_elem)),
            ))),
            space0,
            many0(tuple((
                char(','),
                space0,
                opt(tuple((tag("mut"), space1))),
                alt((variable, tuple_empty_elem)),
                space0,
            ))),
            space0,
            tag(")"),
        )),
        space0,
        tuple((char(':'), space0, tuple_type, space0)),
        space0,
        char(';'),
    ))(input)
    .and_then(
        |(
            next_input,
            (_, _, _, (_, _, fm, f, _, _, _, second, _, rest, _, _), _, (_, _, t, _), _, _),
        )| {
            let mut vars = Vec::new();
            let mut muts = Vec::new();
            vars.push(f);
            muts.push(fm);

            match second {
                Some((sm, s)) => {
                    vars.push(s);
                    muts.push(sm);
                }
                None => {}
            }

            for (_, _, rm, r, _) in rest {
                vars.push(r);
                muts.push(rm);
            }

            let types = match t {
                ast::Type::Tuple(x) => x,
                _ => panic!("Incorrect case!"),
            };

            let mut result = Vec::new();
            for (m, v, t) in itertools::izip!(muts, vars, types) {
                let mu = match m {
                    Some(_) => true,
                    None => false,
                };
                result.push(ast::Command::Binding(ast::Binding::Declaration(v, t, mu)));
            }

            Ok((
                next_input,
                ast::Command::Binding(ast::Binding::Tuple(result)),
            ))
        },
    )
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

fn tuple_empty_elem(input: &str) -> IResult<&str, ast::Variable> {
    tag("_")(input).and_then(|(next_input, _)| Ok((next_input, ast::Variable::Empty)))
}

fn tuple_unpack_left(input: &str) -> IResult<&str, Vec<ast::Variable>> {
    tuple((
        tag("("),
        space0,
        alt((variable, tuple_empty_elem)),
        space0,
        char(','),
        space0,
        opt(alt((variable, tuple_empty_elem))),
        space0,
        many0(tuple((
            char(','),
            space0,
            alt((variable, tuple_empty_elem)),
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

fn tuple_type(input: &str) -> IResult<&str, ast::Type> {
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

fn tuple_values(input: &str) -> IResult<&str, ast::Value> {
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

fn type_def_reference(input: &str) -> IResult<&str, bool> {
    tag("&")(input).and_then(|(next_input, _)| Ok((next_input, false)))
}

fn type_def_reference_mut(input: &str) -> IResult<&str, bool> {
    tag("&mut")(input).and_then(|(next_input, _)| Ok((next_input, true)))
}

fn type_def(input: &str) -> IResult<&str, ast::Type> {
    tuple((
        opt(tuple((
            alt((type_def_reference_mut, type_def_reference)),
            space0,
        ))),
        alt((array_type, tuple_type, type_def_single)),
    ))(input)
    .and_then(|(next_input, (p, t))| match p {
        Some((mutable, _)) => match mutable {
            true => Ok((next_input, ast::Type::ReferenceMutable(Box::new(t)))),
            false => Ok((next_input, ast::Type::Reference(Box::new(t)))),
        },
        None => Ok((next_input, t)),
    })
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
        assert!(function("fn a () {let a: i32 = 14;}").unwrap().0 == "");
        assert!(
            function("fn a () {let a: i32 = 14; let c: i32 = 1 + b;}")
                .unwrap()
                .0
                == ""
        );
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
            return_value: ast::Value::Unit,
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
            return_value: ast::Value::Unit,
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
            return_value: ast::Value::Unit,
        };

        assert!(a == b);
    }

    #[test]
    fn function4() {
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
        let a = function("//%precondition false && false\n//%postcondition (a == 12) && false\n fn a (c: i32, d: bool) -> bool {let a: i32 = 14; 13}")
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
            return_value: ast::Value::Expr(ast::Expr::Number(13)),
        };

        assert!(a == b);
    }

    #[test]
    fn type_def1() {
        assert!(type_def("i32").unwrap().1 == ast::Type::I32);
        assert!(type_def("bool").unwrap().1 == ast::Type::Bool);
        assert!(type_def("&i32").unwrap().1 == ast::Type::Reference(Box::new(ast::Type::I32)));
        assert!(
            type_def("&mut i32").unwrap().1
                == ast::Type::ReferenceMutable(Box::new(ast::Type::I32))
        );
    }

    #[test]
    fn block1() {
        assert!(block("let x: i32 = 1;").unwrap().0 == "");
        assert!(block("let x: i32 = 1;//%assert a < 143\n").unwrap().0 == "");
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
        assert!(command("let a: i32 = 14;").unwrap().0 == "");
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
    fn assignment_single1() {
        assert!(assignment_single("a = 12;").unwrap().0 == "");
        assert!(
            assignment_single("b = 12;").unwrap().1
                == ast::Command::Assignment(ast::Assignment::Single(
                    ast::Variable::Named("b".to_string()),
                    ast::Value::Expr(ast::Expr::Number(12))
                ))
        );
        assert!(
            assignment_single("b = true;").unwrap().1
                == ast::Command::Assignment(ast::Assignment::Single(
                    ast::Variable::Named("b".to_string()),
                    ast::Value::Bool(ast::Bool::True)
                ))
        );
        assert!(assignment_single("a = (12, false, a);").unwrap().0 == "");
    }

    #[test]
    fn assignment_single2() {
        assert!(assignment_single("a[1] = 12;").unwrap().0 == "");
        assert!(
            assignment_single("b[3] = 12;").unwrap().1
                == ast::Command::Assignment(ast::Assignment::Single(
                    ast::Variable::ArrayElem(
                        "b".to_string(),
                        Box::new(ast::Value::Expr(ast::Expr::Number(3)))
                    ),
                    ast::Value::Expr(ast::Expr::Number(12))
                ))
        );

        let mut temp = Vec::new();
        temp.push(ast::Value::Expr(ast::Expr::Number(12)));
        temp.push(ast::Value::Expr(ast::Expr::Number(3)));
        temp.push(ast::Value::Expr(ast::Expr::Number(1)));
        assert!(
            assignment_single("b = [12, 3, 1];").unwrap().1
                == ast::Command::Assignment(ast::Assignment::Single(
                    ast::Variable::Named("b".to_string(),),
                    ast::Value::Array(temp)
                ))
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
        temp.push(ast::Assignment::Single(
            ast::Variable::Named("x".to_string()),
            ast::Value::Expr(ast::Expr::Number(12)),
        ));
        temp.push(ast::Assignment::Single(
            ast::Variable::Named("y".to_string()),
            ast::Value::Bool(ast::Bool::True),
        ));
        temp.push(ast::Assignment::Single(
            ast::Variable::Empty,
            ast::Value::Bool(ast::Bool::False),
        ));

        assert!(
            assignment_tuple_unpack("(x,y, _) = (12, true, false);")
                .unwrap()
                .1
                == ast::Command::Assignment(ast::Assignment::Tuple(temp))
        );
    }

    #[test]
    fn if_else1() {
        assert!(if_else("if a == 12 {let b: i32 = 3;}").unwrap().0 == "");
        assert!(
            if_else("if a == 12 {let b: i32 = 3;//%assert b == 3\n}")
                .unwrap()
                .0
                == ""
        );
    }

    #[test]
    fn if_else2() {
        assert!(
            if_else("if a == 12 {let b: i32 = 3;} else if a == 13 {let b: i32 = 4;} else {let b: i32 = 1;}")
                .unwrap()
                .0
                == ""
        );
    }

    #[test]
    fn if_else3() {
        let a = if_else(
            "if a == 12 {let b: i32 = 3;} else if a == 13 {let b: i32 = 4;} else {let b: i32 = 1;}",
        )
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
            ast::Type::I32,
            ast::Value::Expr(ast::Expr::Number(3)),
            false,
        )));
        comms_2.push(ast::Command::Binding(ast::Binding::Assignment(
            ast::Variable::Named("b".to_string()),
            ast::Type::I32,
            ast::Value::Expr(ast::Expr::Number(4)),
            false,
        )));

        comms.push(comms_1);
        comms.push(comms_2);

        el.push(ast::Command::Binding(ast::Binding::Assignment(
            ast::Variable::Named("b".to_string()),
            ast::Type::I32,
            ast::Value::Expr(ast::Expr::Number(1)),
            false,
        )));

        let b = ast::Command::Block(ast::Block::If(conds, comms, el));

        assert!(a == b);
    }

    #[test]
    fn if_else4() {
        assert!(if_else("if a == 14 {\nlet c: i32 = 3;\n}").unwrap().0 == "");
        assert!(
            if_else(
                "if a == 14 {\nlet c: i32 = 3;\n} else if a == 13 {\n   let c: i32 = a + 43;\n}"
            )
            .unwrap()
            .0 == ""
        );
        assert!(if_else("if a == 14 {\nlet c: i32 = 3;\n} else if a == 13 {\n   let c: i32 = a + 43;\n} else {\n   let c: i32 = a + 123;\n}").unwrap().0 == "");
    }

    #[test]
    fn single_if1() {
        assert!(single_if("if a == 12 {let b: i32 = 3;}").unwrap().0 == "");
        assert!(
            single_if("if a == 12 {let b: i32 = 3;//%assert b == 3\n}")
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
        assert!(function_call("abba()").unwrap().0 == "");
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
    fn variable_5() {
        assert!(variable("a.").unwrap().0 == ".");
        assert!(variable("a.1").is_ok());
        assert!(variable("abc.a").unwrap().0 == "");
        assert!(
            variable("abc.c").unwrap().1
                == ast::Variable::TupleElem(
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
    fn variable_tuple_elem1() {
        assert!(variable_tuple_elem("a.").is_err());
        assert!(variable_tuple_elem("a.1").is_ok());
        assert!(variable_tuple_elem("abc.a").unwrap().0 == "");
        assert!(
            variable_tuple_elem("abc.c").unwrap().1
                == ast::Variable::TupleElem(
                    "abc".to_string(),
                    Box::new(ast::Value::Variable(ast::Variable::Named("c".to_string())))
                )
        );
    }

    #[test]
    fn binding_assignment1() {
        assert!(binding_assignment("let x: i32 = 12;").unwrap().0 == "");
        assert!(binding_assignment("let x: i32 = 12 * 4;").unwrap().0 == "");
        assert!(binding_assignment("let y: i32 = 12 - 5 * 6;").unwrap().0 == "");
        assert!(binding_assignment("let z = 3").is_err());
        assert!(binding_assignment("let z = 3;").is_err());
        assert_eq!(
            binding_assignment("let mut z: i32 = 3;").unwrap().1,
            ast::Command::Binding(ast::Binding::Assignment(
                ast::Variable::Named("z".to_string()),
                ast::Type::I32,
                ast::Value::Expr(ast::Expr::Number(3)),
                true
            ))
        );
        // This one will be validated later on?
        // Or can we just assume that the code we are getting is correct Rust?
        assert!(binding_assignment("let z: bool = 3;").is_err());
        assert_eq!(
            binding_assignment("let z: bool = true;").unwrap().1,
            ast::Command::Binding(ast::Binding::Assignment(
                ast::Variable::Named("z".to_string()),
                ast::Type::Bool,
                ast::Value::Bool(ast::Bool::True),
                false
            ))
        );
        assert_eq!(binding_assignment("let c: i32 = a + 43;").unwrap().0, "");
        assert_eq!(
            binding_assignment("let c: (i32, bool) = (12, false);")
                .unwrap()
                .0,
            ""
        );
    }

    #[test]
    fn binding_assigment2() {
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

        let mut temp = Vec::new();
        temp.push(ast::Value::Expr(ast::Expr::Number(1)));
        temp.push(ast::Value::Expr(ast::Expr::Number(2)));
        temp.push(ast::Value::Expr(ast::Expr::Number(4)));

        assert!(
            binding_assignment("let mut x: [i32; 3] = [1,2, 4];")
                .unwrap()
                .1
                == ast::Command::Binding(ast::Binding::Assignment(
                    ast::Variable::Named("x".to_string()),
                    ast::Type::Array(Box::new(ast::Type::I32), 3),
                    ast::Value::Array(temp),
                    true
                ))
        );

        let mut temp = Vec::new();
        temp.push(ast::Value::Expr(ast::Expr::Number(1)));
        temp.push(ast::Value::Expr(ast::Expr::Number(2)));
        temp.push(ast::Value::Expr(ast::Expr::Number(4)));

        assert!(
            binding_assignment("let mut x: [i32; 3] = [1,2, 4];")
                .unwrap()
                .1
                == ast::Command::Binding(ast::Binding::Assignment(
                    ast::Variable::Named("x".to_string()),
                    ast::Type::Array(Box::new(ast::Type::I32), 3),
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
    fn binding_assignment_tuple_single1() {
        let mut temp = Vec::new();
        temp.push(ast::Command::Binding(ast::Binding::Assignment(
            ast::Variable::Named("x".to_string()),
            ast::Type::I32,
            ast::Value::Variable(ast::Variable::TupleElem(
                "b".to_string(),
                Box::new(ast::Value::Expr(ast::Expr::Number(0))),
            )),
            true,
        )));
        temp.push(ast::Command::Binding(ast::Binding::Assignment(
            ast::Variable::Named("y".to_string()),
            ast::Type::Bool,
            ast::Value::Variable(ast::Variable::TupleElem(
                "b".to_string(),
                Box::new(ast::Value::Expr(ast::Expr::Number(1))),
            )),
            false,
        )));
        temp.push(ast::Command::Binding(ast::Binding::Assignment(
            ast::Variable::Empty,
            ast::Type::Bool,
            ast::Value::Variable(ast::Variable::TupleElem(
                "b".to_string(),
                Box::new(ast::Value::Expr(ast::Expr::Number(2))),
            )),
            false,
        )));

        assert!(
            binding_assignment_tuple_single("let (mut x,y, _): (i32, bool, bool) = b;")
                .unwrap()
                .1
                == ast::Command::Binding(ast::Binding::Tuple(temp))
        );
    }

    #[test]
    fn assignment_tuple_single1() {
        let mut temp = Vec::new();
        temp.push(ast::Assignment::Single(
            ast::Variable::Named("x".to_string()),
            ast::Value::Variable(ast::Variable::TupleElem(
                "b".to_string(),
                Box::new(ast::Value::Expr(ast::Expr::Number(0))),
            )),
        ));
        temp.push(ast::Assignment::Single(
            ast::Variable::Named("y".to_string()),
            ast::Value::Variable(ast::Variable::TupleElem(
                "b".to_string(),
                Box::new(ast::Value::Expr(ast::Expr::Number(1))),
            )),
        ));
        temp.push(ast::Assignment::Single(
            ast::Variable::Empty,
            ast::Value::Variable(ast::Variable::TupleElem(
                "b".to_string(),
                Box::new(ast::Value::Expr(ast::Expr::Number(2))),
            )),
        ));

        assert!(
            assignment_tuple_single("( x,y, _) = b;").unwrap().1
                == ast::Command::Assignment(ast::Assignment::Tuple(temp))
        );
    }

    #[test]
    fn binding_assignment_tuple_multiple1() {
        assert!(
            binding_assignment_tuple_multiple("let (x,): (i32,) = (12,);")
                .unwrap()
                .0
                == ""
        );
        assert!(
            binding_assignment_tuple_multiple("let (x,y): (i32, bool) = (12, true);")
                .unwrap()
                .0
                == ""
        );
        assert!(
            binding_assignment_tuple_multiple(
                "let (x,y, _): (i32, bool, bool) = (12, true, false);"
            )
            .unwrap()
            .0 == ""
        );
        let mut temp = Vec::new();
        temp.push(ast::Command::Binding(ast::Binding::Assignment(
            ast::Variable::Named("x".to_string()),
            ast::Type::I32,
            ast::Value::Expr(ast::Expr::Number(12)),
            true,
        )));
        temp.push(ast::Command::Binding(ast::Binding::Assignment(
            ast::Variable::Named("y".to_string()),
            ast::Type::Bool,
            ast::Value::Bool(ast::Bool::True),
            false,
        )));
        temp.push(ast::Command::Binding(ast::Binding::Assignment(
            ast::Variable::Empty,
            ast::Type::Bool,
            ast::Value::Bool(ast::Bool::False),
            false,
        )));

        assert_eq!(
            binding_assignment_tuple_multiple(
                "let (mut x,y, _): (i32, bool, bool) = (12, true, false);"
            )
            .unwrap()
            .1,
            ast::Command::Binding(ast::Binding::Tuple(temp))
        );

        let mut temp = Vec::new();
        temp.push(ast::Command::Binding(ast::Binding::Assignment(
            ast::Variable::Named("x".to_string()),
            ast::Type::I32,
            ast::Value::Expr(ast::Expr::Number(12)),
            true,
        )));
        temp.push(ast::Command::Binding(ast::Binding::Assignment(
            ast::Variable::Named("y".to_string()),
            ast::Type::Bool,
            ast::Value::Bool(ast::Bool::True),
            false,
        )));
        temp.push(ast::Command::Binding(ast::Binding::Assignment(
            ast::Variable::Empty,
            ast::Type::Bool,
            ast::Value::Bool(ast::Bool::False),
            false,
        )));

        assert!(
            binding_assignment_tuple_multiple(
                "let (mut x,y, _): (i32, bool, bool) = (12, true, false);"
            )
            .unwrap()
            .1 == ast::Command::Binding(ast::Binding::Tuple(temp))
        );
    }

    #[test]
    fn binding_declaration1() {
        assert!(binding_declaration("let x: i32;").unwrap().0 == "");
        assert!(binding_declaration("let x: i32;").unwrap().0 == "");
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
            binding_declaration("let x: i32;").unwrap().1
                == ast::Command::Binding(ast::Binding::Declaration(
                    ast::Variable::Named("x".to_string()),
                    ast::Type::I32,
                    false
                ))
        );
        assert!(binding_declaration("let c: (i32, bool);").unwrap().0 == "");
    }

    #[test]
    fn binding_declaration_tuple1() {
        let mut temp = Vec::new();
        temp.push(ast::Command::Binding(ast::Binding::Declaration(
            ast::Variable::Named("a".to_string()),
            ast::Type::I32,
            false,
        )));
        temp.push(ast::Command::Binding(ast::Binding::Declaration(
            ast::Variable::Named("b".to_string()),
            ast::Type::I32,
            true,
        )));
        assert_eq!(
            binding_declaration_tuple("let (a, mut b): (i32, i32);")
                .unwrap()
                .1,
            ast::Command::Binding(ast::Binding::Tuple(temp))
        );

        let mut temp = Vec::new();
        temp.push(ast::Command::Binding(ast::Binding::Declaration(
            ast::Variable::Named("a".to_string()),
            ast::Type::I32,
            false,
        )));
        temp.push(ast::Command::Binding(ast::Binding::Declaration(
            ast::Variable::Named("b".to_string()),
            ast::Type::Bool,
            true,
        )));
        assert_eq!(
            binding_declaration_tuple("let (a, mut b): (i32, bool);")
                .unwrap()
                .1,
            ast::Command::Binding(ast::Binding::Tuple(temp))
        );
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

        assert!(tuple_values("(a)").is_err());
        assert!(tuple_values("(a,)").unwrap().1 == ast::Value::Tuple(first));
        assert!(tuple_values("(a, false)").unwrap().0 == "");
        assert!(tuple_values("(1,)") == Ok(("", ast::Value::Tuple(second))));
        assert!(tuple_values("(1,2)") == Ok(("", ast::Value::Tuple(third))));
        assert!(tuple_values("(a, e)").unwrap().1 == ast::Value::Tuple(fourth));
        assert!(tuple_values("(a, e, 1)").unwrap().1 == ast::Value::Tuple(fifth));
        assert!(tuple_values("(1, a)").unwrap().1 == ast::Value::Tuple(sixth));
        assert!(tuple_values("(1, 2, a)").unwrap().1 == ast::Value::Tuple(seventh));
    }

    #[test]
    fn tuple_type1() {
        let mut t = Vec::new();
        t.push(ast::Type::I32);
        t.push(ast::Type::Bool);
        t.push(ast::Type::I32);

        assert!(tuple_type("(i32)").is_err());
        assert!(tuple_type("(i32,)").is_ok());
        assert!(tuple_type("(i32, bool, i32)").unwrap().1 == ast::Type::Tuple(t));
    }

    #[test]
    fn tuple_unpack_left1() {
        assert!(tuple_unpack_left("(true)").is_err());
        assert!(tuple_unpack_left("(t, a)").unwrap().0 == "");
        assert!(tuple_unpack_left("(t, a, _)").unwrap().0 == "");
    }

    #[test]
    fn while_parse1() {
        assert!(
            while_parse("while i {}").unwrap().1
                == ast::Command::Block(ast::Block::While(
                    ast::Bool::Value(Box::new(ast::Value::Variable(ast::Variable::Named(
                        String::from("i")
                    )))),
                    Vec::new()
                ))
        );

        let mut temp = Vec::new();
        temp.push(ast::Command::Binding(ast::Binding::Assignment(
            ast::Variable::Named("x".to_string()),
            ast::Type::I32,
            ast::Value::Expr(ast::Expr::Number(1)),
            false,
        )));

        assert_eq!(
            while_parse("while true {let x: i32 = 1;}").unwrap().1,
            ast::Command::Block(ast::Block::While(ast::Bool::True, temp))
        );
    }

    #[test]
    fn for_parse1() {
        assert!(
            for_parse("for i in 0..2 {}").unwrap().1
                == ast::Command::Block(ast::Block::ForRange(
                    ast::Variable::Named("i".to_string()),
                    ast::Value::Expr(ast::Expr::Number(0)),
                    ast::Value::Expr(ast::Expr::Number(2)),
                    Vec::new()
                ))
        );

        assert!(
            for_parse("for i in 0..b {}").unwrap().1
                == ast::Command::Block(ast::Block::ForRange(
                    ast::Variable::Named("i".to_string()),
                    ast::Value::Expr(ast::Expr::Number(0)),
                    ast::Value::Variable(ast::Variable::Named("b".to_string())),
                    Vec::new()
                ))
        );

        let mut temp = Vec::new();
        temp.push(ast::Command::Binding(ast::Binding::Assignment(
            ast::Variable::Named("x".to_string()),
            ast::Type::I32,
            ast::Value::Expr(ast::Expr::Number(1)),
            false,
        )));

        assert_eq!(
            for_parse("for i in 0..b {let x: i32 = 1;}").unwrap().1,
            ast::Command::Block(ast::Block::ForRange(
                ast::Variable::Named("i".to_string()),
                ast::Value::Expr(ast::Expr::Number(0)),
                ast::Value::Variable(ast::Variable::Named("b".to_string())),
                temp
            ))
        );

        let mut temp = Vec::new();
        temp.push(ast::Command::Binding(ast::Binding::Assignment(
            ast::Variable::Named("x".to_string()),
            ast::Type::I32,
            ast::Value::Expr(ast::Expr::Number(1)),
            false,
        )));

        temp.push(ast::Command::Binding(ast::Binding::Assignment(
            ast::Variable::Named("y".to_string()),
            ast::Type::Bool,
            ast::Value::Bool(ast::Bool::True),
            false,
        )));

        assert!(
            for_parse("for i in 0..b {let x: i32 = 1; let y: bool = true;}")
                .unwrap()
                .1
                == ast::Command::Block(ast::Block::ForRange(
                    ast::Variable::Named("i".to_string()),
                    ast::Value::Expr(ast::Expr::Number(0)),
                    ast::Value::Variable(ast::Variable::Named("b".to_string())),
                    temp
                ))
        );
    }

    #[test]
    fn reference1() {
        assert_eq!(
            reference("&b").unwrap().1,
            ast::Value::Reference(Box::new(ast::Value::Variable(ast::Variable::Named(
                String::from("b")
            ))))
        );
        assert_eq!(
            reference("&b.1").unwrap().1,
            ast::Value::Reference(Box::new(ast::Value::Variable(ast::Variable::TupleElem(
                String::from("b"),
                Box::new(ast::Value::Expr(ast::Expr::Number(1)))
            ))))
        );
        assert_eq!(
            reference("&b[0]").unwrap().1,
            ast::Value::Reference(Box::new(ast::Value::Variable(ast::Variable::ArrayElem(
                String::from("b"),
                Box::new(ast::Value::Expr(ast::Expr::Number(0)))
            ))))
        );
    }

    #[test]
    fn reference_mut1() {
        assert_eq!(
            reference_mut("&mut b").unwrap().1,
            ast::Value::ReferenceMutable(Box::new(ast::Value::Variable(ast::Variable::Named(
                String::from("b")
            ))))
        );
        assert_eq!(
            reference_mut("&mut b.1").unwrap().1,
            ast::Value::ReferenceMutable(Box::new(ast::Value::Variable(ast::Variable::TupleElem(
                String::from("b"),
                Box::new(ast::Value::Expr(ast::Expr::Number(1)))
            ))))
        );
        assert_eq!(
            reference_mut("&mut b[0]").unwrap().1,
            ast::Value::ReferenceMutable(Box::new(ast::Value::Variable(ast::Variable::ArrayElem(
                String::from("b"),
                Box::new(ast::Value::Expr(ast::Expr::Number(0)))
            ))))
        );
    }

    #[test]
    fn dereference1() {
        assert_eq!(
            dereference("*b").unwrap().1,
            ast::Value::Dereference(Box::new(ast::Value::Variable(ast::Variable::Named(
                String::from("b")
            ))))
        );
        assert_eq!(
            dereference("*b.1").unwrap().1,
            ast::Value::Dereference(Box::new(ast::Value::Variable(ast::Variable::TupleElem(
                String::from("b"),
                Box::new(ast::Value::Expr(ast::Expr::Number(1)))
            ))))
        );
        assert_eq!(
            dereference("*b[0]").unwrap().1,
            ast::Value::Dereference(Box::new(ast::Value::Variable(ast::Variable::ArrayElem(
                String::from("b"),
                Box::new(ast::Value::Expr(ast::Expr::Number(0)))
            ))))
        );
    }
}
