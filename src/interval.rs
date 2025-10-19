use std::ops::Range;

#[derive(Copy, Clone)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Default for Interval {
    fn default() -> Interval {
        Interval {
            min: f32::NEG_INFINITY,
            max: f32::INFINITY,
        }
    }
}

impl From<Range<f32>> for Interval {
    fn from(range: Range<f32>) -> Interval {
        Interval {
            min: range.start,
            max: range.end,
        }
    }
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f32) -> f32 {
        x.clamp(self.min, self.max)
    }

    pub fn expand(&self, delta: f32) -> Self {
        let padding = delta / 2.0;
        Self::new(self.min - padding, self.max + padding)
    }

    pub const EMPTY: Self = Self {
        min: f32::INFINITY,
        max: f32::NEG_INFINITY,
    };

    pub const UNIVERSE: Self = Self {
        min: f32::NEG_INFINITY,
        max: f32::INFINITY,
    };
}

impl From<(&Interval, &Interval)> for Interval {
    fn from((a, b): (&Interval, &Interval)) -> Self {
        Self {
            min: a.min.min(b.min),
            max: a.max.min(b.max),
        }
    }
}
