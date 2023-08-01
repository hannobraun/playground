use crate::synth::{
    clock::Clock,
    signal::{HasOutput, Signal},
    wave::Wave,
};

#[derive(Default)]
pub struct Oscillator {
    pub frequency: Signal,
    pub wave: Wave,
}

impl HasOutput for Oscillator {
    fn value(&self, clock: &Clock) -> Option<f32> {
        let frequency = self.frequency.value(clock)?;
        let t = clock.t(frequency);
        Some(self.wave.value(t))
    }
}
