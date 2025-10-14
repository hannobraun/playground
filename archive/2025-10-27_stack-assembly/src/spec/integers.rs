use crate::{Effect, Program};

#[test]
fn evaluate_integers() {
    let mut program = Program::compile("3 5");
    assert_eq!(program.operands(), &vec![]);

    program.continue_();
    assert_eq!(program.operands(), &vec![3, 5]);
}

#[test]
fn addition() {
    let mut program = Program::compile_and_run("3 5 +");
    assert_eq!(program.operands(), &vec![8]);
    assert_eq!(program.effect(), None);
}

#[test]
fn addition_triggers_integer_overflow() {
    let mut program = Program::compile_and_run("2147483647 1 +");
    assert_eq!(program.operands(), &vec![]);
    assert_eq!(program.effect(), Some(&Effect::IntegerOverflow));
}

#[test]
fn division() {
    let mut program = Program::compile_and_run("11 4 /");
    assert_eq!(program.operands(), &vec![2]);
    assert_eq!(program.effect(), None);
}

#[test]
fn division_triggers_division_by_zero() {
    let mut program = Program::compile_and_run("1 0 /");
    assert_eq!(program.operands(), &vec![]);
    assert_eq!(program.effect(), Some(&Effect::DivisionByZero));
}

#[test]
fn division_triggers_integer_overflow() {
    let mut program = Program::compile_and_run("-2147483648 -1 /");
    assert_eq!(program.operands(), &vec![]);
    assert_eq!(program.effect(), Some(&Effect::IntegerOverflow));
}

#[test]
fn multiplication() {
    let mut program = Program::compile_and_run("3 5 *");
    assert_eq!(program.operands(), &vec![15]);
    assert_eq!(program.effect(), None);
}

#[test]
fn multiplication_triggers_integer_overflow() {
    let mut program = Program::compile_and_run("1073741824 2 *");
    assert_eq!(program.operands(), &vec![]);
    assert_eq!(program.effect(), Some(&Effect::IntegerOverflow));
}

#[test]
fn subtraction() {
    let mut program = Program::compile_and_run("3 5 -");
    assert_eq!(program.operands(), &vec![-2]);
    assert_eq!(program.effect(), None);
}

#[test]
fn subtraction_triggers_integer_overflow() {
    let mut program = Program::compile_and_run("-2147483648 1 -");
    assert_eq!(program.operands(), &vec![]);
    assert_eq!(program.effect(), Some(&Effect::IntegerOverflow));
}
