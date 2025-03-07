use crate::{
    range::Range,
    signal::{IntoSignal, Signal, Value},
};

pub trait Limit {
    fn limit(
        self,
        min: impl IntoSignal,
        max: impl IntoSignal,
        range: Range,
    ) -> Signal;
}

impl Limit for Signal {
    fn limit(
        mut self,
        min: impl IntoSignal,
        max: impl IntoSignal,
        range: Range,
    ) -> Signal {
        let mut min = min.into_signal(range);
        let mut max = max.into_signal(range);

        Signal::from_fn(move || {
            let value = self.next_value().decode_to(range);

            let min = min.next_value().decode_to(range);
            let max = max.next_value().decode_to(range);

            let value = value.clamp(min, max);
            Value::encode_from(value, range)
        })
    }
}
