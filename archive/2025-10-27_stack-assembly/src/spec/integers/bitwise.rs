use crate::Program;

#[test]
fn and() {
    let mut program = Program::compile_and_run("10 12 and");
    assert_eq!(program.operands(), &vec![8]);
    assert_eq!(program.effect(), None);
}
