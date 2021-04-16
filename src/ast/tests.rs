use crate::ast::*;

macro_rules! set {
    ( $( $x:expr ),* ) => {  // Match zero or more comma delimited items
        {
            let mut temp_set = HashSet::new();  // Create a mutable HashSet
            $(
                temp_set.insert($x); // Insert each item matched into the HashSet
            )*
            temp_set // Return the populated HashSet
        }
    };
}

#[test]
fn get_variables_assignment1() {
    assert_eq!(
        Assignment::Single(Variable::Named(String::from("x")), Value::Unit).get_variables(),
        set![Variable::Named(String::from("x"))]
    );
}

#[test]
fn get_variables_assignment2() {
    assert_eq!(
        Assignment::Single(
            Variable::Named(String::from("x")),
            Value::Variable(Variable::Named(String::from("y")))
        )
        .get_variables(),
        set![
            Variable::Named(String::from("x")),
            Variable::Named(String::from("y"))
        ]
    );
}

#[test]
fn get_variables_assignment3() {
    assert_eq!(
        Assignment::Tuple(vec![
            Assignment::Single(
                Variable::Named(String::from("x")),
                Value::Variable(Variable::Named(String::from("y"))),
            ),
            Assignment::Single(Variable::Named(String::from("z")), Value::Unit),
        ])
        .get_variables(),
        set![
            Variable::Named(String::from("x")),
            Variable::Named(String::from("y")),
            Variable::Named(String::from("z"))
        ]
    );
}

#[test]
fn get_affected_variables_assignment1() {
    assert_eq!(
        Assignment::Single(
            Variable::Named(String::from("x")),
            Value::Variable(Variable::Named(String::from("y")))
        )
        .get_affected_variables(),
        set![Variable::Named(String::from("x"))]
    );
}

#[test]
fn get_variables_assignment4_dedup() {
    assert_eq!(
        Assignment::Tuple(vec![
            Assignment::Single(
                Variable::Named(String::from("x")),
                Value::Variable(Variable::Named(String::from("y")))
            ),
            Assignment::Single(Variable::Named(String::from("x")), Value::Unit),
        ])
        .get_variables(),
        set![
            Variable::Named(String::from("x")),
            Variable::Named(String::from("y"))
        ]
    );
}

#[test]
fn get_variables_binding1() {
    assert_eq!(
        Binding::Declaration(Variable::Named(String::from("x")), Type::I32, false).get_variables(),
        set![Variable::Named(String::from("x"))]
    );
}

#[test]
fn get_variables_binding2() {
    assert_eq!(
        Binding::Assignment(
            Variable::Named(String::from("x")),
            Type::I32,
            Value::Variable(Variable::Named(String::from("y"))),
            false
        )
        .get_variables(),
        set![
            Variable::Named(String::from("x")),
            Variable::Named(String::from("y"))
        ]
    );
}

#[test]
fn get_variables_binding3() {
    assert_eq!(
        Binding::Tuple(vec![
            Command::Binding(Binding::Assignment(
                Variable::Named(String::from("x")),
                Type::I32,
                Value::Variable(Variable::Named(String::from("y"))),
                false
            )),
            Command::Binding(Binding::Declaration(
                Variable::Named(String::from("x")),
                Type::Bool,
                false
            ))
        ])
        .get_variables(),
        set![
            Variable::Named(String::from("x")),
            Variable::Named(String::from("y"))
        ]
    );
}

#[test]
fn get_affected_variables_binding1() {
    assert_eq!(
        Binding::Assignment(
            Variable::Named(String::from("x")),
            Type::Bool,
            Value::Variable(Variable::Named(String::from("y"))),
            false
        )
        .get_affected_variables(),
        set![Variable::Named(String::from("x"))]
    );
}

#[test]
fn get_variables_block_if1() {
    assert_eq!(
        Block::If(
            vec![Bool::Value(Box::new(Value::Variable(Variable::ArrayElem(
                String::from("arr"),
                Box::new(Value::Variable(Variable::Named(String::from("y"))))
            ))))],
            vec![vec![Command::Binding(Binding::Assignment(
                Variable::Named(String::from("x")),
                Type::I32,
                Value::Unit,
                false
            )),]],
            Vec::new()
        )
        .get_variables(),
        set![
            Variable::ArrayElem(
                String::from("arr"),
                Box::new(Value::Variable(Variable::Named(String::from("y"))))
            ),
            Variable::Named(String::from("y")),
            Variable::Named(String::from("x"))
        ]
    );
}

#[test]
fn get_variables_block_for1() {
    assert_eq!(
        Block::ForRange(
            Variable::Named(String::from("i")),
            Value::Variable(Variable::Named(String::from("first")),),
            Value::Variable(Variable::Named(String::from("second")),),
            vec![Command::Binding(Binding::Assignment(
                Variable::Named(String::from("x")),
                Type::I32,
                Value::Unit,
                false
            )),],
            Bool::True
        )
        .get_variables(),
        set![
            Variable::Named(String::from("i")),
            Variable::Named(String::from("first")),
            Variable::Named(String::from("second")),
            Variable::Named(String::from("x"))
        ]
    );
}

#[test]
fn get_variables_block_while1() {
    assert_eq!(
        Block::While(
            Bool::Value(Box::new(Value::Variable(Variable::Named(String::from(
                "check"
            ))))),
            vec![Command::Binding(Binding::Assignment(
                Variable::Named(String::from("x")),
                Type::I32,
                Value::Unit,
                false
            )),],
            Bool::True
        )
        .get_variables(),
        set![
            Variable::Named(String::from("check")),
            Variable::Named(String::from("x"))
        ]
    );
}

#[test]
fn swap1() {
    assert_eq!(
        Bool::Value(Box::new(Value::Variable(Variable::Named(String::from(
            "x"
        )))))
        .swap(
            Variable::Named(String::from("x")),
            Value::Expr(Expr::Number(2))
        ),
        Bool::Value(Box::new(Value::Expr(Expr::Number(2))))
    );
}

#[test]
fn swap2() {
    assert_eq!(
        Value::Variable(Variable::Named(String::from("x"))).swap(
            Variable::Named(String::from("x")),
            Value::Expr(Expr::Number(2))
        ),
        Value::Expr(Expr::Number(2))
    );
}
