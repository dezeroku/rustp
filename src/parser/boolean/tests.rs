use crate::parser::boolean::*;

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
fn factor_compare_not_equal1() {
    assert!(
        factor_compare_not_equal("12 != 43").unwrap().1
            == Box::new(ast::Bool::Not(Box::new(ast::Bool::Equal(
                ast::Expr::Number(12),
                ast::Expr::Number(43)
            ))))
    );

    assert!(
        factor_compare_not_equal("a != (b + 3)").unwrap().1
            == Box::new(ast::Bool::Not(Box::new(ast::Bool::Equal(
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
            ))))
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
            == Box::new(ast::Bool::LowerEqual(
                ast::Expr::Number(12),
                ast::Expr::Number(43)
            ))
    );

    assert!(
        factor_compare_smaller_equal("a <= (b + 3)").unwrap().1
            == Box::new(ast::Bool::LowerEqual(
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
            == Box::new(ast::Bool::GreaterThan(
                ast::Expr::Number(12),
                ast::Expr::Number(43)
            ))
    );

    assert!(
        factor_compare_greater("a > (b + 3)").unwrap().1
            == Box::new(ast::Bool::GreaterThan(
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
            == Box::new(ast::Bool::LowerThan(
                ast::Expr::Number(12),
                ast::Expr::Number(43)
            ))
    );

    assert_eq!(
        factor_compare_smaller("i < n").unwrap().1,
        Box::new(ast::Bool::LowerThan(
            ast::Expr::Value(Box::new(ast::Value::Variable(ast::Variable::Named(
                String::from("i")
            )))),
            ast::Expr::Value(Box::new(ast::Value::Variable(ast::Variable::Named(
                String::from("n")
            ))))
        ))
    );

    assert!(
        factor_compare_smaller("a < (b + 3)").unwrap().1
            == Box::new(ast::Bool::LowerThan(
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
    assert!(factor_not("!true").unwrap().1 == Box::new(ast::Bool::Not(Box::new(ast::Bool::True))));
}

#[test]
fn factor_paren_1() {
    assert!(
        factor_paren("(!true)").unwrap().1 == Box::new(ast::Bool::Not(Box::new(ast::Bool::True)))
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

#[test]
fn equal1() {
    assert_eq!(
        factor_compare_equal("quo * y + rem == x").unwrap().1,
        Box::new(ast::Bool::Equal(
            ast::Expr::Op(
                Box::new(ast::Expr::Op(
                    Box::new(ast::Expr::Value(Box::new(ast::Value::Variable(
                        ast::Variable::Named(String::from("quo"))
                    )))),
                    ast::Opcode::Mul,
                    Box::new(ast::Expr::Value(Box::new(ast::Value::Variable(
                        ast::Variable::Named(String::from("y"))
                    ))))
                )),
                ast::Opcode::Add,
                Box::new(ast::Expr::Value(Box::new(ast::Value::Variable(
                    ast::Variable::Named(String::from("rem"))
                ))))
            ),
            ast::Expr::Value(Box::new(ast::Value::Variable(ast::Variable::Named(
                String::from("x")
            ))))
        ))
    );
}

#[test]
fn forall1() {
    assert!(forall("forall i 143 == 12").is_ok());
    assert!(forall("forall j 3 - j < 2").unwrap().0 == "");
    assert_eq!(
        forall("forall x x == 1").unwrap().1,
        Box::new(ast::Bool::ForAll(
            ast::Variable::Named(String::from("x")),
            Box::new(ast::Bool::Equal(
                ast::Expr::Value(Box::new(ast::Value::Variable(ast::Variable::Named(
                    String::from("x")
                )))),
                ast::Expr::Number(1)
            ))
        ))
    );
}

#[test]
fn exists1() {
    assert!(exists("exists i 143 == 12").is_ok());
    assert!(exists("exists j 3 - j < 2").unwrap().0 == "");
    assert_eq!(
        exists("exists x x == 1").unwrap().1,
        Box::new(ast::Bool::Exists(
            ast::Variable::Named(String::from("x")),
            Box::new(ast::Bool::Equal(
                ast::Expr::Value(Box::new(ast::Value::Variable(ast::Variable::Named(
                    String::from("x")
                )))),
                ast::Expr::Number(1)
            ))
        ))
    );
}
