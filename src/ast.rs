use std::collections::HashSet;
use std::fmt;

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

#[derive(PartialEq, Clone, Debug)]
pub struct Program {
    pub content: Vec<Function>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut temp: String = "".to_owned();
        for item in self.content.iter() {
            temp += &item.to_string();
            temp += "\t";
        }
        write!(f, "{}", temp)
    }
}

#[derive(PartialEq, Clone, Debug, Eq, Hash)]
pub enum Bool {
    And(Box<Bool>, Box<Bool>),
    Or(Box<Bool>, Box<Bool>),
    Not(Box<Bool>),
    Value(Box<Value>),
    True,
    False,
    Equal(Expr, Expr),
    GreaterEqual(Expr, Expr),
    LowerEqual(Expr, Expr),
    GreaterThan(Expr, Expr),
    LowerThan(Expr, Expr),
}

impl fmt::Display for Bool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Bool::And(a, b) => write!(f, "{} && {}", a, b),
            Bool::Or(a, b) => write!(f, "{} || {}", a, b),
            Bool::Not(a) => write!(f, "!{}", a),
            Bool::Value(a) => write!(f, "{}", a),
            Bool::True => write!(f, "true"),
            Bool::False => write!(f, "false"),
            Bool::Equal(a, b) => write!(f, "{} == {}", a, b),
            Bool::GreaterThan(a, b) => write!(f, "{} > {}", a, b),
            Bool::LowerThan(a, b) => write!(f, "{} < {}", a, b),
            Bool::GreaterEqual(a, b) => write!(f, "{} >= {}", a, b),
            Bool::LowerEqual(a, b) => write!(f, "{} <= {}", a, b),
        }
    }
}

#[derive(PartialEq, Clone, Debug, Hash, Eq)]
pub enum ProveControl {
    Assert(Bool),
    Assume(Bool),
}

impl fmt::Display for ProveControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProveControl::Assert(x) => write!(f, "(assert {})", x),
            ProveControl::Assume(x) => write!(f, "(assume {})", x),
        }
    }
}

pub trait ProveControlFuncs {
    fn get_bool(self) -> Bool;
}

impl ProveControlFuncs for ProveControl {
    fn get_bool(self) -> Bool {
        match self {
            ProveControl::Assert(a) => a,
            ProveControl::Assume(a) => a,
        }
    }
}

#[derive(PartialEq, Clone, Debug, Hash, Eq)]
pub enum Type {
    Bool,
    I32,
    /// no nested tuples for now, although this can be easily lifted if needed
    Tuple(Vec<Type>),
    Reference(Box<Type>),
    ReferenceMutable(Box<Type>),
    /// type of the array and its length
    Array(Box<Type>, i32),
    Unit,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::I32 => write!(f, "i32"),
            Type::Bool => write!(f, "bool"),
            Type::Tuple(a) => write!(f, "Tuple({:?})", a),
            Type::Reference(a) => write!(f, "&({})", a),
            Type::ReferenceMutable(a) => write!(f, "mut&({})", a),
            Type::Array(a, l) => write!(f, "[{};{}]", a, l),
            Type::Unit => write!(f, "()"),
        }
    }
}

#[derive(PartialEq, Clone, Debug, Eq, Hash)]
pub enum Value {
    Expr(Expr),
    Bool(Bool),
    Variable(Variable),
    Tuple(Vec<Value>),
    Array(Vec<Value>),
    // function name, arguments, output type
    FunctionCall(String, Vec<Value>),
    Dereference(Box<Value>),
    Reference(Box<Value>),
    ReferenceMutable(Box<Value>),
    Unit,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Expr(expr) => write!(f, "{}", expr),
            Value::Bool(val) => write!(f, "{}", val),
            Value::Variable(var) => write!(f, "{}", var),
            Value::Tuple(tup) => write!(f, "{:?}", tup),
            Value::Array(arr) => write!(f, "{:?}", arr),
            Value::Unit => write!(f, "()"),
            Value::Dereference(x) => write!(f, "{:?}", x),
            Value::Reference(x) => write!(f, "{:?}", x),
            Value::ReferenceMutable(x) => write!(f, "{:?}", x),
            Value::FunctionCall(name, input) => {
                write!(f, "FunctionCall({}, {:?})", name, input)
            }
        }
    }
}

