use crate::compiler::ir::{Signature, Type};

#[derive(Debug)]
pub struct Stack {
    pub inputs: Vec<Type>,
    pub outputs: Vec<Type>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    pub fn push(&mut self, ty: Type) {
        self.outputs.push(ty);
    }

    pub fn pop(&mut self, expected_type: Type) -> Option<Type> {
        if let Some(type_on_stack) = self.outputs.pop() {
            // We're not checking, if the type on the stack matches the expected
            // type. For the most part, the language is untyped, so values are
            // treated differently, depending on the operation that consumes
            // them.
            //
            // There is a nascent static type system that supports some
            // functions of the language (like generating WebAssembly functions,
            // or figuring out the output of `apply` operations), but overall,
            // it's not complete enough to make a check here sensible.
            Some(type_on_stack)
        } else {
            self.inputs.push(expected_type);
            None
        }
    }

    pub fn to_signature(&self) -> Signature {
        Signature {
            inputs: self.inputs.clone(),
            outputs: self.outputs.clone(),
        }
    }
}
