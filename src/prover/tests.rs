use crate::prover::*;

#[test]
fn prove_empty1() {
    assert!(prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::True,
                postcondition: Bool::True,
                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_1() {
    assert!(prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![
                    Command::Binding(Binding::Assignment(
                        Variable::Named(String::from("x")),
                        Type::I32,
                        Value::Expr(Expr::Number(1)),
                        false
                    )),
                    Command::ProveControl(ProveControl::Assert(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Number(1)
                    )))
                ],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::True,
                postcondition: Bool::True,
                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_2() {
    assert!(prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![
                    Command::Binding(Binding::Assignment(
                        Variable::Named(String::from("x")),
                        Type::I32,
                        Value::Expr(Expr::Number(1)),
                        false
                    )),
                    Command::ProveControl(ProveControl::Assert(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Number(1)
                    ))),
                    Command::Binding(Binding::Assignment(
                        Variable::Named(String::from("x")),
                        Type::I32,
                        Value::Expr(Expr::Number(2)),
                        false
                    )),
                    Command::ProveControl(ProveControl::Assert(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Number(2)
                    )))
                ],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::True,
                postcondition: Bool::True,
                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_3() {
    assert!(prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![
                    Command::Binding(Binding::Assignment(
                        Variable::Named(String::from("x")),
                        Type::I32,
                        Value::Expr(Expr::Number(1)),
                        false
                    )),
                    Command::ProveControl(ProveControl::Assert(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Number(1)
                    ))),
                    Command::Binding(Binding::Assignment(
                        Variable::Named(String::from("x")),
                        Type::I32,
                        Value::Expr(Expr::Number(2)),
                        false
                    )),
                    Command::ProveControl(ProveControl::Assert(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Number(2)
                    ))),
                    Command::Binding(Binding::Assignment(
                        Variable::Named(String::from("x")),
                        Type::I32,
                        Value::Expr(Expr::Op(
                            Box::new(Expr::Value(Box::new(Value::Variable(Variable::Named(
                                String::from("x")
                            ))))),
                            Opcode::Add,
                            Box::new(Expr::Number(3))
                        )),
                        false
                    )),
                    Command::ProveControl(ProveControl::Assert(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Number(5)
                    ))),
                    Command::Binding(Binding::Assignment(
                        Variable::Named(String::from("x")),
                        Type::I32,
                        Value::Expr(Expr::Number(7)),
                        false
                    )),
                    Command::ProveControl(ProveControl::Assert(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Number(7)
                    ))),
                ],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::True,
                postcondition: Bool::True,
                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_fail_1() {
    assert!(!prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![
                    Command::Binding(Binding::Assignment(
                        Variable::Named(String::from("x")),
                        Type::I32,
                        Value::Expr(Expr::Number(1)),
                        false
                    )),
                    Command::ProveControl(ProveControl::Assert(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Number(2)
                    )))
                ],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::True,
                postcondition: Bool::True,
                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_return_value1() {
    assert!(prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![Command::Binding(Binding::Assignment(
                    Variable::Named(String::from("x")),
                    Type::I32,
                    Value::Expr(Expr::Number(1)),
                    false
                ))],
                input: vec![],
                output: Type::I32,
                precondition: Bool::True,
                postcondition: Bool::Equal(
                    Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                        "return_value"
                    ))))),
                    Expr::Number(1)
                ),
                return_value: Value::Variable(Variable::Named(String::from("x")))
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_return_value_fail1() {
    assert!(!prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![Command::Binding(Binding::Assignment(
                    Variable::Named(String::from("x")),
                    Type::I32,
                    Value::Expr(Expr::Number(1)),
                    false
                ))],
                input: vec![],
                output: Type::I32,
                precondition: Bool::True,
                postcondition: Bool::Equal(
                    Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                        "return_value"
                    ))))),
                    Expr::Number(2)
                ),
                return_value: Value::Variable(Variable::Named(String::from("x")))
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_postcondition1() {
    assert!(prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![Command::Binding(Binding::Assignment(
                    Variable::Named(String::from("x")),
                    Type::I32,
                    Value::Expr(Expr::Number(1)),
                    false
                ))],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::True,
                postcondition: Bool::Equal(
                    Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                        "x"
                    ))))),
                    Expr::Number(1)
                ),
                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_postcondition2() {
    assert!(prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![Command::Binding(Binding::Assignment(
                    Variable::Named(String::from("x")),
                    Type::I32,
                    Value::Expr(Expr::Number(3)),
                    false
                ))],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::Equal(
                    Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                        "x"
                    ))))),
                    Expr::Number(2)
                ),
                postcondition: Bool::Equal(
                    Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                        "x"
                    ))))),
                    Expr::Number(3)
                ),
                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_postcondition_fail1() {
    assert!(!prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![Command::Binding(Binding::Assignment(
                    Variable::Named(String::from("x")),
                    Type::I32,
                    Value::Expr(Expr::Number(1)),
                    false
                ))],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::True,
                postcondition: Bool::Equal(
                    Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                        "x"
                    ))))),
                    Expr::Number(2)
                ),
                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_postcondition_fail2() {
    assert!(!prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![Command::Binding(Binding::Assignment(
                    Variable::Named(String::from("x")),
                    Type::I32,
                    Value::Expr(Expr::Number(2)),
                    false
                ))],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::Equal(
                    Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                        "x"
                    ))))),
                    Expr::Number(2)
                ),
                postcondition: Bool::Equal(
                    Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                        "x"
                    ))))),
                    Expr::Number(3)
                ),
                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_postcondition_fail3() {
    assert!(!prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::Equal(
                    Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                        "x"
                    ))))),
                    Expr::Number(2)
                ),
                postcondition: Bool::Equal(
                    Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                        "x"
                    ))))),
                    Expr::Number(3)
                ),
                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_old1() {
    assert!(prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![Command::Binding(Binding::Assignment(
                    Variable::Named(String::from("x")),
                    Type::I32,
                    Value::Expr(Expr::Number(3)),
                    false
                ))],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::And(
                    Box::new(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Number(2)
                    )),
                    Box::new(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x'old"
                        ))))),
                    ))
                ),
                postcondition: Bool::Equal(
                    Expr::Op(
                        Box::new(Expr::Value(Box::new(Value::Variable(Variable::Named(
                            String::from("x'old")
                        ))))),
                        Opcode::Add,
                        Box::new(Expr::Number(1))
                    ),
                    Expr::Number(3)
                ),
                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_old2() {
    assert!(prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![Command::Binding(Binding::Assignment(
                    Variable::Named(String::from("x")),
                    Type::I32,
                    Value::Expr(Expr::Number(3)),
                    false
                ))],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::And(
                    Box::new(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Number(2)
                    )),
                    Box::new(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x'old"
                        ))))),
                    ))
                ),
                postcondition: Bool::Equal(
                    Expr::Op(
                        Box::new(Expr::Value(Box::new(Value::Variable(Variable::Named(
                            String::from("x'old")
                        ))))),
                        Opcode::Add,
                        Box::new(Expr::Number(1))
                    ),
                    Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                        "x"
                    )))))
                ),
                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_old_fail1() {
    assert!(!prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![Command::Binding(Binding::Assignment(
                    Variable::Named(String::from("x")),
                    Type::I32,
                    Value::Expr(Expr::Number(3)),
                    false
                ))],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::And(
                    Box::new(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Number(2)
                    )),
                    Box::new(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x'old"
                        ))))),
                    ))
                ),
                postcondition: Bool::Equal(
                    Expr::Op(
                        Box::new(Expr::Value(Box::new(Value::Variable(Variable::Named(
                            String::from("x'old")
                        ))))),
                        Opcode::Add,
                        Box::new(Expr::Number(1))
                    ),
                    Expr::Number(4)
                ),
                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_if1() {
    assert!(prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![Command::Block(Block::If(
                    vec![Bool::True],
                    vec![vec![Command::Binding(Binding::Assignment(
                        Variable::Named(String::from("x")),
                        Type::I32,
                        Value::Expr(Expr::Number(3)),
                        false
                    ))]],
                    vec![]
                ))],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::And(
                    Box::new(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Number(1)
                    )),
                    Box::new(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x'old"
                        ))))),
                    ))
                ),
                postcondition: Bool::Equal(
                    Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                        "x"
                    ))))),
                    Expr::Number(3)
                ),

                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_if2() {
    assert!(prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![Command::Block(Block::If(
                    vec![Bool::True],
                    vec![vec![Command::Binding(Binding::Assignment(
                        Variable::Named(String::from("x")),
                        Type::I32,
                        Value::Expr(Expr::Number(3)),
                        false
                    ))]],
                    vec![Command::Binding(Binding::Assignment(
                        Variable::Named(String::from("x")),
                        Type::I32,
                        Value::Expr(Expr::Number(1)),
                        false
                    ))]
                ))],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::And(
                    Box::new(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Number(1)
                    )),
                    Box::new(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x'old"
                        ))))),
                    ))
                ),
                postcondition: Bool::Equal(
                    Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                        "x"
                    ))))),
                    Expr::Number(3)
                ),

                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_if_fail1() {
    assert!(!prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![Command::Block(Block::If(
                    vec![Bool::False],
                    vec![vec![Command::Binding(Binding::Assignment(
                        Variable::Named(String::from("x")),
                        Type::I32,
                        Value::Expr(Expr::Number(3)),
                        false
                    ))]],
                    vec![Command::Binding(Binding::Assignment(
                        Variable::Named(String::from("x")),
                        Type::I32,
                        Value::Expr(Expr::Number(1)),
                        false
                    ))]
                ))],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::And(
                    Box::new(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Number(1)
                    )),
                    Box::new(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x'old"
                        ))))),
                    ))
                ),
                postcondition: Bool::Equal(
                    Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                        "x"
                    ))))),
                    Expr::Number(3)
                ),

                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_if_nested1() {
    assert!(prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![Command::Block(Block::If(
                    vec![Bool::True],
                    vec![vec![Command::Block(Block::If(
                        vec![Bool::True],
                        vec![vec![Command::Binding(Binding::Assignment(
                            Variable::Named(String::from("x")),
                            Type::I32,
                            Value::Expr(Expr::Number(3)),
                            false
                        ))]],
                        vec![Command::Binding(Binding::Assignment(
                            Variable::Named(String::from("x")),
                            Type::I32,
                            Value::Expr(Expr::Number(1)),
                            false
                        ))]
                    ))]],
                    vec![Command::Binding(Binding::Assignment(
                        Variable::Named(String::from("x")),
                        Type::I32,
                        Value::Expr(Expr::Number(1)),
                        false
                    ))]
                ))],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::And(
                    Box::new(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Number(1)
                    )),
                    Box::new(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x'old"
                        ))))),
                    ))
                ),
                postcondition: Bool::Equal(
                    Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                        "x"
                    ))))),
                    Expr::Number(3)
                ),

                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_if_nested_fail1() {
    assert!(!prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![Command::Block(Block::If(
                    vec![Bool::True],
                    vec![vec![Command::Block(Block::If(
                        vec![Bool::False],
                        vec![vec![Command::Binding(Binding::Assignment(
                            Variable::Named(String::from("x")),
                            Type::I32,
                            Value::Expr(Expr::Number(3)),
                            false
                        ))]],
                        vec![Command::Binding(Binding::Assignment(
                            Variable::Named(String::from("x")),
                            Type::I32,
                            Value::Expr(Expr::Number(1)),
                            false
                        ))]
                    ))]],
                    vec![Command::Binding(Binding::Assignment(
                        Variable::Named(String::from("x")),
                        Type::I32,
                        Value::Expr(Expr::Number(1)),
                        false
                    ))]
                ))],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::And(
                    Box::new(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Number(1)
                    )),
                    Box::new(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x'old"
                        ))))),
                    ))
                ),
                postcondition: Bool::Equal(
                    Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                        "x"
                    ))))),
                    Expr::Number(3)
                ),

                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_if_multiple1() {
    assert!(prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![Command::Block(Block::If(
                    vec![Bool::False, Bool::False, Bool::True],
                    vec![
                        vec![Command::Binding(Binding::Assignment(
                            Variable::Named(String::from("x")),
                            Type::I32,
                            Value::Expr(Expr::Number(5)),
                            false
                        ))],
                        vec![Command::Binding(Binding::Assignment(
                            Variable::Named(String::from("x")),
                            Type::I32,
                            Value::Expr(Expr::Number(6)),
                            false
                        ))],
                        vec![Command::Binding(Binding::Assignment(
                            Variable::Named(String::from("x")),
                            Type::I32,
                            Value::Expr(Expr::Number(3)),
                            false
                        ))]
                    ],
                    vec![Command::Binding(Binding::Assignment(
                        Variable::Named(String::from("x")),
                        Type::I32,
                        Value::Expr(Expr::Number(9)),
                        false
                    ))]
                ))],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::And(
                    Box::new(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Number(1)
                    )),
                    Box::new(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x'old"
                        ))))),
                    ))
                ),
                postcondition: Bool::Equal(
                    Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                        "x"
                    ))))),
                    Expr::Number(3)
                ),

                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_if_multiple_fail1() {
    assert!(!prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![Command::Block(Block::If(
                    vec![Bool::False, Bool::True, Bool::False],
                    vec![
                        vec![Command::Binding(Binding::Assignment(
                            Variable::Named(String::from("x")),
                            Type::I32,
                            Value::Expr(Expr::Number(3)),
                            false
                        ))],
                        vec![Command::Binding(Binding::Assignment(
                            Variable::Named(String::from("x")),
                            Type::I32,
                            Value::Expr(Expr::Number(5)),
                            false
                        ))],
                        vec![Command::Binding(Binding::Assignment(
                            Variable::Named(String::from("x")),
                            Type::I32,
                            Value::Expr(Expr::Number(3)),
                            false
                        ))]
                    ],
                    vec![Command::Binding(Binding::Assignment(
                        Variable::Named(String::from("x")),
                        Type::I32,
                        Value::Expr(Expr::Number(3)),
                        false
                    ))]
                ))],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::And(
                    Box::new(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Number(1)
                    )),
                    Box::new(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x'old"
                        ))))),
                    ))
                ),
                postcondition: Bool::Equal(
                    Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                        "x"
                    ))))),
                    Expr::Number(3)
                ),

                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_while_invariant1() {
    /*
    let q: i32 = 0;
    let r: i32 = 2;
    let z: i32 = 1;
    //%invariant q + r == 2 * z
    while z >= q {
        z = z + 1;
        q = q + 2;
    }
    */
    assert!(prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![Command::Block(Block::While(
                    Bool::GreaterEqual(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "z"
                        ))))),
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "q"
                        )))))
                    ),
                    vec![
                        Command::Assignment(Assignment::Single(
                            Variable::Named(String::from("z")),
                            Value::Expr(Expr::Op(
                                Box::new(Expr::Value(Box::new(Value::Variable(Variable::Named(
                                    String::from("z")
                                ))))),
                                Opcode::Add,
                                Box::new(Expr::Number(1))
                            ))
                        )),
                        Command::Assignment(Assignment::Single(
                            Variable::Named(String::from("q")),
                            Value::Expr(Expr::Op(
                                Box::new(Expr::Value(Box::new(Value::Variable(Variable::Named(
                                    String::from("q")
                                ))))),
                                Opcode::Add,
                                Box::new(Expr::Number(2))
                            ))
                        ))
                    ],
                    Bool::Equal(
                        Expr::Op(
                            Box::new(Expr::Value(Box::new(Value::Variable(Variable::Named(
                                String::from("q")
                            ))))),
                            Opcode::Add,
                            Box::new(Expr::Value(Box::new(Value::Variable(Variable::Named(
                                String::from("r")
                            )))))
                        ),
                        Expr::Op(
                            Box::new(Expr::Value(Box::new(Value::Variable(Variable::Named(
                                String::from("z")
                            ))))),
                            Opcode::Mul,
                            Box::new(Expr::Number(2))
                        )
                    ),
                    Expr::Number(0)
                ))],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::And(
                    Box::new(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "q"
                        ))))),
                        Expr::Number(0)
                    )),
                    Box::new(Bool::And(
                        Box::new(Bool::Equal(
                            Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                                "r"
                            ))))),
                            Expr::Number(2)
                        )),
                        Box::new(Bool::Equal(
                            Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                                "z"
                            ))))),
                            Expr::Number(1)
                        ))
                    ))
                ),
                postcondition: Bool::True,
                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_while_invariant_fail_1() {
    /*
    let q: i32 = 0;
    let r: i32 = 2;
    let z: i32 = 1;
    //%invariant q + r == 2 * z
    while z >= q {
        z = z + 1;
        q = q + 3;
    }
    */
    assert!(!prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![Command::Block(Block::While(
                    Bool::GreaterEqual(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "z"
                        ))))),
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "q"
                        )))))
                    ),
                    vec![
                        Command::Assignment(Assignment::Single(
                            Variable::Named(String::from("z")),
                            Value::Expr(Expr::Op(
                                Box::new(Expr::Value(Box::new(Value::Variable(Variable::Named(
                                    String::from("z")
                                ))))),
                                Opcode::Add,
                                Box::new(Expr::Number(1))
                            ))
                        )),
                        Command::Assignment(Assignment::Single(
                            Variable::Named(String::from("q")),
                            Value::Expr(Expr::Op(
                                Box::new(Expr::Value(Box::new(Value::Variable(Variable::Named(
                                    String::from("q")
                                ))))),
                                Opcode::Add,
                                Box::new(Expr::Number(3))
                            ))
                        ))
                    ],
                    Bool::Equal(
                        Expr::Op(
                            Box::new(Expr::Value(Box::new(Value::Variable(Variable::Named(
                                String::from("q")
                            ))))),
                            Opcode::Add,
                            Box::new(Expr::Value(Box::new(Value::Variable(Variable::Named(
                                String::from("r")
                            )))))
                        ),
                        Expr::Op(
                            Box::new(Expr::Value(Box::new(Value::Variable(Variable::Named(
                                String::from("z")
                            ))))),
                            Opcode::Mul,
                            Box::new(Expr::Number(2))
                        )
                    ),
                    Expr::Number(0)
                ))],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::And(
                    Box::new(Bool::Equal(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "q"
                        ))))),
                        Expr::Number(0)
                    )),
                    Box::new(Bool::And(
                        Box::new(Bool::Equal(
                            Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                                "r"
                            ))))),
                            Expr::Number(2)
                        )),
                        Box::new(Bool::Equal(
                            Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                                "z"
                            ))))),
                            Expr::Number(1)
                        ))
                    ))
                ),
                postcondition: Bool::True,
                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_while_variant1() {
    /*
    let mut i: i32 = 10;
    //%invariant i >= 0
    //%variant i
    while i > 0 {
        i = i - 1;
    }
    */
    assert!(prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![
                    Command::Binding(Binding::Assignment(
                        Variable::Named(String::from("i")),
                        Type::I32,
                        Value::Expr(Expr::Number(10)),
                        true,
                    )),
                    Command::Block(Block::While(
                        Bool::GreaterThan(
                            Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                                "i"
                            ))))),
                            Expr::Number(0)
                        ),
                        vec![Command::Assignment(Assignment::Single(
                            Variable::Named(String::from("i")),
                            Value::Expr(Expr::Op(
                                Box::new(Expr::Value(Box::new(Value::Variable(Variable::Named(
                                    String::from("i")
                                ))))),
                                Opcode::Sub,
                                Box::new(Expr::Number(1))
                            ))
                        )),],
                        Bool::GreaterEqual(
                            Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                                "i"
                            ))))),
                            Expr::Number(0)
                        ),
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "i"
                        )))))
                    ))
                ],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::True,
                postcondition: Bool::True,
                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_while_variant_fail_1() {
    /*
    let mut i: i32 = 10;
    //%invariant true
    //%variant a
    while i > 0 {
        i = i - 1;
    }
    */
    assert!(!prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![
                    Command::Binding(Binding::Assignment(
                        Variable::Named(String::from("i")),
                        Type::I32,
                        Value::Expr(Expr::Number(10)),
                        true,
                    )),
                    Command::Block(Block::While(
                        Bool::GreaterThan(
                            Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                                "i"
                            ))))),
                            Expr::Number(0)
                        ),
                        vec![Command::Assignment(Assignment::Single(
                            Variable::Named(String::from("i")),
                            Value::Expr(Expr::Op(
                                Box::new(Expr::Value(Box::new(Value::Variable(Variable::Named(
                                    String::from("i")
                                ))))),
                                Opcode::Sub,
                                Box::new(Expr::Number(1))
                            ))
                        )),],
                        Bool::True,
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "a"
                        )))))
                    ))
                ],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::True,
                postcondition: Bool::True,
                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_while_postcondition1() {
    /*
    //%precondition x>=0 && y>=0
    //%postcondition q * y + r == x
    fn remainder_simple(x: i32, y: i32) {
        let mut q: i32 = 0;
        let mut r: i32 = x;

        //%invariant q * y + r == x
        while r >= y {
            r = r - y;
            q = q + 1;
        }
    }
    */
    assert!(prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![
                    Command::Assignment(Assignment::Single(
                        Variable::Named(String::from("q")),
                        Value::Expr(Expr::Number(0))
                    )),
                    Command::Assignment(Assignment::Single(
                        Variable::Named(String::from("r")),
                        Value::Variable(Variable::Named(String::from("x")))
                    )),
                    Command::Block(Block::While(
                        Bool::GreaterEqual(
                            Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                                "r"
                            ))))),
                            Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                                "y"
                            )))))
                        ),
                        vec![
                            Command::Assignment(Assignment::Single(
                                Variable::Named(String::from("r")),
                                Value::Expr(Expr::Op(
                                    Box::new(Expr::Value(Box::new(Value::Variable(
                                        Variable::Named(String::from("r"))
                                    )))),
                                    Opcode::Sub,
                                    Box::new(Expr::Value(Box::new(Value::Variable(
                                        Variable::Named(String::from("y"))
                                    )))),
                                ))
                            )),
                            Command::Assignment(Assignment::Single(
                                Variable::Named(String::from("q")),
                                Value::Expr(Expr::Op(
                                    Box::new(Expr::Value(Box::new(Value::Variable(
                                        Variable::Named(String::from("q"))
                                    )))),
                                    Opcode::Add,
                                    Box::new(Expr::Number(1))
                                ))
                            )),
                        ],
                        Bool::Equal(
                            Expr::Op(
                                Box::new(Expr::Op(
                                    Box::new(Expr::Value(Box::new(Value::Variable(
                                        Variable::Named(String::from("q"))
                                    )))),
                                    Opcode::Mul,
                                    Box::new(Expr::Value(Box::new(Value::Variable(
                                        Variable::Named(String::from("y"))
                                    ))))
                                )),
                                Opcode::Add,
                                Box::new(Expr::Value(Box::new(Value::Variable(Variable::Named(
                                    String::from("r")
                                )))))
                            ),
                            Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                                "x"
                            )))))
                        ),
                        Expr::Number(0)
                    ))
                ],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::And(
                    Box::new(Bool::GreaterEqual(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Number(0)
                    )),
                    Box::new(Bool::GreaterEqual(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "y"
                        ))))),
                        Expr::Number(0)
                    ))
                ),
                postcondition: Bool::Equal(
                    Expr::Op(
                        Box::new(Expr::Op(
                            Box::new(Expr::Value(Box::new(Value::Variable(Variable::Named(
                                String::from("q")
                            ))))),
                            Opcode::Mul,
                            Box::new(Expr::Value(Box::new(Value::Variable(Variable::Named(
                                String::from("y")
                            )))))
                        )),
                        Opcode::Add,
                        Box::new(Expr::Value(Box::new(Value::Variable(Variable::Named(
                            String::from("r")
                        )))))
                    ),
                    Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                        "x"
                    )))))
                ),
                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_while_postcondition_fail1() {
    /*
    //%precondition x>=0 && y>=0
    //%postcondition q * y + r == x + 1
    fn remainder_simple(x: i32, y: i32) {
        let mut q: i32 = 0;
        let mut r: i32 = x;

        //%invariant q * y + r == x
        while r >= y {
            r = r - y;
            q = q + 1;
        }
    }
    */
    assert!(!prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![
                    Command::Assignment(Assignment::Single(
                        Variable::Named(String::from("q")),
                        Value::Expr(Expr::Number(0))
                    )),
                    Command::Assignment(Assignment::Single(
                        Variable::Named(String::from("r")),
                        Value::Variable(Variable::Named(String::from("x")))
                    )),
                    Command::Block(Block::While(
                        Bool::GreaterEqual(
                            Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                                "r"
                            ))))),
                            Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                                "y"
                            )))))
                        ),
                        vec![
                            Command::Assignment(Assignment::Single(
                                Variable::Named(String::from("r")),
                                Value::Expr(Expr::Op(
                                    Box::new(Expr::Value(Box::new(Value::Variable(
                                        Variable::Named(String::from("r"))
                                    )))),
                                    Opcode::Sub,
                                    Box::new(Expr::Value(Box::new(Value::Variable(
                                        Variable::Named(String::from("y"))
                                    )))),
                                ))
                            )),
                            Command::Assignment(Assignment::Single(
                                Variable::Named(String::from("q")),
                                Value::Expr(Expr::Op(
                                    Box::new(Expr::Value(Box::new(Value::Variable(
                                        Variable::Named(String::from("q"))
                                    )))),
                                    Opcode::Add,
                                    Box::new(Expr::Number(1))
                                ))
                            )),
                        ],
                        Bool::Equal(
                            Expr::Op(
                                Box::new(Expr::Op(
                                    Box::new(Expr::Value(Box::new(Value::Variable(
                                        Variable::Named(String::from("q"))
                                    )))),
                                    Opcode::Mul,
                                    Box::new(Expr::Value(Box::new(Value::Variable(
                                        Variable::Named(String::from("y"))
                                    ))))
                                )),
                                Opcode::Add,
                                Box::new(Expr::Value(Box::new(Value::Variable(Variable::Named(
                                    String::from("r")
                                )))))
                            ),
                            Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                                "x"
                            )))))
                        ),
                        Expr::Number(0)
                    ))
                ],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::And(
                    Box::new(Bool::GreaterEqual(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "x"
                        ))))),
                        Expr::Number(0)
                    )),
                    Box::new(Bool::GreaterEqual(
                        Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                            "y"
                        ))))),
                        Expr::Number(0)
                    ))
                ),
                postcondition: Bool::Equal(
                    Expr::Op(
                        Box::new(Expr::Op(
                            Box::new(Expr::Value(Box::new(Value::Variable(Variable::Named(
                                String::from("q")
                            ))))),
                            Opcode::Mul,
                            Box::new(Expr::Value(Box::new(Value::Variable(Variable::Named(
                                String::from("y")
                            )))))
                        )),
                        Opcode::Add,
                        Box::new(Expr::Value(Box::new(Value::Variable(Variable::Named(
                            String::from("r")
                        )))))
                    ),
                    Expr::Op(
                        Box::new(Expr::Value(Box::new(Value::Variable(Variable::Named(
                            String::from("x")
                        ))))),
                        Opcode::Add,
                        Box::new(Expr::Number(1))
                    )
                ),
                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_array_elem_dummy1() {
    assert!(prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![Command::Binding(Binding::Assignment(
                    Variable::ArrayElem(String::from("x"), Box::new(Value::Expr(Expr::Number(0)))),
                    Type::I32,
                    Value::Expr(Expr::Number(1)),
                    false
                ))],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::True,
                postcondition: Bool::Equal(
                    Expr::Value(Box::new(Value::Variable(Variable::ArrayElem(
                        String::from("x"),
                        Box::new(Value::Expr(Expr::Number(0)))
                    )))),
                    Expr::Number(1)
                ),
                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_array_elem_dummy_fail1() {
    assert!(!prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![Command::Binding(Binding::Assignment(
                    Variable::ArrayElem(String::from("x"), Box::new(Value::Expr(Expr::Number(0)))),
                    Type::I32,
                    Value::Expr(Expr::Number(1)),
                    false
                ))],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::True,
                postcondition: Bool::Equal(
                    Expr::Value(Box::new(Value::Variable(Variable::ArrayElem(
                        String::from("x"),
                        Box::new(Value::Expr(Expr::Number(0)))
                    )))),
                    Expr::Number(2)
                ),
                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_array_elem_var1() {
    /*
    y = 0;
    x[0] = 1;
    //%assert x[y] == 1
    */

    assert!(prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![Command::Binding(Binding::Assignment(
                    Variable::ArrayElem(String::from("x"), Box::new(Value::Expr(Expr::Number(0)))),
                    Type::I32,
                    Value::Expr(Expr::Number(1)),
                    false
                ))],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::Equal(
                    Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                        "y"
                    ))))),
                    Expr::Number(0)
                ),

                postcondition: Bool::Equal(
                    Expr::Value(Box::new(Value::Variable(Variable::ArrayElem(
                        String::from("x"),
                        Box::new(Value::Variable(Variable::Named(String::from("y"))))
                    )))),
                    Expr::Number(1)
                ),
                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_array_elem_var_fail1() {
    /*
    y = 0;
    x[0] = 2;
    //%assert x[y] == 1
    */
    assert!(!prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![Command::Binding(Binding::Assignment(
                    Variable::ArrayElem(String::from("x"), Box::new(Value::Expr(Expr::Number(0)))),
                    Type::I32,
                    Value::Expr(Expr::Number(2)),
                    false
                ))],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::Equal(
                    Expr::Value(Box::new(Value::Variable(Variable::Named(String::from(
                        "y"
                    ))))),
                    Expr::Number(0)
                ),

                postcondition: Bool::Equal(
                    Expr::Value(Box::new(Value::Variable(Variable::ArrayElem(
                        String::from("x"),
                        Box::new(Value::Variable(Variable::Named(String::from("y"))))
                    )))),
                    Expr::Number(1)
                ),
                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_array_dummy1() {
    assert!(prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![Command::Binding(Binding::Assignment(
                    Variable::Named(String::from("x")),
                    Type::Array(Box::new(Type::I32), 1),
                    Value::Array(vec![Value::Expr(Expr::Number(1))]),
                    false
                ))],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::True,
                postcondition: Bool::Equal(
                    Expr::Value(Box::new(Value::Variable(Variable::ArrayElem(
                        String::from("x"),
                        Box::new(Value::Expr(Expr::Number(0)))
                    )))),
                    Expr::Number(1)
                ),
                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_array_dummy_fail1() {
    assert!(!prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![Command::Binding(Binding::Assignment(
                    Variable::Named(String::from("x")),
                    Type::Array(Box::new(Type::I32), 1),
                    Value::Array(vec![Value::Expr(Expr::Number(2))]),
                    false
                ))],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::True,
                postcondition: Bool::Equal(
                    Expr::Value(Box::new(Value::Variable(Variable::ArrayElem(
                        String::from("x"),
                        Box::new(Value::Expr(Expr::Number(0)))
                    )))),
                    Expr::Number(1)
                ),
                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_array_dummy_variable_index1() {
    assert!(prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![
                    Command::Binding(Binding::Assignment(
                        Variable::Named(String::from("y")),
                        Type::I32,
                        Value::Expr(Expr::Number(0)),
                        false
                    )),
                    Command::Binding(Binding::Assignment(
                        Variable::Named(String::from("x")),
                        Type::Array(Box::new(Type::I32), 1),
                        Value::Array(vec![Value::Expr(Expr::Number(1))]),
                        false
                    ))
                ],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::True,
                postcondition: Bool::Equal(
                    Expr::Value(Box::new(Value::Variable(Variable::ArrayElem(
                        String::from("x"),
                        Box::new(Value::Variable(Variable::Named(String::from("y"))))
                    )))),
                    Expr::Number(1)
                ),
                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}

#[test]
fn prove_array_dummy_variable_index_fail1() {
    assert!(!prove(
        Program {
            content: vec![Function {
                name: String::from("test"),
                content: vec![
                    Command::Binding(Binding::Assignment(
                        Variable::Named(String::from("y")),
                        Type::I32,
                        Value::Expr(Expr::Number(1)),
                        false
                    )),
                    Command::Binding(Binding::Assignment(
                        Variable::Named(String::from("x")),
                        Type::Array(Box::new(Type::I32), 1),
                        Value::Array(vec![Value::Expr(Expr::Number(1))]),
                        false
                    ))
                ],
                input: vec![],
                output: Type::Unit,
                precondition: Bool::True,
                postcondition: Bool::Equal(
                    Expr::Value(Box::new(Value::Variable(Variable::ArrayElem(
                        String::from("x"),
                        Box::new(Value::Variable(Variable::Named(String::from("y"))))
                    )))),
                    Expr::Number(1)
                ),
                return_value: Value::Unit
            }]
        },
        vec![]
    ));
}
