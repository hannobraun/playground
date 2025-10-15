use crate::Application;

#[test]
fn literals() {
    let mut program = Application::compile("3 5");
    assert_eq!(program.operands(), &vec![]);

    program.continue_();
    assert_eq!(program.operands(), &vec![3, 5]);
}
