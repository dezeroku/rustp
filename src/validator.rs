use crate::ast::*;

static FORBIDDEN_DECS: [&'static str; 1] = ["return_value"];

pub fn validate(input: Program) -> bool {
    no_shadowing(input.clone()) && no_forbidden_decs(input.clone()) && no_undefined(input)
}

/// Check for using undefined variables in ProveCommands
fn no_undefined(input: Program) -> bool {
    let definitions = Vec::new();

    let mut functions = Vec::new();
    for func in input.content.clone() {
        functions.push(func.name);
    }

    for func in input.content {
        if !no_undefined_func(func, definitions.clone(), functions.clone()) {
            return false;
        }
    }

    true
}

enum Namedec {
    Variable(Variable),
    Name(String),
}

/// Unpack the bool and get all the variables that are used in it
fn get_namedecs(b: Bool) -> Vec<Namedec> {
    let mut result = Vec::new();

    _get_namedecs_bool(b, &mut result);

    result
}

fn _get_namedecs_bool(z: Bool, mut decs: &mut Vec<Namedec>) {
    match z {
        Bool::And(a, b) => {
            _get_namedecs_bool(*a, &mut decs);
            _get_namedecs_bool(*b, &mut decs);
        }
        Bool::Or(a, b) => {
            _get_namedecs_bool(*a, &mut decs);
            _get_namedecs_bool(*b, &mut decs);
        }
        Bool::Not(a) => {
            _get_namedecs_bool(*a, &mut decs);
        }
        Bool::Value(a) => {
            _get_namedecs_val(*a, &mut decs);
        }
        Bool::Equal(a, b) => {
            _get_namedecs_expr(a, &mut decs);
            _get_namedecs_expr(b, &mut decs);
        }
        Bool::GreaterEqual(a, b) => {
            _get_namedecs_expr(a, &mut decs);
            _get_namedecs_expr(b, &mut decs);
        }
        Bool::SmallerEqual(a, b) => {
            _get_namedecs_expr(a, &mut decs);
            _get_namedecs_expr(b, &mut decs);
        }
        Bool::Greater(a, b) => {
            _get_namedecs_expr(a, &mut decs);
            _get_namedecs_expr(b, &mut decs);
        }
        Bool::Smaller(a, b) => {
            _get_namedecs_expr(a, &mut decs);
            _get_namedecs_expr(b, &mut decs);
        }
        Bool::True => {}
        Bool::False => {}
    }
}

fn _get_namedecs_val(z: Value, mut decs: &mut Vec<Namedec>) {
    match z {
        Value::Expr(a) => {
            _get_namedecs_expr(a, &mut decs);
        }
        Value::Bool(a) => {
            _get_namedecs_bool(a, &mut decs);
        }
        Value::Variable(v) => {
            decs.push(Namedec::Variable(v));
        }
        Value::Tuple(a) => {
            for i in a {
                _get_namedecs_val(i, &mut decs);
            }
        }
        Value::Array(a) => {
            for i in a {
                _get_namedecs_val(i, &mut decs);
            }
        }
        Value::FunctionCall(name, a) => {
            decs.push(Namedec::Name(name));
            for i in a {
                _get_namedecs_val(i, &mut decs);
            }
        }
        Value::Dereference(a) => {
            _get_namedecs_val(*a, &mut decs);
        }
        Value::Reference(a) => {
            _get_namedecs_val(*a, &mut decs);
        }
        Value::ReferenceMutable(a) => {
            _get_namedecs_val(*a, &mut decs);
        }
        Value::Unit => {}
        Value::Unknown => {}
    }
}

fn _get_namedecs_expr(z: Expr, mut decs: &mut Vec<Namedec>) {
    match z {
        Expr::Number(_) => {}
        Expr::Op(a, _, b) => {
            _get_namedecs_expr(*a, &mut decs);
            _get_namedecs_expr(*b, &mut decs);
        }
        Expr::Value(a) => _get_namedecs_val(*a, &mut decs),
    }
}

fn def_push(definitions: &mut Vec<String>, val: Variable) {
    let name = match val {
        Variable::Named(a) => a,
        _ => return,
    };

    definitions.push(name);
}

fn no_undefined_check(
    definitions: &mut Vec<String>,
    functions: &Vec<String>,
    val: Namedec,
) -> bool {
    match val {
        Namedec::Variable(t) => {
            let name = match t {
                Variable::Named(a) => a,
                Variable::ArrayElem(a, _) => a,
                Variable::TupleElem(a, _) => a,
                _ => return true,
            };
            if !definitions.iter().any(|i| *i == name) {
                println!("Undefined variable used: {}", name);
                return false;
            }
        }
        Namedec::Name(name) => {
            if !functions.iter().any(|i| *i == name) {
                println!("Undefined function used: {}", name);
                return false;
            }
        }
    };

    true
}

fn no_undefined_logic(
    content: Vec<Command>,
    mut definitions: &mut Vec<String>,
    functions: &Vec<String>,
) -> bool {
    for comm in content {
        match comm {
            Command::ProveControl(ProveControl::Assert(a)) => {
                for i in get_namedecs(a) {
                    if !no_undefined_check(&mut definitions, &functions, i) {
                        return false;
                    }
                }
            }
            Command::ProveControl(ProveControl::Assume(a)) => {
                for i in get_namedecs(a) {
                    if !no_undefined_check(&mut definitions, &functions, i) {
                        return false;
                    }
                }
            }
            Command::ProveControl(ProveControl::LoopInvariant(a)) => {
                for i in get_namedecs(a) {
                    if !no_undefined_check(&mut definitions, &functions, i) {
                        return false;
                    }
                }
            }
            Command::Binding(Binding::Declaration(name, _, _)) => {
                def_push(&mut definitions, name);
            }
            Command::Binding(Binding::Assignment(name, _, _, _)) => {
                def_push(&mut definitions, name);
            }
            Command::Binding(Binding::Tuple(vec)) => {
                for dec in vec {
                    match dec {
                        Command::Binding(Binding::Declaration(name, _, _)) => {
                            def_push(&mut definitions, name);
                        }
                        Command::Binding(Binding::Assignment(name, _, _, _)) => {
                            def_push(&mut definitions, name);
                        }
                        _ => {}
                    }
                }
            }
            Command::Block(Block::If(_, blocks, el)) => {
                let mut temp = blocks;
                temp.push(el);

                for block in temp {
                    let mut state = definitions.clone();
                    if !no_undefined_logic(block, &mut state, &functions) {
                        return false;
                    }
                }
            }
            Command::Block(Block::ForRange(_, _, _, vec)) => {
                let mut temp = definitions.clone();

                if !no_undefined_logic(vec, &mut temp, &functions) {
                    return false;
                }
            }
            _ => {}
        }
    }

    true
}

fn no_undefined_func(func: Function, mut definitions: Vec<String>, functions: Vec<String>) -> bool {
    no_undefined_logic(func.content, &mut definitions, &functions)
}

/// Check for declarations of reserved names in declarations (like e.g. return_value)
fn no_forbidden_decs(input: Program) -> bool {
    for func in input.content {
        if !no_forbidden_decs_func(func) {
            return false;
        }
    }

    true
}

fn no_forbidden_decs_check(val: Variable) -> bool {
    match val {
        Variable::Named(a) => {
            if !FORBIDDEN_DECS.contains(&a.as_str()) {
                true
            } else {
                println!("Keyword variable used in binding: {}", a);
                false
            }
        }
        _ => true,
    }
}

fn no_forbidden_decs_logic(content: Vec<Command>) -> bool {
    for comm in content {
        match comm {
            Command::Binding(Binding::Declaration(name, _, _)) => {
                if !no_forbidden_decs_check(name) {
                    return false;
                }
            }
            Command::Binding(Binding::Assignment(name, _, _, _)) => {
                if !no_forbidden_decs_check(name) {
                    return false;
                }
            }
            Command::Binding(Binding::Tuple(vec)) => {
                for dec in vec {
                    match dec {
                        Command::Binding(Binding::Declaration(name, _, _)) => {
                            if !no_forbidden_decs_check(name) {
                                return false;
                            }
                        }
                        Command::Binding(Binding::Assignment(name, _, _, _)) => {
                            if !no_forbidden_decs_check(name) {
                                return false;
                            }
                        }
                        _ => {}
                    }
                }
            }
            Command::Block(Block::If(_, blocks, el)) => {
                let mut temp = blocks;
                temp.push(el);

                for block in temp {
                    if !no_forbidden_decs_logic(block) {
                        return false;
                    }
                }
            }
            Command::Block(Block::ForRange(iter, _, _, vec)) => {
                if !no_forbidden_decs_check(iter) {
                    return false;
                }

                if !no_forbidden_decs_logic(vec) {
                    return false;
                }
            }
            _ => {}
        }
    }

    true
}

fn no_forbidden_decs_func(func: Function) -> bool {
    no_forbidden_decs_logic(func.content)
}

/// Check for overshadowing declarations (the ones that overwrite another)
fn no_shadowing(input: Program) -> bool {
    let mut definitions = Vec::new();
    for func in input.content.clone() {
        definitions.push(func.name);
    }

    for func in input.content {
        if !no_shadowing_func(func, definitions.clone()) {
            return false;
        }
    }

    true
}

fn no_shadowing_check(definitions: &mut Vec<String>, val: Variable) -> bool {
    let name = match val {
        Variable::Named(a) => a,
        _ => return true,
    };

    if definitions.iter().any(|i| *i == name) {
        println!("Variable redeclared: {}", name);
        return false;
    } else {
        definitions.push(name)
    }
    true
}

fn no_shadowing_logic(content: Vec<Command>, mut definitions: &mut Vec<String>) -> bool {
    for comm in content {
        match comm {
            Command::Binding(Binding::Declaration(name, _, _)) => {
                if !no_shadowing_check(&mut definitions, name) {
                    return false;
                }
            }
            Command::Binding(Binding::Assignment(name, _, _, _)) => {
                if !no_shadowing_check(&mut definitions, name) {
                    return false;
                }
            }
            Command::Binding(Binding::Tuple(vec)) => {
                for dec in vec {
                    match dec {
                        Command::Binding(Binding::Declaration(name, _, _)) => {
                            if !no_shadowing_check(&mut definitions, name) {
                                return false;
                            }
                        }
                        Command::Binding(Binding::Assignment(name, _, _, _)) => {
                            if !no_shadowing_check(&mut definitions, name) {
                                return false;
                            }
                        }
                        _ => {}
                    }
                }
            }
            Command::Block(Block::If(_, blocks, el)) => {
                let mut temp = blocks;
                temp.push(el);

                for block in temp {
                    let mut state = definitions.clone();
                    if !no_shadowing_logic(block, &mut state) {
                        return false;
                    }
                }
            }
            Command::Block(Block::ForRange(iter, _, _, vec)) => {
                let mut temp = definitions.clone();
                if !no_shadowing_check(&mut temp, iter) {
                    return false;
                }

                if !no_shadowing_logic(vec, &mut temp) {
                    return false;
                }
            }
            _ => {}
        }
    }

    true
}

fn no_shadowing_func(func: Function, mut definitions: Vec<String>) -> bool {
    no_shadowing_logic(func.content, &mut definitions)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_forbidden_decs_check1() {
        assert!(!no_forbidden_decs_check(Variable::Named(String::from(
            "return_value"
        ))));
    }

    #[test]
    fn no_forbidden_decs_check2() {
        assert!(no_forbidden_decs_check(Variable::Named(String::from("a"))));
    }

    #[test]
    fn no_shadowing_check1() {
        let mut defs = Vec::new();
        defs.push(String::from("a"));

        assert!(!no_shadowing_check(
            &mut defs,
            Variable::Named(String::from("a"))
        ));
    }

    #[test]
    fn no_shadowing_check2() {
        let mut defs = Vec::new();
        defs.push(String::from("a"));

        assert!(no_shadowing_check(
            &mut defs,
            Variable::Named(String::from("b"))
        ));
    }

    #[test]
    fn no_shadowing_check3() {
        let mut defs = Vec::new();
        defs.push(String::from("a"));

        assert!(no_shadowing_check(
            &mut defs,
            Variable::Named(String::from("b"))
        ));

        let mut temp = Vec::new();
        temp.push(String::from("a"));
        temp.push(String::from("b"));
        assert_eq!(defs, temp)
    }

    #[test]
    fn no_shadowing_logic1() {
        let mut defs = Vec::new();
        defs.push(String::from("a"));

        let mut coms = Vec::new();
        let mut for1 = Vec::new();
        for1.push(Command::Binding(Binding::Assignment(
            Variable::Named(String::from("a")),
            Type::Unknown,
            Value::Unit,
            false,
        )));

        coms.push(Command::Block(Block::ForRange(
            Variable::Named(String::from("i")),
            Value::Unit,
            Value::Unit,
            for1,
        )));

        assert!(!no_shadowing_logic(coms, &mut defs));
    }

    #[test]
    fn no_shadowing_logic2() {
        let mut defs = Vec::new();
        defs.push(String::from("a"));

        let mut coms = Vec::new();
        let mut for1 = Vec::new();
        for1.push(Command::Binding(Binding::Assignment(
            Variable::Named(String::from("b")),
            Type::Unknown,
            Value::Unit,
            false,
        )));

        coms.push(Command::Block(Block::ForRange(
            Variable::Named(String::from("i")),
            Value::Unit,
            Value::Unit,
            for1,
        )));

        assert!(no_shadowing_logic(coms, &mut defs));
    }

    #[test]
    fn no_shadowing_logic3() {
        let mut defs = Vec::new();
        defs.push(String::from("a"));

        let mut coms = Vec::new();
        let mut for1 = Vec::new();
        for1.push(Command::Binding(Binding::Assignment(
            Variable::Named(String::from("b")),
            Type::Unknown,
            Value::Unit,
            false,
        )));

        coms.push(Command::Block(Block::ForRange(
            Variable::Named(String::from("a")),
            Value::Unit,
            Value::Unit,
            for1,
        )));

        assert!(!no_shadowing_logic(coms, &mut defs));
    }

    #[test]
    fn no_shadowing_logic4() {
        let mut defs = Vec::new();
        defs.push(String::from("a"));

        let mut coms = Vec::new();

        let mut for2 = Vec::new();
        for2.push(Command::Binding(Binding::Assignment(
            Variable::Named(String::from("c")),
            Type::Unknown,
            Value::Unit,
            false,
        )));

        let mut for1 = Vec::new();

        for1.push(Command::Block(Block::ForRange(
            Variable::Named(String::from("d")),
            Value::Unit,
            Value::Unit,
            for2,
        )));

        coms.push(Command::Block(Block::ForRange(
            Variable::Named(String::from("c")),
            Value::Unit,
            Value::Unit,
            for1,
        )));

        assert!(!no_shadowing_logic(coms, &mut defs));
    }

    #[test]
    fn no_shadowing_logic5() {
        let mut defs = Vec::new();
        defs.push(String::from("a"));

        let mut coms = Vec::new();

        let mut if1 = Vec::new();
        if1.push(Command::Binding(Binding::Assignment(
            Variable::Named(String::from("a")),
            Type::Unknown,
            Value::Unit,
            false,
        )));

        let mut ifg = Vec::new();
        ifg.push(if1);

        coms.push(Command::Block(Block::If(Vec::new(), ifg, Vec::new())));

        assert!(!no_shadowing_logic(coms, &mut defs));
    }

    #[test]
    fn no_shadowing_logic6() {
        let mut defs = Vec::new();
        defs.push(String::from("a"));

        let mut coms = Vec::new();

        let mut if1 = Vec::new();
        if1.push(Command::Binding(Binding::Assignment(
            Variable::Named(String::from("b")),
            Type::Unknown,
            Value::Unit,
            false,
        )));

        let mut ifg = Vec::new();
        ifg.push(if1);

        coms.push(Command::Block(Block::If(Vec::new(), ifg, Vec::new())));

        assert!(no_shadowing_logic(coms, &mut defs));
    }

    #[test]
    fn no_shadowing_logic7() {
        let mut defs = Vec::new();
        defs.push(String::from("a"));

        let mut coms = Vec::new();

        let mut if1 = Vec::new();
        if1.push(Command::Binding(Binding::Assignment(
            Variable::Named(String::from("b")),
            Type::Unknown,
            Value::Unit,
            false,
        )));

        let mut el = Vec::new();
        el.push(Command::Binding(Binding::Assignment(
            Variable::Named(String::from("a")),
            Type::Unknown,
            Value::Unit,
            false,
        )));

        let mut ifg = Vec::new();
        ifg.push(if1);

        coms.push(Command::Block(Block::If(Vec::new(), ifg, el)));

        assert!(!no_shadowing_logic(coms, &mut defs));
    }
}