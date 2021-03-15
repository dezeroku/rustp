use crate::ast::*;

/// Just error out if some error occurs, no output
pub fn validate(input: Program) -> bool {
    no_shadowing(input.clone())
}

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
