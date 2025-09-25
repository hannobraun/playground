use crate::Program;

#[test]
fn empty_program_exits_with_empty_stack() {
    let mut program = Program::compile("");
    program.run();

    assert_eq!(program.stack(), &vec![]);
}
