use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub enum Bool {
    And(Box<Bool>, Box<Bool>),
    Or(Box<Bool>, Box<Bool>),
    Not(Box<Bool>),
    Variable(Variable),
    True,
    False,
}

impl fmt::Display for Bool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Bool::And(a, b) => write!(f, "AND: {} {}", a, b),
            Bool::Or(a, b) => write!(f, "OR: {} {}", a, b),
            Bool::Not(a) => write!(f, "NOT: {}", a),
            Bool::Variable(a) => write!(f, "VAR: {}", a),
            Bool::True => write!(f, "true"),
            Bool::False => write!(f, "false"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
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

#[derive(PartialEq, Clone, Debug)]
pub enum Type {
    Bool,
    I32,
    Tuple(Box<Type>),
    Vector(Box<Type>),
    // How to handle pointer type?
    Pointer(Box<Type>),
    // type of the array and its length
    Array(Box<Type>, i32),
    Unit,
    // placeholder to be used when parsing expressions with nothing on the right side (infer is needed)
    Unknown,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::I32 => write!(f, "Int"),
            Type::Bool => write!(f, "Bool"),
            Type::Tuple(a) => write!(f, "Tuple({})", a),
            Type::Vector(a) => write!(f, "Vector({})", a),
            Type::Pointer(a) => write!(f, "Pointer({})", a),
            Type::Array(a, l) => write!(f, "Array({}, {})", a, l),
            Type::Unit => write!(f, "Unit"),
            Type::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Value {
    Expr(Expr),
    Bool(Bool),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Expr(expr) => write!(f, "{}", expr),
            Value::Bool(val) => write!(f, "{}", val),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
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

#[derive(PartialEq, Clone, Debug)]
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

#[derive(PartialEq, Clone, Debug)]
pub enum Variable {
    Named(String),
    Value(Box<Value>),
    // function name, arguments, output type
    FunctionCall(String, Vec<Variable>, Type),
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Variable::Named(x) => write!(f, "Named({})", x),
            Variable::Value(x) => write!(f, "Value({})", x),
            Variable::FunctionCall(name, input, t) => {
                write!(f, "FunctionCall({}, {:?}, {})", name, input, t)
            }
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
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

#[derive(PartialEq, Clone, Debug)]
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

pub enum Block {
    // vector of conditions for if/elif, vector of vectors of commands for if/elif, vector of commands for else
    If(Vec<Bool>, Vec<Vec<Command>>, Vec<Command>),
    // iterator's name, first range elem, second range elem, commands
    ForRange(Variable, Variable, Variable, Vec<Command>),
    // TODO: Handle vector iterator somehow?
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Block::If(conds, comms, el) => {
                let mut temp = String::new();
                for i in conds.iter().zip(comms.iter()) {
                    let (cond, comm) = i;
                    temp += &format!("if ({}) ({:?}) el", cond, comm).to_owned();
                }
                temp += &format!("se ({:?})", el).to_owned();
                write!(f, "{}", temp)
            }
            Block::ForRange(i, a, b, comms) => {
                let mut temp = String::new();
                for i in comms.iter() {
                    temp += &format!("{:?}", i).to_owned();
                }

                write!(f, "for {} in range {}..{} ({})", i, a, b, temp)
            }
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Function {
    pub name: String,
    pub content: Vec<Command>,
    pub input: Vec<Binding>,
    // single type, as returning multiple values requires tuple anyway
    pub output: Type,
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut temp: String = "".to_owned();
        for item in self.content.iter() {
            temp += &item.to_string();
            temp += "\t";
        }
        let mut input: String = "".to_owned();
        for item in self.input.iter() {
            temp += &input.to_string();
            temp += "\t";
        }
        write!(
            f,
            "fn {}({}) -> {} () {}",
            self.name, input, self.output, temp
        )
    }
}
