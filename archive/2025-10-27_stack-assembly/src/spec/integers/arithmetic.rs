use crate::{Application, Effect};

#[test]
fn addition() {
    let mut app = Application::compile_and_run("3 5 +");
    assert_eq!(app.operands(), &vec![8]);
    assert_eq!(app.effect(), None);
}

#[test]
fn addition_wraps() {
    let mut app = Application::compile_and_run("2147483647 1 +");
    assert_eq!(app.operands(), &vec![-2147483648]);
    assert_eq!(app.effect(), None);
}

#[test]
fn division() {
    let mut app = Application::compile_and_run("11 4 /");
    assert_eq!(app.operands(), &vec![2]);
    assert_eq!(app.effect(), None);
}

#[test]
fn division_triggers_division_by_zero() {
    let mut app = Application::compile_and_run("1 0 /");
    assert_eq!(app.operands(), &vec![]);
    assert_eq!(app.effect(), Some(&Effect::DivisionByZero));
}

#[test]
fn division_triggers_integer_overflow() {
    let mut app = Application::compile_and_run("-2147483648 -1 /");
    assert_eq!(app.operands(), &vec![]);
    assert_eq!(app.effect(), Some(&Effect::IntegerOverflow));
}

#[test]
fn multiplication() {
    let mut app = Application::compile_and_run("3 5 *");
    assert_eq!(app.operands(), &vec![15]);
    assert_eq!(app.effect(), None);
}

#[test]
fn multiplication_wraps() {
    let mut app = Application::compile_and_run("1073741824 2 *");
    assert_eq!(app.operands(), &vec![-2147483648]);
    assert_eq!(app.effect(), None);
}

#[test]
fn remainder() {
    let mut app = Application::compile_and_run("11 4 %");
    assert_eq!(app.operands(), &vec![3]);
    assert_eq!(app.effect(), None);
}

#[test]
fn remainder_triggers_division_by_zero() {
    let mut app = Application::compile_and_run("1 0 %");
    assert_eq!(app.operands(), &vec![]);
    assert_eq!(app.effect(), Some(&Effect::DivisionByZero));
}

#[test]
fn remainder_triggers_integer_overflow() {
    let mut app = Application::compile_and_run("-2147483648 -1 %");
    assert_eq!(app.operands(), &vec![]);
    assert_eq!(app.effect(), Some(&Effect::IntegerOverflow));
}

#[test]
fn subtraction() {
    let mut app = Application::compile_and_run("3 5 -");
    assert_eq!(app.operands(), &vec![-2]);
    assert_eq!(app.effect(), None);
}

#[test]
fn subtraction_wraps() {
    let mut app = Application::compile_and_run("-2147483648 1 -");
    assert_eq!(app.operands(), &vec![2147483647]);
    assert_eq!(app.effect(), None);
}
