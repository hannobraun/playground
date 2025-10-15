use crate::{Application, Effect};

// bit logic

#[test]
fn and() {
    let mut app = Application::compile_and_run("10 12 and");
    assert_eq!(app.operands(), &vec![8]);
    assert_eq!(app.effect(), None);
}

#[test]
fn or() {
    let mut app = Application::compile_and_run("10 12 or");
    assert_eq!(app.operands(), &vec![14]);
    assert_eq!(app.effect(), None);
}

#[test]
fn xor() {
    let mut app = Application::compile_and_run("10 12 xor");
    assert_eq!(app.operands(), &vec![6]);
    assert_eq!(app.effect(), None);
}

// bit counting

#[test]
fn count_ones() {
    let mut app = Application::compile_and_run("10 count_ones");
    assert_eq!(app.operands(), &vec![2]);
    assert_eq!(app.effect(), None);
}

#[test]
fn leading_zeros() {
    let mut app = Application::compile_and_run("1 leading_zeros");
    assert_eq!(app.operands(), &vec![31]);
    assert_eq!(app.effect(), None);
}

#[test]
fn trailing_zeros() {
    let mut app = Application::compile_and_run("-2147483648 trailing_zeros");
    assert_eq!(app.operands(), &vec![31]);
    assert_eq!(app.effect(), None);
}

// rotating and shifting

#[test]
fn rotate_left() {
    let mut app = Application::compile_and_run("-2147483646 2 rotate_left");
    assert_eq!(app.operands(), &vec![10]);
    assert_eq!(app.effect(), None);
}

#[test]
fn rotate_left_trigger_invalid_operand() {
    let mut app = Application::compile_and_run("0 -2 rotate_left");
    assert_eq!(app.operands(), &vec![]);
    assert_eq!(app.effect(), Some(&Effect::InvalidOperand));
}

#[test]
fn rotate_right() {
    let mut app = Application::compile_and_run("10 2 rotate_right");
    assert_eq!(app.operands(), &vec![-2147483646]);
    assert_eq!(app.effect(), None);
}

#[test]
fn rotate_right_trigger_invalid_operand() {
    let mut app = Application::compile_and_run("0 -2 rotate_right");
    assert_eq!(app.operands(), &vec![]);
    assert_eq!(app.effect(), Some(&Effect::InvalidOperand));
}

#[test]
fn shift_left() {
    let mut app = Application::compile_and_run("5 2 shift_left");
    assert_eq!(app.operands(), &vec![20]);
    assert_eq!(app.effect(), None);
}

#[test]
fn shift_left_overflow() {
    let mut app = Application::compile_and_run("-1610612736 2 shift_left");
    assert_eq!(app.operands(), &vec![-2147483648]);
    assert_eq!(app.effect(), None);
}

#[test]
fn shift_left_trigger_invalid_operand() {
    let mut app = Application::compile_and_run("0 -2 shift_left");
    assert_eq!(app.operands(), &vec![]);
    assert_eq!(app.effect(), Some(&Effect::InvalidOperand));
}

#[test]
fn shift_right() {
    let mut app = Application::compile_and_run("20 2 shift_right");
    assert_eq!(app.operands(), &vec![5]);
    assert_eq!(app.effect(), None);
}

#[test]
fn shift_right_overflow() {
    let mut app = Application::compile_and_run("10 2 shift_right");
    assert_eq!(app.operands(), &vec![2]);
    assert_eq!(app.effect(), None);
}

#[test]
fn shift_right_trigger_invalid_operand() {
    let mut app = Application::compile_and_run("0 -2 shift_right");
    assert_eq!(app.operands(), &vec![]);
    assert_eq!(app.effect(), Some(&Effect::InvalidOperand));
}
