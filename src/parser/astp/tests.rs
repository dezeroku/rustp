use crate::parser::astp::*;

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

    assert_eq!(a, b);
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
        precondition: a.precondition.clone(),
        postcondition: ast::Bool::True,
        return_value: ast::Value::Unit,
    };

    assert_eq!(a, b);
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
        precondition: a.precondition.clone(),
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

    assert_eq!(a, b);
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
        precondition: a.precondition.clone(),
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

    assert_eq!(a, b);
}

#[test]
fn type_def1() {
    assert!(type_def("i32").unwrap().1 == ast::Type::I32);
    assert!(type_def("bool").unwrap().1 == ast::Type::Bool);
    assert!(type_def("&i32").unwrap().1 == ast::Type::Reference(Box::new(ast::Type::I32)));
    assert!(
        type_def("&mut i32").unwrap().1 == ast::Type::ReferenceMutable(Box::new(ast::Type::I32))
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
    assert!(array_type("[bool;4]").unwrap().1 == ast::Type::Array(Box::new(ast::Type::Bool), 4));
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
        if_else(
            "if a == 12 {let b: i32 = 3;} else if a == 13 {let b: i32 = 4;} else {let b: i32 = 1;}"
        )
        .unwrap()
        .0 == ""
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
        if_else("if a == 14 {\nlet c: i32 = 3;\n} else if a == 13 {\n   let c: i32 = a + 43;\n}")
            .unwrap()
            .0
            == ""
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
fn loop_invariant1() {
    assert!(loop_invariant("//%invariant 143 == 12\n").is_ok());
    assert!(loop_invariant("//%invariant 143 - 4 < 2\n").unwrap().0 == "");
    assert!(loop_invariant("//%invariant true\n").unwrap().0 == "");
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
        binding_assignment_tuple_multiple("let (x,y, _): (i32, bool, bool) = (12, true, false);")
            .unwrap()
            .0
            == ""
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
        while_parse("//%invariant true\nwhile i {}").unwrap().1
            == ast::Command::Block(ast::Block::While(
                ast::Bool::Value(Box::new(ast::Value::Variable(ast::Variable::Named(
                    String::from("i")
                )))),
                Vec::new(),
                ast::Bool::True
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
        while_parse("//%invariant true\n  while true {let x: i32 = 1;}")
            .unwrap()
            .1,
        ast::Command::Block(ast::Block::While(ast::Bool::True, temp, ast::Bool::True))
    );
}

#[test]
fn for_parse1() {
    assert!(
        for_parse("//%invariant true\nfor i in 0..2 {}").unwrap().1
            == ast::Command::Block(ast::Block::ForRange(
                ast::Variable::Named("i".to_string()),
                ast::Value::Expr(ast::Expr::Number(0)),
                ast::Value::Expr(ast::Expr::Number(2)),
                Vec::new(),
                ast::Bool::True
            ))
    );

    assert!(
        for_parse("//%invariant true\n for i in 0..b {}").unwrap().1
            == ast::Command::Block(ast::Block::ForRange(
                ast::Variable::Named("i".to_string()),
                ast::Value::Expr(ast::Expr::Number(0)),
                ast::Value::Variable(ast::Variable::Named("b".to_string())),
                Vec::new(),
                ast::Bool::True
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
        for_parse("//%invariant true\nfor i in 0..b {let x: i32 = 1;}")
            .unwrap()
            .1,
        ast::Command::Block(ast::Block::ForRange(
            ast::Variable::Named("i".to_string()),
            ast::Value::Expr(ast::Expr::Number(0)),
            ast::Value::Variable(ast::Variable::Named("b".to_string())),
            temp,
            ast::Bool::True
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
        for_parse("//%invariant true\n  for i in 0..b {let x: i32 = 1; let y: bool = true;}")
            .unwrap()
            .1
            == ast::Command::Block(ast::Block::ForRange(
                ast::Variable::Named("i".to_string()),
                ast::Value::Expr(ast::Expr::Number(0)),
                ast::Value::Variable(ast::Variable::Named("b".to_string())),
                temp,
                ast::Bool::True
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
