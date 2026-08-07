mod continuations;

pub mod infra;

use arbtest::arbtest;

use crate::{
    CompileError, Script,
    tests::infra::{TestHost, TestHostFn},
};

#[test]
fn unresolved_name() {
    // A name in the source that can not be resolved to a function or binding
    // must result in a compile error.

    let host = TestHost::new::<TestHostFn>();

    let result = Script::compile("unresolved", &host);
    assert_eq!(result, Err(CompileError::UnresolvedName));
}

#[test]
fn random_source_code() {
    // Random source code should never trigger a crash.

    arbtest(|u| {
        let source = u.arbitrary::<String>()?;

        let mut host = TestHost::new::<TestHostFn>();

        if let Ok(script) = Script::compile(&source, &host) {
            script.run(&mut host);
        }

        Ok(())
    });
}

#[test]
fn syntactically_correct_source_code() {
    // Syntactically correct source code should never trigger a crash.

    arbtest(|u| {
        let mut source = String::new();

        for _ in 0..u.arbitrary_len::<TestHostFn>()? {
            if !source.is_empty() {
                source.push(' ');
            }

            let fragment = match u.arbitrary::<TestHostFn>()? {
                TestHostFn::Exit => "exit",
                TestHostFn::Signal => "signal",
            };
            source.push_str(fragment);
        }

        let mut host = TestHost::new::<TestHostFn>();

        if let Ok(script) = Script::compile(&source, &host) {
            script.run(&mut host);
        }

        Ok(())
    });
}