#[derive(PartialEq, Clone, Debug, Hash, Eq)]
pub enum Command {
    Binding(Binding),
    Assignment(Assignment),
    ProveControl(ProveControl),
    Block(Block),
    Noop,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Binding(x) => write!(f, "{}", x),
            Command::Assignment(x) => write!(f, "{}", x),
            Command::ProveControl(x) => write!(f, "{}", x),
            Command::Block(x) => write!(f, "{}", x),
            Command::Noop => write!(f, "noop"),
        }
    }
}

#[derive(PartialEq, Clone, Debug, Hash, Eq)]
pub enum Assignment {
    Tuple(Vec<Assignment>),
    /// Variable has to be already defined via binding to be assigned
    Single(Variable, Value),
}

impl fmt::Display for Assignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Assignment::Tuple(v) => write!(f, "{:?}", v),
            Assignment::Single(v, val) => write!(f, "{} = {}", v, val),
        }
    }
}

#[derive(PartialEq, Clone, Debug, Hash, Eq)]
pub enum Binding {
    /// name, type, is_mutable
    Declaration(Variable, Type, bool),
    /// name, type, value, is_mutable
    Assignment(Variable, Type, Value, bool),
    Tuple(Vec<Command>),
}

impl fmt::Display for Binding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Binding::Declaration(name, t, m) => write!(f, "let {} {}: {};\n", m, name, t),
            Binding::Tuple(x) => write!(f, "{:?}", x),
            Binding::Assignment(name, t, val, m) => {
                write!(f, "let {} {}: {} =  {};\n", m, name, t, val)
            }
        }
    }
}

#[derive(PartialEq, Clone, Debug, Eq, Hash)]
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

#[derive(PartialEq, Clone, Debug, Eq, Hash)]
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

#[derive(PartialEq, Clone, Debug, Eq, Hash)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
    /// The % in rust is actually remainder, not modulo
    Rem,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Opcode::Mul => write!(f, "*"),
            Opcode::Div => write!(f, "/"),
            Opcode::Add => write!(f, "+"),
            Opcode::Sub => write!(f, "-"),
            Opcode::Rem => write!(f, "%"),
        }
    }
}

#[derive(PartialEq, Clone, Debug, Hash, Eq)]
pub enum Block {
    /// vector of conditions for if/elif, vector of vectors of commands for if/elif, vector of commands for else
    If(Vec<Bool>, Vec<Vec<Command>>, Vec<Command>),
    /// iterator's name, first range elem, second range elem, commands, invariant
    ForRange(Variable, Value, Value, Vec<Command>, Bool),
    /// condition, commands, invariant
    While(Bool, Vec<Command>, Bool),
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Block::If(conds, comms, el) => {
                let mut temp = String::new();
                for i in conds.iter().zip(comms.iter()) {
                    let (cond, comm) = i;
                    temp += &format!("if ({}) \n({:?}) el", cond, comm).to_owned();
                }
                temp += &format!("se ({:?})", el).to_owned();
                write!(f, "{}", temp)
            }
            Block::ForRange(i, a, b, comms, inv) => {
                let mut temp = String::new();
                for i in comms.iter() {
                    temp += &format!("{}", i).to_owned();
                }

                write!(
                    f,
                    "[{}] for {} in range {}..{} (\n{}\n)",
                    inv, i, a, b, temp
                )
            }
            Block::While(c, comms, inv) => {
                let mut temp = String::new();
                for i in comms.iter() {
                    temp += &format!("{}", i).to_owned();
                }

                write!(f, "[{}] while {} (\n{}\n)", inv, c, temp)
            }
        }
    }
}

