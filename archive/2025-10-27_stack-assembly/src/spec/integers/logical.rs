use crate::{Application, Effect};

#[test]
fn not_from_false_to_true() {
    let mut program = Application::compile_and_run("0 not");
    assert_eq!(program.operands(), &vec![1]);
    assert_eq!(program.effect(), None);
}

#[test]
fn not_from_true_to_false() {
    let mut program = Application::compile_and_run("1 not");
    assert_eq!(program.operands(), &vec![0]);
    assert_eq!(program.effect(), None);
}

#[test]
fn not_trigger_invalid_operator() {
    let mut program = Application::compile_and_run("2 not");
    assert_eq!(program.operands(), &vec![]);
    assert_eq!(program.effect(), Some(&Effect::InvalidOperand));
}
