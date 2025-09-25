use crate::Program;

#[test]
fn empty_program_exits_with_empty_stack() {
    let program = Program::compile_and_run("");
    assert_eq!(program.stack(), &vec![]);
}

#[test]
fn stack_stays_empty_if_program_never_starts() {
    let program = Program::compile("3");
    assert_eq!(program.stack(), &vec![]);
}
