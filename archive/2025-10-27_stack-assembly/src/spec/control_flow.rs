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

#[test]
fn evaluating_reference_pushes_address_to_stack() {
    let program = Program::compile_and_run("@f f:");
    assert_eq!(program.operands().len(), 1);
    assert_eq!(program.effect(), None);
}

#[test]
fn apply_functions_unconditionally() {
    let program = Program::compile_and_run(
        "
        @f apply
        @g apply

        f:
            3
        g:
            5
        ",
    );
    assert_eq!(program.operands(), &vec![3, 5]);
    assert_eq!(program.effect(), None);
}

#[test]
fn apply_should_trigger_effect_on_invalid_address() {
    let program = Program::compile_and_run("-1 apply 3");
    assert_eq!(program.operands(), &vec![]);
    assert_eq!(program.effect(), Some(&Effect::InvalidInstructionAddress));
}
