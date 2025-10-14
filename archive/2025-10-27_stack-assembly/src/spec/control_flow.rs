use crate::{Effect, Program};

#[test]
fn functions_have_no_effect_if_not_called() {
    let mut program = Program::compile_and_run("3 f: 5");
    assert_eq!(program.operands(), &vec![3]);
    assert_eq!(program.effect(), None);
}

#[test]
fn invalid_reference_triggers_effect() {
    let mut program = Program::compile_and_run("@g 3");
    assert_eq!(program.operands(), &vec![]);
    assert_eq!(program.effect(), Some(&Effect::InvalidReference));
}

#[test]
fn evaluating_reference_pushes_address_to_stack() {
    let mut program = Program::compile_and_run("@f f:");
    assert_eq!(program.operands().len(), 1);
    assert_eq!(program.effect(), None);
}

#[test]
fn call_functions_unconditionally() {
    let mut program = Program::compile_and_run(
        "
        @f call
        @g call

        f:
            3
        g:
            5
        ",
    );
    assert_eq!(program.operands(), &vec![3, 5]);
    assert_eq!(program.effect(), None);
}

#[test]
fn call_should_trigger_effect_on_invalid_address() {
    let mut program = Program::compile_and_run("-1 call 3");
    assert_eq!(program.operands(), &vec![]);
    assert_eq!(program.effect(), Some(&Effect::InvalidOperand));
}

#[test]
fn call_function_because_of_condition() {
    let mut program = Program::compile_and_run("1 @f call_if f: 3");
    assert_eq!(program.operands(), &vec![3]);
    assert_eq!(program.effect(), None);
}

#[test]
fn do_not_call_function_because_of_condition() {
    let mut program = Program::compile_and_run("0 @f call_if f: 3");
    assert_eq!(program.operands(), &vec![]);
    assert_eq!(program.effect(), None);
}

#[test]
fn tail_call_optimization_should_work_for_unconditional_calls() {
    let mut program = Program::compile_and_run("@f call f: yield @f call");

    assert_eq!(program.call_stack().len(), 0);
    assert_eq!(program.effect(), Some(&Effect::Yield));

    program.continue_();

    assert_eq!(program.call_stack().len(), 0);
    assert_eq!(program.effect(), Some(&Effect::Yield));
}

#[test]
fn tail_call_optimization_should_work_for_conditional_calls() {
    let mut program = Program::compile_and_run("@f call f: yield 1 @f call_if");

    assert_eq!(program.call_stack().len(), 0);
    assert_eq!(program.effect(), Some(&Effect::Yield));

    program.continue_();

    assert_eq!(program.call_stack().len(), 0);
    assert_eq!(program.effect(), Some(&Effect::Yield));
}
