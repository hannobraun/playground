use crate::{Application, Effect};

#[test]
fn memory_should_be_zero_initialized() {
    let mut app = Application::compile("");

    assert_ne!(app.memory().len(), 0);

    for word in app.memory() {
        assert_eq!(*word, 0);
    }
}

#[test]
fn read_from_memory() {
    let mut app = Application::compile("0 read 1 read");

    app.memory()[0] = 3;
    app.memory()[1] = 5;

    app.continue_();

    assert_eq!(app.operands(), &vec![3, 5]);
    assert_eq!(app.effect(), None);
}

#[test]
fn trigger_effect_on_read_from_out_of_bounds_address() {
    let mut app = Application::compile("1024 read");

    assert!(app.memory().len() >= 1024);
    app.continue_();

    assert_eq!(app.operands(), &vec![]);
    assert_eq!(app.effect(), Some(&Effect::OutOfBoundsAddress));
}

#[test]
fn trigger_effect_on_read_from_invalid_address() {
    let mut app = Application::compile_and_run("-1 read");
    assert_eq!(app.operands(), &vec![]);
    assert_eq!(app.effect(), Some(&Effect::InvalidOperand));
}

#[test]
fn write_to_memory() {
    let mut app = Application::compile_and_run("3 0 write 5 1 write");
    assert_eq!(app.operands(), &vec![]);
    assert!(matches!(app.memory(), &mut [3, 5, ..]));
    assert_eq!(app.effect(), None);
}

#[test]
fn trigger_effect_on_write_to_out_of_bounds_address() {
    let mut app = Application::compile("3 1024 write");

    assert!(app.memory().len() >= 1024);
    app.continue_();

    assert_eq!(app.operands(), &vec![]);
    assert_eq!(app.effect(), Some(&Effect::OutOfBoundsAddress));
}

#[test]
fn trigger_effect_on_write_to_invalid_address() {
    let mut app = Application::compile_and_run("3 -1 write");
    assert_eq!(app.operands(), &vec![]);
    assert_eq!(app.effect(), Some(&Effect::InvalidOperand));
}
