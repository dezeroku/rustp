use crate::parser::math::*;

#[test]
fn mult_expr_right1() {
    assert!(mult_expr_right("13").is_err());
    assert!(mult_expr_right("*13").is_ok());
    assert!(
        mult_expr_right("/13") == Ok(("", (ast::Opcode::Div, Box::new(ast::Expr::Number(13)))))
    );
}

#[test]
fn add_expr_right1() {
    assert!(add_expr_right("13").is_err());
    assert!(add_expr_right("-13").is_ok());
    assert!(add_expr_right("+13") == Ok(("", (ast::Opcode::Add, Box::new(ast::Expr::Number(13))))));
}

#[test]
fn mult_expr1() {
    assert!(
        mult_expr("13 * 4")
            == Ok((
                "",
                Box::new(ast::Expr::Op(
                    Box::new(ast::Expr::Number(13)),
                    ast::Opcode::Mul,
                    Box::new(ast::Expr::Number(4))
                ))
            ))
    );
}

#[test]
fn mult_expr2() {
    assert!(
        mult_expr(" 13 * 4")
            == Ok((
                "",
                Box::new(ast::Expr::Op(
                    Box::new(ast::Expr::Number(13)),
                    ast::Opcode::Mul,
                    Box::new(ast::Expr::Number(4))
                ))
            ))
    );
}

#[test]
fn mult_expr3() {
    assert!(
        mult_expr("13 * 4 ")
            == Ok((
                "",
                Box::new(ast::Expr::Op(
                    Box::new(ast::Expr::Number(13)),
                    ast::Opcode::Mul,
                    Box::new(ast::Expr::Number(4))
                ))
            ))
    );
}

#[test]
fn primary_expr1() {
    assert!(primary_expr("1").is_ok());
    assert!(primary_expr("3").is_ok());
    assert!(primary_expr("13") == Ok(("", Box::new(ast::Expr::Number(13)))));
    assert!(primary_expr("(3)").unwrap().0 == "");
    assert!(primary_expr("").is_err());
}

#[test]
fn primary_expr2() {
    assert!(primary_expr("1 + 1").is_ok());
    assert!(primary_expr("3 - 1").is_ok());
    assert!(primary_expr("1 / 1").is_ok());
    assert!(primary_expr("1 /") == Ok((" /", Box::new(ast::Expr::Number(1)))));
    assert!(
        *expr("12 * 3").unwrap().1
            == *Box::new(ast::Expr::Op(
                Box::new(ast::Expr::Number(12)),
                ast::Opcode::Mul,
                Box::new(ast::Expr::Number(3))
            ))
    );
}

#[test]
fn primary_expr3() {
    assert!(primary_expr("1 - 1 + 1").is_ok());
    assert!(primary_expr("1 * 2 -3").is_ok());
    assert!(primary_expr("1 + 1 - 3").is_ok());
}

#[test]
fn expr1() {
    assert!(expr("1 + 1").is_ok());
    assert!(expr("13 - 4").is_ok());
    assert!(expr("13-4").is_ok());
    assert!(expr("13 / ").unwrap().0 != "");
}

#[test]
fn expr2() {
    assert!(
        *expr("12 * 3").unwrap().1
            == *Box::new(ast::Expr::Op(
                Box::new(ast::Expr::Number(12)),
                ast::Opcode::Mul,
                Box::new(ast::Expr::Number(3))
            ))
    );

    assert!(
        *expr("12 % 3").unwrap().1
            == *Box::new(ast::Expr::Op(
                Box::new(ast::Expr::Number(12)),
                ast::Opcode::Rem,
                Box::new(ast::Expr::Number(3))
            ))
    );
}

#[test]
fn expr3() {
    assert!(expr("13 * 4 + 2 / 4 - 8").unwrap().0 == "");
}

#[test]
fn expr4() {
    assert!(
        expr("1 + x").unwrap().1
            == Box::new(ast::Expr::Op(
                Box::new(ast::Expr::Number(1)),
                ast::Opcode::Add,
                Box::new(ast::Expr::Value(Box::new(ast::Value::Variable(
                    ast::Variable::Named("x".to_string())
                ))))
            ))
    );
}

#[test]
fn expr_number1() {
    assert!(expr_number("1").is_ok());
    assert!(*expr_number("13").unwrap().1 == *Box::new(ast::Expr::Number(13)));
}

#[test]
fn mult_or_divide_or_mod1() {
    assert!(mult_or_divide_or_mod("*").is_ok());
    assert!(mult_or_divide_or_mod("/").is_ok());
    assert!(mult_or_divide_or_mod("%").is_ok());
    assert!(mult_or_divide_or_mod("+").is_err());
}

#[test]
fn add_or_subtract1() {
    assert!(add_or_subtract("+").is_ok());
    assert!(add_or_subtract("-").is_ok());
    assert!(add_or_subtract("/").is_err());
}

#[test]
fn number1() {
    assert!(number("1").is_ok());
    assert!(number("12").is_ok());
    assert!(number("194").is_ok());
    assert!(number("").is_err());
}
