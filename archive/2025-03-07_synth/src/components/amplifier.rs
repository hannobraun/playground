use crate::{range, signal::Signal};

pub trait Amplify {
    fn amplify(self, factor: impl Into<Signal>) -> Signal;
}

impl Amplify for Signal {
    fn amplify(mut self, gain: impl Into<Signal>) -> Signal {
        let mut gain = gain.into();

        Signal::from_fn(move || {
            self.next_value() * gain.next_value().decode_to(range::AMPLIFIER)
        })
    }
}
