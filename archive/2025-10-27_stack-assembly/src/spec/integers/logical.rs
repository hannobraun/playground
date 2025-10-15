use crate::{Application, Effect};

#[test]
fn not_from_false_to_true() {
    let mut app = Application::compile_and_run("0 not");
    assert_eq!(app.operands(), &vec![1]);
    assert_eq!(app.effect(), None);
}

#[test]
fn not_from_true_to_false() {
    let mut app = Application::compile_and_run("1 not");
    assert_eq!(app.operands(), &vec![0]);
    assert_eq!(app.effect(), None);
}

#[test]
fn not_trigger_invalid_operator() {
    let mut app = Application::compile_and_run("2 not");
    assert_eq!(app.operands(), &vec![]);
    assert_eq!(app.effect(), Some(&Effect::InvalidOperand));
}
