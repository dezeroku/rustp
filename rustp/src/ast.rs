use std::fmt;

#[derive(PartialEq, Clone)]
pub enum ProveControl {
    Assert(String),
}

impl fmt::Display for ProveControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProveControl::Assert(x) => write!(f, "(assert {})", x),
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum Type {
    I32,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::I32 => write!(f, "Int"),
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum Value {
    I32(i32),
    Expr(Expr),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::I32(val) => write!(f, "{}", val),
            Value::Expr(expr) => write!(f, "{}", expr),
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum Command {
    Binding(Binding),
    ProveControl(ProveControl),
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Binding(x) => write!(f, "{}", x),
            Command::ProveControl(x) => write!(f, "{}", x),
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum Binding {
    // name, type
    Declaration(Variable, Type),
    Assignment(Variable, Type, Value),
}

impl fmt::Display for Binding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Binding::Declaration(name, t) => write!(f, "(declare-const {} {})", name, t),
            Binding::Assignment(name, t, val) => write!(
                f,
                "(declare-const {} {}); (assert (= {} {}));",
                name, t, name, val
            ),
        }
    }
}

#[derive(PartialEq, Clone)]
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

#[derive(PartialEq, Clone)]
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

#[derive(PartialEq, Clone)]
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
    pub content: Vec<Command>,
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
