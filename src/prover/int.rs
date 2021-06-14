use crate::prover::*;

pub trait ProvableValue {
    fn as_bool<'a>(self, ctx: &'a z3::Context) -> (HashSet<Check<'a>>, z3::ast::Bool<'a>);
    fn as_int<'a>(self, ctx: &'a z3::Context) -> (HashSet<Check<'a>>, z3::ast::Int<'a>);
}

impl ProvableValue for Value {
    fn as_bool<'a>(self, ctx: &'a z3::Context) -> (HashSet<Check<'a>>, z3::ast::Bool<'a>) {
        log::trace!("AS_BOOL: {}", self.clone());
        match self {
            Value::Expr(e) => panic!("Bool value used as an int: {}", e),
            Value::Bool(b) => b.as_bool(ctx),
            Value::Variable(x) => match x {
                Variable::Named(name) => (set![], z3::ast::Bool::new_const(ctx, name)),
                Variable::Empty => panic!("Empty variable tried to be used as a bool!"),
                Variable::ArrayElem(arr_name, ind) => {
                    let t = z3::ast::Array::new_const(
                        ctx,
                        arr_name,
                        &z3::Sort::int(ctx),
                        &z3::Sort::int(ctx),
                    );

                    let (checks_ind, ind) = ind.as_int(ctx);
                    (checks_ind, t.select(&ind).as_bool().unwrap())
                }
                Variable::TupleElem(_name, _ind) => unimplemented!(),
            },
            Value::Tuple(t) => panic!("Tuple {:?} tried to be used as a bool!", t),
            Value::Array(a) => panic!("Array {:?} tried to be used as a bool!", a),
            Value::FunctionCall(_name, _args) => unimplemented!(),
            Value::Reference(_v) => unimplemented!(),
            Value::ReferenceMutable(_v) => unimplemented!(),
            Value::Dereference(_v) => unimplemented!(),
            Value::Unit => unimplemented!(),
            Value::Ternary(cond, a, b) => {
                let (mut checks_t, t) = cond.as_bool(ctx);
                let (checks_a, a) = a.as_bool(ctx);
                let (checks_b, b) = b.as_bool(ctx);
                checks_t.extend(checks_a);
                checks_t.extend(checks_b);
                (checks_t, t.ite(&a, &b))
            }
        }
    }

    fn as_int<'a>(self, ctx: &'a z3::Context) -> (HashSet<Check<'a>>, z3::ast::Int<'a>) {
        log::trace!("AS_INT: {}", self.clone());
        match self {
            Value::Expr(e) => e.as_int(ctx),
            Value::Bool(b) => {
                // HACK: This is ugly, should be solved on the parsing level somehow?
                //panic!("Bool value ({}) used as an int", b)
                log::debug!("Bool value ({}) used as an int", b);
                match b.clone() {
                    Bool::Value(a) => match *a {
                        Value::Variable(x) => {
                            Value::Expr(Expr::Value(Box::new(Value::Variable(x)))).as_int(ctx)
                        }
                        Value::Expr(x) => Value::Expr(x).as_int(ctx),
                        _ => panic!("Bool value ({}) used as an int", b),
                    },
                    _ => panic!("Bool value ({}) used as an int", b),
                }
            }
            Value::Variable(x) => match x {
                Variable::Named(name) => (set![], z3::ast::Int::new_const(ctx, name)),
                Variable::Empty => panic!("Empty variable tried to be used as an int!"),
                Variable::ArrayElem(arr_name, ind) => {
                    let t = z3::ast::Array::new_const(
                        ctx,
                        arr_name,
                        &z3::Sort::int(ctx),
                        &z3::Sort::int(ctx),
                    );

                    let (checks, ind) = ind.as_int(ctx);
                    (checks, t.select(&ind).as_int().unwrap())
                }

                Variable::TupleElem(_name, _ind) => unimplemented!(),
            },
            Value::Tuple(t) => panic!("Tuple {:?} tried to be used as an intl!", t),
            Value::Array(a) => panic!("Array {:?} tried to be used as an int!", a),
            Value::FunctionCall(_name, _args) => unimplemented!(),
            Value::Reference(_v) => unimplemented!(),
            Value::ReferenceMutable(_v) => unimplemented!(),
            Value::Dereference(_v) => unimplemented!(),
            Value::Unit => unimplemented!(),
            Value::Ternary(cond, a, b) => {
                let (mut checks_t, t) = cond.as_bool(ctx);
                let (checks_a, a) = a.as_int(ctx);
                let (checks_b, b) = b.as_int(ctx);
                checks_t.extend(checks_a);
                checks_t.extend(checks_b);
                (checks_t, t.ite(&a, &b))
            }
        }
    }
}

