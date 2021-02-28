use crate::ast;
use crate::parser::astp;
use crate::parser::math;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::space0, multi::many0, sequence::tuple,
    IResult,
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
    alt((factor_compare, factor_id, factor_not, factor_paren))(input)
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
            Ok((next_input, Box::new(ast::Bool::Greater(*a, *b))))
        },
    )
}

fn factor_compare_smaller_equal(input: &str) -> IResult<&str, Box<ast::Bool>> {
    tuple((math::expr, space0, tag("<="), space0, math::expr))(input).and_then(
        |(next_input, res)| {
            let (a, _, _, _, b) = res;
            Ok((next_input, Box::new(ast::Bool::SmallerEqual(*a, *b))))
        },
    )
}
fn factor_compare_smaller(input: &str) -> IResult<&str, Box<ast::Bool>> {
    tuple((math::expr, space0, tag("<"), space0, math::expr))(input).and_then(
        |(next_input, res)| {
            let (a, _, _, _, b) = res;
            Ok((next_input, Box::new(ast::Bool::Smaller(*a, *b))))
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mult_expr_right1() {
        assert!(mult_expr_right("&& true").unwrap().0 == "");
    }

    #[test]
    fn mult_expr_right2() {
        assert!(mult_expr_right(" && true").unwrap().0 == "");
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

    #[test]
    fn expr3() {
        assert!(expr("true && (a == (b + 3))").unwrap().0 == "");
        assert!(
            expr("true && (a == (b + 3))").unwrap().1
                == Box::new(ast::Bool::And(
                    Box::new(ast::Bool::True),
                    Box::new(ast::Bool::Equal(
                        ast::Expr::Value(Box::new(ast::Value::Variable(ast::Variable::Named(
                            "a".to_string()
                        )))),
                        ast::Expr::Op(
                            Box::new(ast::Expr::Value(Box::new(ast::Value::Variable(
                                ast::Variable::Named("b".to_string())
                            )))),
                            ast::Opcode::Add,
                            Box::new(ast::Expr::Number(3))
                        )
                    ))
                ))
        );
    }

    #[test]
    fn factor_id_1() {
        assert!(factor_id("true").unwrap().1 == Box::new(ast::Bool::True));
        assert!(factor_id("false").unwrap().1 == Box::new(ast::Bool::False));
    }

    #[test]
    fn factor_compare_equal1() {
        assert!(
            factor_compare_equal("12 == 43").unwrap().1
                == Box::new(ast::Bool::Equal(
                    ast::Expr::Number(12),
                    ast::Expr::Number(43)
                ))
        );

        assert!(
            factor_compare_equal("a == (b + 3)").unwrap().1
                == Box::new(ast::Bool::Equal(
                    ast::Expr::Value(Box::new(ast::Value::Variable(ast::Variable::Named(
                        "a".to_string()
                    )))),
                    ast::Expr::Op(
                        Box::new(ast::Expr::Value(Box::new(ast::Value::Variable(
                            ast::Variable::Named("b".to_string())
                        )))),
                        ast::Opcode::Add,
                        Box::new(ast::Expr::Number(3))
                    )
                ))
        );
    }

    #[test]
    fn factor_compare_greater_equal1() {
        assert!(
            factor_compare_greater_equal("12 >= 43").unwrap().1
                == Box::new(ast::Bool::GreaterEqual(
                    ast::Expr::Number(12),
                    ast::Expr::Number(43)
                ))
        );
        assert!(
            factor_compare_greater_equal("a >= (b + 3)").unwrap().1
                == Box::new(ast::Bool::GreaterEqual(
                    ast::Expr::Value(Box::new(ast::Value::Variable(ast::Variable::Named(
                        "a".to_string()
                    )))),
                    ast::Expr::Op(
                        Box::new(ast::Expr::Value(Box::new(ast::Value::Variable(
                            ast::Variable::Named("b".to_string())
                        )))),
                        ast::Opcode::Add,
                        Box::new(ast::Expr::Number(3))
                    )
                ))
        );
    }

    #[test]
    fn factor_compare_smaller_equal1() {
        assert!(
            factor_compare_smaller_equal("12 <= 43").unwrap().1
                == Box::new(ast::Bool::SmallerEqual(
                    ast::Expr::Number(12),
                    ast::Expr::Number(43)
                ))
        );

        assert!(
            factor_compare_smaller_equal("a <= (b + 3)").unwrap().1
                == Box::new(ast::Bool::SmallerEqual(
                    ast::Expr::Value(Box::new(ast::Value::Variable(ast::Variable::Named(
                        "a".to_string()
                    )))),
                    ast::Expr::Op(
                        Box::new(ast::Expr::Value(Box::new(ast::Value::Variable(
                            ast::Variable::Named("b".to_string())
                        )))),
                        ast::Opcode::Add,
                        Box::new(ast::Expr::Number(3))
                    )
                ))
        );
    }

    #[test]
    fn factor_compare_greater1() {
        assert!(
            factor_compare_greater("12 > 43").unwrap().1
                == Box::new(ast::Bool::Greater(
                    ast::Expr::Number(12),
                    ast::Expr::Number(43)
                ))
        );

        assert!(
            factor_compare_greater("a > (b + 3)").unwrap().1
                == Box::new(ast::Bool::Greater(
                    ast::Expr::Value(Box::new(ast::Value::Variable(ast::Variable::Named(
                        "a".to_string()
                    )))),
                    ast::Expr::Op(
                        Box::new(ast::Expr::Value(Box::new(ast::Value::Variable(
                            ast::Variable::Named("b".to_string())
                        )))),
                        ast::Opcode::Add,
                        Box::new(ast::Expr::Number(3))
                    )
                ))
        );
    }

    #[test]
    fn factor_compare_smaller1() {
        assert!(
            factor_compare_smaller("12 < 43").unwrap().1
                == Box::new(ast::Bool::Smaller(
                    ast::Expr::Number(12),
                    ast::Expr::Number(43)
                ))
        );

        assert!(
            factor_compare_smaller("a < (b + 3)").unwrap().1
                == Box::new(ast::Bool::Smaller(
                    ast::Expr::Value(Box::new(ast::Value::Variable(ast::Variable::Named(
                        "a".to_string()
                    )))),
                    ast::Expr::Op(
                        Box::new(ast::Expr::Value(Box::new(ast::Value::Variable(
                            ast::Variable::Named("b".to_string())
                        )))),
                        ast::Opcode::Add,
                        Box::new(ast::Expr::Number(3))
                    )
                ))
        );
    }

    #[test]
    fn factor_not_1() {
        assert!(
            factor_not("!true").unwrap().1 == Box::new(ast::Bool::Not(Box::new(ast::Bool::True)))
        );
    }

    #[test]
    fn factor_paren_1() {
        assert!(
            factor_paren("(!true)").unwrap().1
                == Box::new(ast::Bool::Not(Box::new(ast::Bool::True)))
        );
        assert!(
            factor_paren("(!a)").unwrap().1
                == Box::new(ast::Bool::Not(Box::new(ast::Bool::Value(Box::new(
                    ast::Value::Variable(ast::Variable::Named("a".to_string()))
                )))))
        );
    }

    #[test]
    fn _true1() {
        assert!(_true("true").unwrap().0 == "");
        assert!(_true("false").is_err());
    }

    #[test]
    fn _false1() {
        assert!(_false("false").unwrap().0 == "");
        assert!(_false("true").is_err());
    }

    #[test]
    fn and1() {
        assert!(and("&&").unwrap().0 == "");
        assert!(and("&").is_err());
    }

    #[test]
    fn or1() {
        assert!(or("||").unwrap().0 == "");
        assert!(or("|").is_err());
    }

    #[test]
    fn not1() {
        assert!(not("!").unwrap().0 == "");
        assert!(not("?").is_err());
    }
}