#[derive(PartialEq, Clone, Debug, Hash, Eq)]
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
    pub return_value: Value,
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut temp: String = "".to_owned();
        for item in self.content.iter() {
            temp += &format!("{}", &item);
            temp += "\t";
        }
        let mut input: String = "".to_owned();
        for item in self.input.iter() {
            input += &item.to_string();
            input += "\t";
        }
        write!(
            f,
            "fn {}({}) -> {} () \n{}",
            self.name, input, self.output, temp
        )
    }
}

/// Update the precondition with proper 'old assertions
pub trait PreconditionCreator {
    fn update_precondition(self) -> Self;
}

impl PreconditionCreator for Function {
    fn update_precondition(self) -> Self {
        let mut t = self;
        let inputs = t.input.clone();
        let mut vars = HashSet::new();
        for i in inputs {
            vars.extend(i.get_affected_variables());
        }

        // Actually add the 'old == original assertions
        // TODO: IMPORTANT: this should actually take types in the matter, how are we supposed to compare bools?
        for i in vars {
            t.precondition = Bool::And(Box::new(t.precondition), Box::new(i.old_wrapper()));
        }

        t
    }
}

/// Create a bool condition setting var == var'old
trait OldWrapper {
    fn old_wrapper(self) -> Bool;
}

impl OldWrapper for Variable {
    fn old_wrapper(self) -> Bool {
        match self {
            Variable::Named(x) => Bool::Equal(
                Expr::Value(Box::new(Value::Variable(Variable::Named(x.clone())))),
                Expr::Value(Box::new(Value::Variable(Variable::Named(
                    x.clone() + "'old",
                )))),
            ),
            Variable::Empty => Bool::True,
            Variable::ArrayElem(_, _a) => {
                unimplemented!();
            }
            Variable::TupleElem(_, _a) => {
                unimplemented!();
            }
        }
    }
}

/// List all the variables that are assigned to
pub trait AffectedVarGetter {
    fn get_affected_variables(self) -> HashSet<Variable>;
}

impl AffectedVarGetter for Assignment {
    fn get_affected_variables(self) -> HashSet<Variable> {
        match self {
            Assignment::Tuple(v) => {
                let mut a = HashSet::new();
                for i in v {
                    let t = i.get_affected_variables();
                    // merge a and t
                    a.extend(t);
                }
                a
            }
            Assignment::Single(var, _) => var.get_variables(),
        }
    }
}

impl AffectedVarGetter for Binding {
    fn get_affected_variables(self) -> HashSet<Variable> {
        match self {
            Binding::Declaration(var, _, _) => var.get_variables(),
            Binding::Assignment(var, _, _, _) => var.get_variables(),
            Binding::Tuple(vec) => {
                let mut a = HashSet::new();
                for i in vec {
                    let t = i.get_affected_variables();
                    a.extend(t);
                }

                a
            }
        }
    }
}

impl AffectedVarGetter for Block {
    fn get_affected_variables(self) -> HashSet<Variable> {
        match self {
            Block::If(_, ifs, el) => {
                let mut a = HashSet::new();

                for j in ifs {
                    for i in j {
                        let t = i.get_affected_variables();
                        a.extend(t);
                    }
                }

                for i in el {
                    let t = i.get_affected_variables();
                    a.extend(t);
                }

                a
            }
            Block::ForRange(var, _, _, vec, _) => {
                let mut a = var.get_variables();

                for i in vec {
                    let t = i.get_affected_variables();
                    a.extend(t);
                }

                a
            }
            Block::While(_, vec, _) => {
                let mut a = HashSet::new();
                for i in vec {
                    let t = i.get_affected_variables();
                    a.extend(t);
                }

                a
            }
        }
    }
}