pub trait ProvableBool {
    fn as_bool<'a>(self, ctx: &'a z3::Context) -> (HashSet<Check<'a>>, z3::ast::Bool<'a>);
}

pub trait ProvableInt {
    fn as_int<'a>(self, ctx: &'a z3::Context) -> (HashSet<Check<'a>>, z3::ast::Int<'a>);
}

impl ProvableInt for Expr {
    fn as_int<'a>(self, ctx: &'a z3::Context) -> (HashSet<Check<'a>>, z3::ast::Int<'a>) {
        log::trace!("AS_INT: {}", self.clone());
        match self.clone() {
            Expr::Number(a) => (set![], z3::ast::Int::from_i64(ctx, a.into())),
            Expr::Op(a, op, b) => match op {
                Opcode::Add => {
                    // These checks are only done after the solver has already finished
                    let (mut checks_a, a) = a.as_int(ctx);
                    let (checks_b, b) = b.as_int(ctx);
                    checks_a.extend(checks_b);

                    (checks_a, z3::ast::Int::add(ctx, &[&a, &b]))
                }
                Opcode::Sub => {
                    // These checks are only done after the solver has already finished
                    let (mut checks_a, a) = a.as_int(ctx);
                    let (checks_b, b) = b.as_int(ctx);
                    checks_a.extend(checks_b);

                    (checks_a, z3::ast::Int::sub(ctx, &[&a, &b]))
                }
                Opcode::Mul => {
                    // These checks are only done after the solver has already finished
                    let (mut checks_a, a) = a.as_int(ctx);
                    let (checks_b, b) = b.as_int(ctx);
                    checks_a.extend(checks_b);

                    (checks_a, z3::ast::Int::mul(ctx, &[&a, &b]))
                }
                Opcode::Div => {
                    // These checks are only done after the solver has already finished
                    let (mut checks_a, a) = a.as_int(ctx);
                    let (checks_b, b) = b.as_int(ctx);
                    checks_a.extend(checks_b);

                    (checks_a, a.div(&b))
                }
                Opcode::Rem => {
                    let (mut checks_a, a) = a.as_int(ctx);
                    let (checks_b, b) = b.as_int(ctx);
                    checks_a.extend(checks_b);

                    (checks_a, a.rem(&b))
                }
            },
            Expr::Value(a) => a.as_int(ctx),
        }
    }
}

