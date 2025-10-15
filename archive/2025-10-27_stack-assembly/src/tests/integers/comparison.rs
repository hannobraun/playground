use crate::Application;

#[test]
fn equal_is_false() {
    let mut app = Application::compile_and_run("3 5 =");
    assert_eq!(app.operands(), &vec![0]);
    assert_eq!(app.effect(), None);
}

#[test]
fn equal_is_true() {
    let mut app = Application::compile_and_run("3 3 =");
    assert_eq!(app.operands(), &vec![1]);
    assert_eq!(app.effect(), None);
}

#[test]
fn larger_is_equal() {
    let mut app = Application::compile_and_run("3 3 >");
    assert_eq!(app.operands(), &vec![0]);
    assert_eq!(app.effect(), None);
}

#[test]
fn larger_is_larger() {
    let mut app = Application::compile_and_run("5 3 >");
    assert_eq!(app.operands(), &vec![1]);
    assert_eq!(app.effect(), None);
}

#[test]
fn larger_is_smaller() {
    let mut app = Application::compile_and_run("3 5 >");
    assert_eq!(app.operands(), &vec![0]);
    assert_eq!(app.effect(), None);
}

#[test]
fn larger_or_equal_equal() {
    let mut app = Application::compile_and_run("3 3 >=");
    assert_eq!(app.operands(), &vec![1]);
    assert_eq!(app.effect(), None);
}

#[test]
fn larger_or_equal_larger() {
    let mut app = Application::compile_and_run("5 3 >=");
    assert_eq!(app.operands(), &vec![1]);
    assert_eq!(app.effect(), None);
}

#[test]
fn larger_or_equal_smaller() {
    let mut app = Application::compile_and_run("3 5 >=");
    assert_eq!(app.operands(), &vec![0]);
    assert_eq!(app.effect(), None);
}

#[test]
fn smaller_is_equal() {
    let mut app = Application::compile_and_run("3 3 <");
    assert_eq!(app.operands(), &vec![0]);
    assert_eq!(app.effect(), None);
}

#[test]
fn smaller_is_larger() {
    let mut app = Application::compile_and_run("5 3 <");
    assert_eq!(app.operands(), &vec![0]);
    assert_eq!(app.effect(), None);
}

#[test]
fn smaller_is_smaller() {
    let mut app = Application::compile_and_run("3 5 <");
    assert_eq!(app.operands(), &vec![1]);
    assert_eq!(app.effect(), None);
}

#[test]
fn smaller_or_equal_is_equal() {
    let mut app = Application::compile_and_run("3 3 <=");
    assert_eq!(app.operands(), &vec![1]);
    assert_eq!(app.effect(), None);
}

#[test]
fn smaller_or_equal_is_larger() {
    let mut app = Application::compile_and_run("5 3 <=");
    assert_eq!(app.operands(), &vec![0]);
    assert_eq!(app.effect(), None);
}

#[test]
fn smaller_or_equal_is_smaller() {
    let mut app = Application::compile_and_run("3 5 <=");
    assert_eq!(app.operands(), &vec![1]);
    assert_eq!(app.effect(), None);
}
