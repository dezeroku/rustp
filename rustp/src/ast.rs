use std::fmt;

pub enum Clause {
    Expr(Expr),
    Assert(String),
}

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Clause::Expr(x) => write!(f, "{}", x),
            Clause::Assert(x) => write!(f, "{}", x),
        }
    }
}

#[derive(PartialEq)]
pub enum Variable {
    Named(String),
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Variable::Named(x) => write!(f, "{}", x),
        }
    }
}

#[derive(PartialEq)]
pub enum Expr {
    Number(i32),
    Variable(Variable),
    Op(Box<Expr>, Opcode, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Number(x) => write!(f, "{}", x),
            Expr::Op(a, o, b) => write!(f, "({} {} {})", o, a, b),
            Expr::Variable(x) => write!(f, "{}", x),
        }
    }
}

#[derive(PartialEq)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Opcode::Mul => write!(f, "*"),
            Opcode::Div => write!(f, "/"),
            Opcode::Add => write!(f, "+"),
            Opcode::Sub => write!(f, "-"),
        }
    }
}

pub struct Function {
    pub name: String,
    pub content: Vec<Clause>,
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut temp: String = "".to_owned();
        for item in self.content.iter() {
            temp += &item.to_string();
            temp += "\t";
        }
        write!(f, "{} {}", self.name, temp)
    }
}
