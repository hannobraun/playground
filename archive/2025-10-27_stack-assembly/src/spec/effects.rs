use crate::{Application, Effect};

#[test]
fn trigger_effect_on_abort() {
    let mut app = Application::compile_and_run("3 abort 5");
    assert_eq!(app.operands(), &vec![3]);
    assert_eq!(app.effect(), Some(&Effect::Abort));
}

#[test]
fn trigger_effect_on_unknown_operator() {
    let mut app = Application::compile_and_run("3 unknown_operator 5");
    assert_eq!(app.operands(), &vec![3]);
    assert_eq!(app.effect(), Some(&Effect::UnknownOperator));
}

#[test]
fn trigger_effect_on_yield() {
    let mut app = Application::compile_and_run("3 yield 5");

    assert_eq!(app.operands(), &vec![3]);
    assert_eq!(app.effect(), Some(&Effect::Yield));

    app.continue_();

    assert_eq!(app.operands(), &vec![3, 5]);
    assert_eq!(app.effect(), None);
}
