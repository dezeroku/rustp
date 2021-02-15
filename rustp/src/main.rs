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

fn check(t: &str) -> i32 {
    let temp = calculator::TermParser::new().parse(t);
    if temp.is_ok() {
        return temp.unwrap();
    } else {
        return -1;
    }
}

fn main() {
    println!("Hello, world!");
    println!("{}", check("22"));
    println!("{}", check("(22"));
    println!("{}", check("(22)"));
    println!("{}", check("(((22)))"));
    println!("{}", unpack(fun::ExprParser::new().parse("2").unwrap()));
    println!("{}", unpack(fun::ExprParser::new().parse("2+2").unwrap()));
    println!("{}", unpack(fun::ExprParser::new().parse("2-2").unwrap()));
}
