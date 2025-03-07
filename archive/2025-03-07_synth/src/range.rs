#[derive(Clone, Copy, Debug)]
pub struct Range {
    pub min: f32,
    pub max: f32,
}

impl Range {
    pub const fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn width(&self) -> f32 {
        self.max - self.min
    }

    pub fn contains(&self, value: f32) -> bool {
        assert!(value.is_finite());
        (self.min..=self.max).contains(&value)
    }

    pub fn convert_value_to(&self, value: f32, other: Range) -> f32 {
        assert!(
            self.contains(value),
            "assertion failed: {self:?}.contains({value:?})"
        );
        other.min + other.width() / self.width() * (value - self.min)
    }
}

pub const VALUE: Range = Range::new(-1., 1.);
pub const AUDIBLE: Range = Range::new(20., 20_000.);
pub const LFO: Range = Range::new(0.1, 20.);
pub const AMPLIFIER: Range = Range::new(0., 100.);
