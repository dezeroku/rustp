#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub calculator); // synthesized by LALRPOP

#[test]
fn calculator1() {
    assert!(calculator::TermParser::new().parse("22").is_ok());
    assert!(calculator::TermParser::new().parse("(22)").is_ok());
    assert!(calculator::TermParser::new().parse("((((22))))").is_ok());
    assert!(calculator::TermParser::new().parse("((22)").is_err());
}

lalrpop_mod!(pub fun); // synthesized by LALRPOP

pub mod ast;
use crate::ast::Expr;

fn unpack(b: Box<Expr>) -> String {
    let temp = *b;
    return temp.to_string();
}

#[test]
fn fun1() {
    assert!(fun::ExprParser::new().parse("2 + 2").is_ok());
    assert!(unpack(fun::ExprParser::new().parse("2 + 2").unwrap()) == "(+ 2 2)");
    assert!(unpack(fun::ExprParser::new().parse("2 - 2").unwrap()) == "(- 2 2)");
    assert!(unpack(fun::ExprParser::new().parse("2").unwrap()) == "2");
    assert!(unpack(fun::ExprParser::new().parse("2 + 2 - 2").unwrap()) == "(- (+ 2 2) 2)");
    assert!(fun::ExprParser::new().parse("3 -").is_err());
}

#[test]
fn fun2() {
    assert!(fun::ExprsParser::new().parse("2").is_ok());
}

#[test]
fn fun3() {
    assert!(fun::ExprsParser::new().parse("2 + 2 \n 3 - 1").is_ok());
}

#[test]
fn fun4() {
    assert!(fun::ExprsParser::new()
        .parse("2 + 2 \n 3 - 1 \n 12")
        .is_ok());
}

fn check(t: &str) -> i32 {
    let temp = calculator::TermParser::new().parse(t);
    if temp.is_ok() {
        return temp.unwrap();
    } else {
        return -1;
    }
}

use std::fs;
fn main() {
    // Open a file
    // Read its content
    // Write it down in prefix notation
    let filename = "example";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);

    //println!("Hello, world!");
    //println!("{}", check("22"));
    //println!("{}", check("(22"));
    //println!("{}", check("(22)"));
    //println!("{}", check("(((22)))"));
    //println!("{}", unpack(fun::ExprParser::new().parse("2").unwrap()));
    //println!("{}", unpack(fun::ExprParser::new().parse("2+2").unwrap()));
    //println!("{}", unpack(fun::ExprsParser::new().parse("2-2").unwrap()));

    for item in fun::ExprsParser::new().parse(&contents).unwrap().iter() {
        println!("{}", item.to_string());
    }
}
