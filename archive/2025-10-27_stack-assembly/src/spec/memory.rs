use crate::Program;

#[test]
fn memory_should_be_zero_initialized() {
    let mut program = Program::compile("");

    assert_ne!(program.memory().len(), 0);

    for word in program.memory() {
        assert_eq!(*word, 0);
    }
}
