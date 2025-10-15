use crate::Application;

#[test]
fn literals() {
    let mut app = Application::compile("3 5");
    assert_eq!(app.operands(), &vec![]);

    app.continue_();
    assert_eq!(app.operands(), &vec![3, 5]);
}
