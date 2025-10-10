use crate::{Effect, Program};

#[test]
fn memory_should_be_zero_initialized() {
    let mut program = Program::compile("");

    assert_ne!(program.memory().len(), 0);

    for word in program.memory() {
        assert_eq!(*word, 0);
    }
}

#[test]
fn read_from_memory() {
    let mut program = Program::compile("0 read 1 read");

    program.memory()[0] = 3;
    program.memory()[1] = 5;

    program.continue_();

    assert_eq!(program.operands(), &vec![3, 5]);
    assert_eq!(program.effect(), None);
}

#[test]
fn trigger_effect_on_read_from_out_of_bounds_address() {
    let mut program = Program::compile("1024 read");

    assert!(program.memory().len() >= 1024);
    program.continue_();

    assert_eq!(program.operands(), &vec![]);
    assert_eq!(program.effect(), Some(&Effect::OutOfBoundsAddress));
}

#[test]
fn trigger_effect_on_read_from_invalid_address() {
    let mut program = Program::compile_and_run("-1 read");
    assert_eq!(program.operands(), &vec![]);
    assert_eq!(program.effect(), Some(&Effect::InvalidAddress));
}

#[test]
fn write_to_memory() {
    let mut program = Program::compile_and_run("3 0 write 5 1 write");
    assert_eq!(program.operands(), &vec![]);
    assert!(matches!(program.memory(), &mut [3, 5, ..]));
    assert_eq!(program.effect(), None);
}

#[test]
fn trigger_effect_on_write_to_out_of_bounds_address() {
    let mut program = Program::compile("3 1024 write");

    assert!(program.memory().len() >= 1024);
    program.continue_();

    assert_eq!(program.operands(), &vec![]);
    assert_eq!(program.effect(), Some(&Effect::OutOfBoundsAddress));
}

#[test]
fn trigger_effect_on_write_to_invalid_address() {
    let mut program = Program::compile_and_run("3 -1 write");
    assert_eq!(program.operands(), &vec![]);
    assert_eq!(program.effect(), Some(&Effect::InvalidAddress));
}
