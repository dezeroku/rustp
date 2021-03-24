use crate::ast;

mod types;

pub fn simplify(program: ast::Program) -> ast::Program {
    types::simplify(program)
}
