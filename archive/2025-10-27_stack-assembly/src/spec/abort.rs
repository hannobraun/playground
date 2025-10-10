use crate::{Effect, Program};

#[test]
fn trigger_effect_on_abort() {
    let mut program = Program::compile_and_run("3 abort 5");
    assert_eq!(program.operands(), &vec![3]);
    assert_eq!(program.effect(), Some(&Effect::Abort));
}
