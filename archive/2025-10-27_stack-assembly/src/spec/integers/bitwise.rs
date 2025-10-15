use crate::{Effect, Program};

// bit logic

#[test]
fn and() {
    let mut program = Program::compile_and_run("10 12 and");
    assert_eq!(program.operands(), &vec![8]);
    assert_eq!(program.effect(), None);
}

#[test]
fn or() {
    let mut program = Program::compile_and_run("10 12 or");
    assert_eq!(program.operands(), &vec![14]);
    assert_eq!(program.effect(), None);
}

#[test]
fn xor() {
    let mut program = Program::compile_and_run("10 12 xor");
    assert_eq!(program.operands(), &vec![6]);
    assert_eq!(program.effect(), None);
}

// bit counting

#[test]
fn count_ones() {
    let mut program = Program::compile_and_run("10 count_ones");
    assert_eq!(program.operands(), &vec![2]);
    assert_eq!(program.effect(), None);
}

#[test]
fn leading_zeros() {
    let mut program = Program::compile_and_run("1 leading_zeros");
    assert_eq!(program.operands(), &vec![31]);
    assert_eq!(program.effect(), None);
}

#[test]
fn trailing_zeros() {
    let mut program = Program::compile_and_run("-2147483648 trailing_zeros");
    assert_eq!(program.operands(), &vec![31]);
    assert_eq!(program.effect(), None);
}

// rotating and shifting

#[test]
fn shift_left() {
    let mut program = Program::compile_and_run("5 2 shift_left");
    assert_eq!(program.operands(), &vec![20]);
    assert_eq!(program.effect(), None);
}

#[test]
fn shift_left_overflow() {
    let mut program = Program::compile_and_run("-1610612736 2 shift_left");
    assert_eq!(program.operands(), &vec![-2147483648]);
    assert_eq!(program.effect(), None);
}

#[test]
fn shift_left_trigger_invalid_operand() {
    let mut program = Program::compile_and_run("0 -2 shift_left");
    assert_eq!(program.operands(), &vec![]);
    assert_eq!(program.effect(), Some(&Effect::InvalidOperand));
}
