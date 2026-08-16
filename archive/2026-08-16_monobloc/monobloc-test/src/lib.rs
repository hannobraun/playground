//! # End-to-end test for the main `monobloc` crate
//!
//! ## Design Note
//!
//! It is easy to add a type which is supposed to be part of the public API, but
//! forget to make this type available in a location where users from outside of
//! the crate can actually access it.
//!
//! Having the end-to-end crates in a dedicated crate ensures that such a
//! mistake gets caught immediately.

#![cfg(test)]

mod continuations;
mod values;

pub mod infra;

use arbtest::arbtest;
use monobloc::{CompileError, Script};

use self::{
    continuations::ContinuationHostFn,
    infra::{TestHost, TestHostFn},
};

// The following tests are either very general, or they are very insular and
// don't fit into any of the larger groups of tests, for which there are modules
// above. As the language grows and more tests are being added, some of them may
// be grouped with new tests, into a new module.

#[test]
fn unresolved_name() {
    // A name in the source that can not be resolved to a function or binding
    // must result in a compile error.

    let host = TestHost::new::<ContinuationHostFn>();

    let result = Script::compile("unresolved", &host);
    assert_eq!(result, Err(CompileError::UnresolvedName));
}

#[test]
fn random_source_code() {
    // Random source code should never trigger a crash.

    arbtest(|u| {
        let source = u.arbitrary::<String>()?;

        let mut host = TestHost::new::<ContinuationHostFn>();

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

        for _ in 0..u.arbitrary_len::<ContinuationHostFn>()? {
            if !source.is_empty() {
                source.push(' ');
            }

            let fragment = u.arbitrary::<ContinuationHostFn>()?.attrs().name;
            source.push_str(fragment);
        }

        let mut host = TestHost::new::<ContinuationHostFn>();

        if let Ok(script) = Script::compile(&source, &host) {
            script.run(&mut host);
        }

        Ok(())
    });
}
