use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct Time {
    last:   Instant,
    delta:  Duration,
    total:  Duration,
}

impl Time {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            last:  now,
            delta: Duration::ZERO,
            total: Duration::ZERO,
        }
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        self.delta = now - self.last;
        self.total += self.delta;
        self.last = now;
    }

    pub fn delta_seconds(&self) -> f32 {
        self.delta.as_secs_f32()
    }

    pub fn total_seconds(&self) -> f32 {
        self.total.as_secs_f32()
    }
}