impl AffectedVarGetter for Command {
    fn get_affected_variables(self) -> HashSet<Variable> {
        match self {
            Command::Binding(a) => a.get_affected_variables(),
            Command::Assignment(a) => a.get_affected_variables(),
            Command::ProveControl(_) => HashSet::new(),
            Command::Block(a) => a.get_affected_variables(),
            Command::Noop => HashSet::new(),
        }
    }
}

/// List all the variables that are used
pub trait VarGetter {
    fn get_variables(self) -> HashSet<Variable>;
}

impl VarGetter for Assignment {
    fn get_variables(self) -> HashSet<Variable> {
        match self {
            Assignment::Tuple(v) => {
                let mut a = HashSet::new();
                for i in v {
                    let t = i.get_variables();
                    // merge a and t
                    a.extend(t);
                }
                a
            }
            Assignment::Single(var, val) => {
                let mut a = var.get_variables();
                let f = val.get_variables();
                a.extend(f);
                a
            }
        }
    }
}

impl VarGetter for Binding {
    fn get_variables(self) -> HashSet<Variable> {
        match self {
            Binding::Declaration(var, _, _) => var.get_variables(),
            Binding::Assignment(var, _, val, _) => {
                let mut a = var.get_variables();
                a.extend(val.get_variables());
                a
            }
            Binding::Tuple(vec) => {
                let mut a = HashSet::new();
                for i in vec {
                    let t = i.get_variables();
                    a.extend(t);
                }

                a
            }
        }
    }
}

impl VarGetter for Block {
    fn get_variables(self) -> HashSet<Variable> {
        match self {
            Block::If(conds, ifs, el) => {
                let mut a = HashSet::new();

                for i in conds {
                    let t = i.get_variables();
                    a.extend(t);
                }

                for j in ifs {
                    for i in j {
                        let t = i.get_variables();
                        a.extend(t);
                    }
                }

                for i in el {
                    let t = i.get_variables();
                    a.extend(t);
                }

                a
            }
            Block::ForRange(var, first, last, vec, c) => {
                let mut a = var.get_variables();
                a.extend(first.get_variables());
                a.extend(last.get_variables());
                a.extend(c.get_variables());

                for i in vec {
                    let t = i.get_variables();
                    a.extend(t);
                }

                a
            }
            Block::While(b, vec, c) => {
                let mut a = b.get_variables();
                a.extend(c.get_variables());

                for i in vec {
                    let t = i.get_variables();
                    a.extend(t);
                }

                a
            }
        }
    }
}

impl VarGetter for Bool {
    fn get_variables(self) -> HashSet<Variable> {
        match self {
            Bool::And(a, b) => {
                let mut t = a.get_variables();
                t.extend(b.get_variables());

                t
            }
            Bool::Or(a, b) => {
                let mut t = a.get_variables();
                t.extend(b.get_variables());

                t
            }
            Bool::Not(a) => a.get_variables(),
            Bool::Value(a) => a.get_variables(),
            Bool::True => HashSet::new(),
            Bool::False => HashSet::new(),
            Bool::Equal(a, b) => {
                let mut t = a.get_variables();
                t.extend(b.get_variables());

                t
            }
            Bool::GreaterEqual(a, b) => {
                let mut t = a.get_variables();
                t.extend(b.get_variables());

                t
            }
            Bool::LowerEqual(a, b) => {
                let mut t = a.get_variables();
                t.extend(b.get_variables());

                t
            }
            Bool::GreaterThan(a, b) => {
                let mut t = a.get_variables();
                t.extend(b.get_variables());

                t
            }
            Bool::LowerThan(a, b) => {
                let mut t = a.get_variables();
                t.extend(b.get_variables());

                t
            }
        }
    }
}

impl VarGetter for Command {
    fn get_variables(self) -> HashSet<Variable> {
        match self {
            Command::Binding(a) => a.get_variables(),
            Command::Assignment(a) => a.get_variables(),
            Command::ProveControl(a) => a.get_variables(),
            Command::Block(a) => a.get_variables(),
            Command::Noop => HashSet::new(),
        }
    }
}

