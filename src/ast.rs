use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub enum Bool {
    And(Box<Bool>, Box<Bool>),
    Or(Box<Bool>, Box<Bool>),
    Not(Box<Bool>),
    Value(Box<Value>),
    True,
    False,
    Equal(Expr, Expr),
    GreaterEqual(Expr, Expr),
    SmallerEqual(Expr, Expr),
    Greater(Expr, Expr),
    Smaller(Expr, Expr),
}

impl fmt::Display for Bool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Bool::And(a, b) => write!(f, "AND: {} {}", a, b),
            Bool::Or(a, b) => write!(f, "OR: {} {}", a, b),
            Bool::Not(a) => write!(f, "NOT: {}", a),
            Bool::Value(a) => write!(f, "VAL: {}", a),
            Bool::True => write!(f, "true"),
            Bool::False => write!(f, "false"),
            Bool::Equal(a, b) => write!(f, "==: {} {}", a, b),
            Bool::Greater(a, b) => write!(f, ">: {} {}", a, b),
            Bool::Smaller(a, b) => write!(f, "<: {} {}", a, b),
            Bool::GreaterEqual(a, b) => write!(f, ">=: {} {}", a, b),
            Bool::SmallerEqual(a, b) => write!(f, "<=: {} {}", a, b),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum ProveControl {
    Assert(Bool),
    Assume(Bool),
    // It may be possible that this is a duplicate of assert in Z3's context?
    // It also has to be checked later on in validator if it's defined in the loop and error thrown if that's not true
    LoopInvariant(Bool),
}

impl fmt::Display for ProveControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProveControl::Assert(x) => write!(f, "(assert {})", x),
            ProveControl::Assume(x) => write!(f, "(assume {})", x),
            ProveControl::LoopInvariant(x) => write!(f, "(loop_invariant {})", x),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Type {
    Bool,
    I32,
    /// no nested tuples for now, although this can be easily lifted if needed
    Tuple(Vec<Type>),
    Pointer(Box<Type>),
    MutablePointer(Box<Type>),
    /// type of the array and its length
    Array(Box<Type>, i32),
    Unit,
    /// placeholder to be used when parsing expressions with nothing on the right side (infer is needed)
    Unknown,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::I32 => write!(f, "Int"),
            Type::Bool => write!(f, "Bool"),
            Type::Tuple(a) => write!(f, "Tuple({:?})", a),
            Type::Pointer(a) => write!(f, "Pointer({})", a),
            Type::MutablePointer(a) => write!(f, "MutablePointer({})", a),
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
    Variable(Variable),
    Tuple(Vec<Value>),
    Array(Vec<Value>),
    // function name, arguments, output type
    FunctionCall(String, Vec<Value>),
    Dereference(Box<Value>),
    TakeAddress(Box<Value>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Expr(expr) => write!(f, "{}", expr),
            Value::Bool(val) => write!(f, "{}", val),
            Value::Variable(var) => write!(f, "{}", var),
            Value::Tuple(tup) => write!(f, "{:?}", tup),
            Value::Array(arr) => write!(f, "{:?}", arr),
            Value::Dereference(x) => write!(f, "{:?}", x),
            Value::TakeAddress(x) => write!(f, "{:?}", x),
            Value::FunctionCall(name, input) => {
                write!(f, "FunctionCall({}, {:?})", name, input)
            }
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Command {
    Binding(Binding),
    Assignment(Assignment),
    ProveControl(ProveControl),
    Block(Block),
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Binding(x) => write!(f, "{}", x),
            Command::Assignment(x) => write!(f, "{:?}", x),
            Command::ProveControl(x) => write!(f, "{}", x),
            Command::Block(x) => write!(f, "{}", x),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Assignment {
    Tuple(Vec<Assignment>),
    /// Variable has to be already defined via binding to be assigned
    Single(Variable, Value),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Binding {
    /// name, type, is_mutable
    Declaration(Variable, Type, bool),
    /// name, type, value, is_mutable
    Assignment(Variable, Type, Value, bool),
    Tuple(Vec<Binding>),
}

impl fmt::Display for Binding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Binding::Declaration(name, t, m) => write!(f, "(declare-const {} {} {})", name, t, m),
            Binding::Tuple(x) => write!(f, "{:?}", x),
            Binding::Assignment(name, t, val, m) => write!(
                f,
                "(declare-const {} {}); (assert (= {} {})) {};",
                name, t, name, val, m
            ),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Variable {
    Named(String),
    /// Just a _ equivalent
    Empty,
    /// array name, index
    ArrayElem(String, Box<Value>),
    /// tuple name, index
    TupleElem(String, Box<Value>),
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Variable::Named(x) => write!(f, "Named({})", x),
            Variable::Empty => write!(f, "_"),
            Variable::ArrayElem(a, i) => write!(f, "{}[{}]", a, i),
            Variable::TupleElem(a, i) => write!(f, "{}.{}", a, i),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    Number(i32),
    Value(Box<Value>),
    Op(Box<Expr>, Opcode, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Number(x) => write!(f, "{}", x),
            Expr::Op(a, o, b) => write!(f, "({} {} {})", o, a, b),
            Expr::Value(x) => write!(f, "{}", x),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
    /// The % in rust is actually remainder, not modulo
    Mod,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Opcode::Mul => write!(f, "*"),
            Opcode::Div => write!(f, "/"),
            Opcode::Add => write!(f, "+"),
            Opcode::Sub => write!(f, "-"),
            Opcode::Mod => write!(f, "%"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Block {
    /// vector of conditions for if/elif, vector of vectors of commands for if/elif, vector of commands for else
    If(Vec<Bool>, Vec<Vec<Command>>, Vec<Command>),
    /// iterator's name, first range elem, second range elem, commands
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
    /// single type, as returning multiple values requires tuple anyway
    pub output: Type,
    /// default value is just true
    pub precondition: Bool,
    /// default value is just true
    pub postcondition: Bool,
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
            input += &item.to_string();
            input += "\t";
        }
        write!(
            f,
            "fn {}({}) -> {} () {}",
            self.name, input, self.output, temp
        )
    }
}
