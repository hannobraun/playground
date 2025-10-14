use crate::Program;

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
fn leading_zeros() {
    let mut program = Program::compile_and_run("1 leading_zeros");
    assert_eq!(program.operands(), &vec![31]);
    assert_eq!(program.effect(), None);
}
