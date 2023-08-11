use crate::math::*;

pub trait DirichletBoundary: Sync {
    fn value(&self, x: Vector) -> Real;
}
pub struct Sinusoid {
    pub frequency: Real,
}

impl DirichletBoundary for Sinusoid {
    fn value(&self, pos: Vector) -> Real {
        (self.frequency * Real::atan2(pos.y, pos.x)).sin()
    }
}
