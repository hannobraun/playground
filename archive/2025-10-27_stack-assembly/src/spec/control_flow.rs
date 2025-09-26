use crate::{Effect, Program};

#[test]
fn functions_have_no_effect_if_not_applied() {
    let program = Program::compile_and_run("3 f: 5");
    assert_eq!(program.operands(), &vec![3]);
    assert_eq!(program.effect(), None);
}

#[test]
fn invalid_reference_triggers_effect() {
    let program = Program::compile_and_run("@g 3");
    assert_eq!(program.operands(), &vec![]);
    assert_eq!(program.effect(), Some(&Effect::InvalidReference));
}
