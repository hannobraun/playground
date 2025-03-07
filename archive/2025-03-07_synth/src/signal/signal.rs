use crate::range::Range;

use super::{
    source::{Fn, SignalSource},
    Value,
};

/// A signal is a series of values over time
///
/// Can represent audio, or some kind of control signal. It is weakly typed, in
/// the sense that the values of `Signal` are always of type [`Value`], which
/// means that whatever they represent, the value is encoded into the range (-1,
/// 1).
///
/// This design decision was made for two reasons:
///
/// 1. To not over-complicate the API with generics.
/// 2. To keep in the spirit of modular synthesizers.
///
/// The trade-off this design makes is to achieve simplicity and flexibility, at
/// the cost of being error-prone. Whether this is the right trade-off remains
/// to be seen.
///
/// While this software is certainly *inspired by* modular hardware synths, it
/// was not created to copy them. It is meant to re-imagine the concept of
/// modular synthesis for a code-based, functional world. And it might yet turn
/// out, that stronger typing makes more sense there.
pub struct Signal {
    source: Box<dyn SignalSource<Value> + Send>,
}

impl Signal {
    pub fn constant(value: Value) -> Self {
        Self::from_fn(move || value)
    }

    pub fn from_fn(f: impl FnMut() -> Value + Send + 'static) -> Self {
        Self {
            source: Box::new(Fn(f)),
        }
    }

    pub fn next_value(&mut self) -> Value {
        self.source.next_value()
    }
}

pub trait IntoSignal {
    fn into_signal(self, range: Range) -> Signal;
}

impl IntoSignal for Signal {
    fn into_signal(self, range: Range) -> Signal {
        // If a signal is already a signal, then the range is ignored. This
        // might be error-prone. See documentation of `Signal` for a discussion
        // of the trade-offs made in `Signal`'s design.
        //
        // It's possible that in the future, `Signal` will store the range it's
        // encoded in. In that case, this parameter could be used for a check,
        // or maybe a conversion.
        let _ = range;

        self
    }
}

impl IntoSignal for f32 {
    fn into_signal(self, range: Range) -> Signal {
        let value = Value::encode_from(self, range);
        Signal::constant(value)
    }
}
