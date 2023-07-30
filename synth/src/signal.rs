use crate::clock::Clock;

pub struct Signal {
    pub inner: Box<dyn IsSignal>,
}

impl Signal {
    pub fn value(&self, clock: &Clock) -> f32 {
        self.inner.value(clock)
    }
}

pub trait IsSignal: Send {
    fn value(&self, clock: &Clock) -> f32;
}

pub struct Value(pub f32);

impl IsSignal for Value {
    fn value(&self, _: &Clock) -> f32 {
        self.0
    }
}