impl VarGetter for Expr {
    fn get_variables(self) -> HashSet<Variable> {
        match self {
            Expr::Number(_) => HashSet::new(),
            Expr::Value(v) => v.get_variables(),
            Expr::Op(a, _, b) => {
                let mut t = a.get_variables();
                t.extend(b.get_variables());

                t
            }
        }
    }
}

impl VarGetter for ProveControl {
    fn get_variables(self) -> HashSet<Variable> {
        self.get_bool().get_variables()
    }
}

impl VarGetter for Value {
    fn get_variables(self) -> HashSet<Variable> {
        match self {
            Value::Expr(a) => a.get_variables(),
            Value::Bool(a) => a.get_variables(),
            Value::Variable(a) => a.get_variables(),
            Value::Tuple(vals) => {
                let mut a = HashSet::new();
                for i in vals {
                    let t = i.get_variables();
                    a.extend(t);
                }

                a
            }
            Value::Array(vals) => {
                let mut a = HashSet::new();
                for i in vals {
                    let t = i.get_variables();
                    a.extend(t);
                }

                a
            }
            Value::FunctionCall(_, vals) => {
                let mut a = HashSet::new();
                for i in vals {
                    let t = i.get_variables();
                    a.extend(t);
                }

                a
            }
            Value::Dereference(a) => a.get_variables(),
            Value::Reference(a) => a.get_variables(),
            Value::ReferenceMutable(a) => a.get_variables(),
            Value::Unit => HashSet::new(),
        }
    }
}

impl VarGetter for Variable {
    fn get_variables(self) -> HashSet<Variable> {
        match self.clone() {
            Variable::Named(_) => set!(self),
            Variable::Empty => HashSet::new(),
            Variable::ArrayElem(_, a) => {
                let mut t = HashSet::new();
                t.insert(self);
                t.extend(a.get_variables());
                t
            }
            Variable::TupleElem(_, a) => {
                let mut t = HashSet::new();
                t.insert(self);
                t.extend(a.get_variables());
                t
            }
        }
    }
}

pub trait Swapper {
    /// Swap all the occurences of `var` with `val`
    fn swap(self, var: Variable, val: Value) -> Self;
}

impl Swapper for Bool {
    fn swap(self, var: Variable, val: Value) -> Self {
        match self.clone() {
            Bool::And(a, b) => {
                log::trace!("Bool::And::swap: {} {} {}", self, var, val);
                Bool::And(
                    Box::new(a.swap(var.clone(), val.clone())),
                    Box::new(b.swap(var, val)),
                )
            }
            Bool::Or(a, b) => {
                log::trace!("Bool::Or::swap: {} {} {}", self, var, val);
                Bool::Or(
                    Box::new(a.swap(var.clone(), val.clone())),
                    Box::new(b.swap(var, val)),
                )
            }
            Bool::Not(a) => {
                log::trace!("Bool::Not::swap: {} {} {}", self, var, val);
                Bool::Not(Box::new(a.swap(var.clone(), val.clone())))
            }
            Bool::Value(a) => {
                log::trace!("Bool::Value::swap: {} {} {}", self, var, val);
                Bool::Value(Box::new(a.swap(var.clone(), val.clone())))
            }
            Bool::True => {
                log::trace!("Bool::True::swap: {} {} {}", self, var, val);
                Bool::True
            }
            Bool::False => {
                log::trace!("Bool::False::swap: {} {} {}", self, var, val);
                Bool::False
            }
            Bool::Equal(a, b) => {
                log::trace!("Bool::Equal::swap: {} {} {}", self, var, val);
                Bool::Equal(a.swap(var.clone(), val.clone()), b.swap(var, val))
            }
            Bool::GreaterEqual(a, b) => {
                log::trace!("Bool::GreaterEqual::swap: {} {} {}", self, var, val);
                Bool::GreaterEqual(a.swap(var.clone(), val.clone()), b.swap(var, val))
            }
            Bool::LowerEqual(a, b) => {
                log::trace!("Bool::LowerEqual::swap: {} {} {}", self, var, val);
                Bool::LowerEqual(a.swap(var.clone(), val.clone()), b.swap(var, val))
            }
            Bool::GreaterThan(a, b) => {
                log::trace!("Bool::GreaterThan::swap: {} {} {}", self, var, val);
                Bool::GreaterThan(a.swap(var.clone(), val.clone()), b.swap(var, val))
            }
            Bool::LowerThan(a, b) => {
                log::trace!("Bool::LowerThan::swap: {} {} {}", self, var, val);
                Bool::LowerThan(a.swap(var.clone(), val.clone()), b.swap(var, val))
            }
        }
    }
}

