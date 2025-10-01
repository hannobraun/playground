use crate::{Effect, Program};

#[test]
fn yield_triggers_effect() {
    let program = Program::compile_and_run("3 yield 5");
    assert_eq!(program.operands(), &vec![3]);
    assert_eq!(program.effect(), Some(&Effect::Yield));
}
