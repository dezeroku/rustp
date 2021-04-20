use crate::ast;
use crate::ast::PreconditionCreator;
use crate::parser::boolean;
use crate::parser::math;

use nom::{
    branch::alt, bytes::complete::tag, bytes::complete::take_until, bytes::complete::take_while1,
    character::complete::char, character::complete::multispace0, character::complete::newline,
    character::complete::space0, character::complete::space1, combinator::not, combinator::opt,
    combinator::recognize, multi::many0, multi::many1, sequence::tuple, IResult,
};

#[cfg(test)]
mod tests;

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
        type_def_function,
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
            }
            .update_precondition(),
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
    not(command)(input).and_then(|(_, _)| {
        not(prove_start)(input).and_then(|(_, _)| {
            tuple((tag("//"), take_while_not_newline, opt(newline)))(input).and_then(
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
        loop_invariant,
        space0,
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
        |(next_input, (inv, _, _, _, iter, _, _, _, start, _, _, _, end, _, _, _, comms, _, _))| {
            Ok((
                next_input,
                ast::Command::Block(ast::Block::ForRange(iter, start, end, comms, inv)),
            ))
        },
    )
}

fn while_parse(input: &str) -> IResult<&str, ast::Command> {
    tuple((
        loop_invariant,
        space0,
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
    .and_then(|(next_input, (inv, _, _, _, c, _, _, _, comms, _, _))| {
        Ok((
            next_input,
            ast::Command::Block(ast::Block::While(*c, comms, inv)),
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
    assert(input)
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

fn loop_invariant(input: &str) -> IResult<&str, ast::Bool> {
    tuple((
        prove_start,
        tag("invariant"),
        space1,
        boolean::expr,
        newline,
    ))(input)
    .map(|(next_input, res)| {
        let (_, _, _, a, _) = res;
        (next_input, *a)
    })
}

fn prove_start(input: &str) -> IResult<&str, &str> {
    tag("//%")(input)
}

fn function_name(input: &str) -> IResult<&str, &str> {
    let l = |x: char| char::is_alphabetic(x) || char::is_numeric(x) || '_' == x;

    take_while1(l)(input).and_then(|(next_input, res)| Ok((next_input, res)))
}

fn variable_name(input: &str) -> IResult<&str, &str> {
    let l = |x: char| char::is_alphabetic(x) || '_' == x;

    recognize(tuple((take_while1(l), opt(tag("'old")))))(input).and_then(|(next_input, res)| {
        // if matches then fail
        if !KEYWORDS.contains(&res) {
            // check for the 'old case
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

pub fn variable_single(input: &str) -> IResult<&str, ast::Variable> {
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

fn array_slice_type(input: &str) -> IResult<&str, ast::Type> {
    tuple((
        char('&'),
        space0,
        char('['),
        space0,
        type_def_single,
        space0,
        char(']'),
    ))(input)
    .and_then(|(next_input, (_, _, _, _, t, _, _))| {
        Ok((next_input, ast::Type::ArraySlice(Box::new(t))))
    })
}

fn type_def_reference(input: &str) -> IResult<&str, bool> {
    tag("&")(input).and_then(|(next_input, _)| Ok((next_input, false)))
}

fn type_def_reference_mut(input: &str) -> IResult<&str, bool> {
    tag("&mut")(input).and_then(|(next_input, _)| Ok((next_input, true)))
}

fn type_def_function(input: &str) -> IResult<&str, ast::Type> {
    alt((array_slice_type, type_def))(input)
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