impl Swapper for Expr {
    fn swap(self, var: Variable, val: Value) -> Self {
        match self.clone() {
            Expr::Number(_) => {
                log::trace!("Expr::Number::swap: {} {} {}", self, var, val);
                self
            }
            Expr::Op(a, op, b) => {
                log::trace!("Expr::Op::swap: {} {} {}", self, var, val);
                Expr::Op(
                    Box::new(a.swap(var.clone(), val.clone())),
                    op,
                    Box::new(b.swap(var, val)),
                )
            }
            Expr::Value(v) => {
                log::trace!("Expr::Value::swap: {} {} {}", self, var, val);
                Expr::Value(Box::new(v.swap(var, val)))
            }
        }
    }
}

impl Swapper for Value {
    fn swap(self, var: Variable, val: Value) -> Self {
        match self.clone() {
            Value::Expr(a) => {
                log::trace!("Value::Expr::swap: {} {} {}", self, var, val);
                Value::Expr(a.swap(var, val))
            }
            Value::Bool(a) => {
                log::trace!("Value::Bool::swap: {} {} {}", self, var, val);
                Value::Bool(a.swap(var, val))
            }
            Value::Variable(a) => {
                log::trace!("Value::Variable::swap: {} {} {}", self, var, val);
                if a == var {
                    val
                } else {
                    Value::Variable(a)
                }
            }
            Value::Tuple(vec) => {
                log::trace!("Value::Tuple::swap: {} {} {}", self, var, val);
                let mut res = Vec::new();
                for i in vec {
                    res.push(i.swap(var.clone(), val.clone()));
                }
                Value::Tuple(res)
            }
            Value::Array(vec) => {
                log::trace!("Value::Array::swap: {} {} {}", self, var, val);
                let mut res = Vec::new();
                for i in vec {
                    res.push(i.swap(var.clone(), val.clone()));
                }
                Value::Array(res)
            }
            Value::FunctionCall(name, vec) => {
                log::trace!("Value::FunctionCall::swap: {} {} {}", self, var, val);
                let mut res = Vec::new();
                for i in vec {
                    res.push(i.swap(var.clone(), val.clone()));
                }
                Value::FunctionCall(name, res)
            }
            Value::Dereference(a) => {
                log::trace!("Value::Dereference::swap: {} {} {}", self, var, val);
                Value::Dereference(Box::new(a.swap(var, val)))
            }
            Value::Reference(a) => {
                log::trace!("Value::Reference::swap: {} {} {}", self, var, val);
                Value::Reference(Box::new(a.swap(var, val)))
            }
            Value::ReferenceMutable(a) => {
                log::trace!("Value::ReferenceMutable::swap: {} {} {}", self, var, val);
                Value::ReferenceMutable(Box::new(a.swap(var, val)))
            }
            Value::Unit => {
                log::trace!("Value::Unit::swap: {} {} {}", self, var, val);
                Value::Unit
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
            Binding::Declaration(Variable::Named(String::from("x")), Type::I32, false)
                .get_variables(),
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
}
