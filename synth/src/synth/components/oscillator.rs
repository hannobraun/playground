use crate::synth::{
    clock::Clock,
    signal::{IsSignal, Signal},
};

pub struct Oscillator {
    pub frequency: Signal,
    pub wave: fn(f32) -> f32,
}

impl IsSignal for Oscillator {
    fn value(&self, clock: &Clock) -> f32 {
        let t = clock.t(self.frequency.value(clock));
        (self.wave)(t)
    }
}