impl ProvableBool for Bool {
    fn as_bool<'a>(self, ctx: &'a z3::Context) -> (HashSet<Check<'a>>, z3::ast::Bool<'a>) {
        log::trace!("AS_BOOL: {}", self.clone());
        match self {
            Bool::ForAll(var, b) => {
                let (mut checks_var, var) = Value::Variable(var).as_int(ctx);
                let (checks_b, b) = b.as_bool(ctx);
                let forall: z3::ast::Bool =
                    z3::ast::forall_const(ctx, &[&var.clone().into()], &[], &b)
                        .try_into()
                        .unwrap();

                checks_var.extend(checks_b);
                (checks_var, forall)
            }
            Bool::Exists(var, b) => {
                let (mut checks_var, var) = Value::Variable(var).as_int(ctx);
                let (checks_b, b) = b.as_bool(ctx);
                let exists: z3::ast::Bool =
                    z3::ast::exists_const(ctx, &[&var.clone().into()], &[], &b)
                        .try_into()
                        .unwrap();

                checks_var.extend(checks_b);
                (checks_var, exists)
            }
            Bool::True => (set![], (z3::ast::Bool::from_bool(&ctx, true))),
            Bool::False => (set![], (z3::ast::Bool::from_bool(&ctx, false))),
            Bool::And(_a, _b) => {
                let (mut checks_a, a) = (*_a).as_bool(ctx);
                let (checks_b, b) = (*_b).as_bool(ctx);
                checks_a.extend(checks_b);
                (checks_a, z3::ast::Bool::and(ctx, &[&a, &b]))
            }
            Bool::Or(_a, _b) => {
                let (mut checks_a, a) = (*_a).as_bool(ctx);
                let (checks_b, b) = (*_b).as_bool(ctx);
                checks_a.extend(checks_b);
                (checks_a, z3::ast::Bool::or(ctx, &[&a, &b]))
            }
            Bool::Not(a) => {
                let (checks, t) = a.as_bool(ctx);
                (checks, z3::ast::Bool::not(&t))
            }
            Bool::ValueEqual(a, b) => {
                let (mut checks_a, a) = a.as_int(ctx);
                let (checks_b, b) = b.as_int(ctx);
                checks_a.extend(checks_b);
                (checks_a, z3::ast::Bool::and(ctx, &[&a.ge(&b), &a.le(&b)]))
            }
            Bool::Equal(a, b) => {
                let (mut checks_a, a) = a.as_int(ctx);
                let (checks_b, b) = b.as_int(ctx);
                checks_a.extend(checks_b);
                (checks_a, z3::ast::Bool::and(ctx, &[&a.ge(&b), &a.le(&b)]))
            }
            Bool::GreaterEqual(a, b) => {
                let (mut checks_a, a) = a.as_int(ctx);
                let (checks_b, b) = b.as_int(ctx);
                checks_a.extend(checks_b);

                (checks_a, a.ge(&b))
            }
            Bool::LowerEqual(a, b) => {
                let (mut checks_a, a) = a.as_int(ctx);
                let (checks_b, b) = b.as_int(ctx);
                checks_a.extend(checks_b);

                (checks_a, a.le(&b))
            }
            Bool::GreaterThan(a, b) => {
                let (mut checks_a, a) = a.as_int(ctx);
                let (checks_b, b) = b.as_int(ctx);
                checks_a.extend(checks_b);

                (checks_a, a.gt(&b))
            }
            Bool::LowerThan(a, b) => {
                let (checks_a, a) = a.as_int(ctx);
                let (checks_b, b) = b.as_int(ctx);
                checks_a.union(&checks_b);

                (checks_a, a.lt(&b))
            }
            Bool::Value(v) => v.as_bool(ctx),
        }
    }
}

pub trait ProvableCommand {
    fn as_bool<'a>(self, ctx: &'a z3::Context) -> (bool, HashSet<Check<'a>>, z3::ast::Bool<'a>);
}

impl ProvableCommand for Command {
    // Concept: Try to prove not (negate everything) to try to find if there exists an incorrect mapping?
    fn as_bool<'a>(self, ctx: &'a z3::Context) -> (bool, HashSet<Check<'a>>, z3::ast::Bool<'a>) {
        log::trace!("AS_BOOL: {}", self.clone());
        match self {
            Command::ProveControl(a) => match a {
                // We are trying to find COUNTER example here.
                // So if we get sat, then it means that the assertion is actually incorrect
                ProveControl::Assert(b) => {
                    let (checks, t) = b.as_bool(ctx);
                    (true, checks, t.not())
                }
            },
            _ => {
                // Nothing to prove here
                (false, set![], z3::ast::Bool::from_bool(&ctx, true))
            }
        }
    }
}
