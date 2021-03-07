use crate::ast;

mod types;

/// Infer types for all the bindings
pub fn simplify(program: ast::Program) -> ast::Program {
    types::simplify(program)
}
