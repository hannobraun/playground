use crate::{clock::Clock, signal::Signal};

pub struct Osc {
    pub clock: Clock,
    pub frequency: Box<dyn Signal>,
    pub amplitude: f32,
    pub offset: f32,
    pub wave: fn(f32) -> f32,
}

impl Signal for Osc {
    fn next_value(&mut self) -> f32 {
        self.clock.advance();

        // I don't believe this works for timers < 1 Hz. This requires some
        // investigation.
        let t = ((self.clock.time % self.clock.sample_rate) as f64
            / self.clock.sample_rate as f64
            * self.frequency.next_value() as f64
            % 1.) as f32;
        self.offset + (self.wave)(t) * self.amplitude
    }
}
