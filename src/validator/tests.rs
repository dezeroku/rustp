#[cfg(test)]
mod test {
    use crate::validator::*;

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
    fn no_forbidden_decs_check3() {
        assert!(!no_forbidden_decs_check(Variable::Named(String::from(
            "a'old"
        ))));
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
            Type::I32,
            Value::Unit,
            false,
        )));

        coms.push(Command::Block(Block::ForRange(
            Variable::Named(String::from("i")),
            Value::Unit,
            Value::Unit,
            for1,
            Bool::True,
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
            Type::I32,
            Value::Unit,
            false,
        )));

        coms.push(Command::Block(Block::ForRange(
            Variable::Named(String::from("i")),
            Value::Unit,
            Value::Unit,
            for1,
            Bool::True,
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
            Type::I32,
            Value::Unit,
            false,
        )));

        coms.push(Command::Block(Block::ForRange(
            Variable::Named(String::from("a")),
            Value::Unit,
            Value::Unit,
            for1,
            Bool::True,
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
            Type::I32,
            Value::Unit,
            false,
        )));

        let mut for1 = Vec::new();

        for1.push(Command::Block(Block::ForRange(
            Variable::Named(String::from("d")),
            Value::Unit,
            Value::Unit,
            for2,
            Bool::True,
        )));

        coms.push(Command::Block(Block::ForRange(
            Variable::Named(String::from("c")),
            Value::Unit,
            Value::Unit,
            for1,
            Bool::True,
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
            Type::I32,
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
            Type::I32,
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
            Type::I32,
            Value::Unit,
            false,
        )));

        let mut el = Vec::new();
        el.push(Command::Binding(Binding::Assignment(
            Variable::Named(String::from("a")),
            Type::I32,
            Value::Unit,
            false,
        )));

        let mut ifg = Vec::new();
        ifg.push(if1);

        coms.push(Command::Block(Block::If(Vec::new(), ifg, el)));

        assert!(!no_shadowing_logic(coms, &mut defs));
    }
}
