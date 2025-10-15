use crate::Application;

#[test]
fn empty_application_exits_with_empty_stack() {
    let mut app = Application::compile_and_run("");
    assert_eq!(app.operands(), &vec![]);
}

#[test]
fn empty_application_should_not_trigger_effect() {
    let app = Application::compile_and_run("");
    assert_eq!(app.effect(), None);
}
