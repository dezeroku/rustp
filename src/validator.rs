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

fn no_shadowing_func(func: Function, mut definitions: Vec<String>) -> bool {
    for comm in func.content {
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
            // TODO: handle block, should we even allow declarations inside?
            _ => {}
        }
    }

    true
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
}
