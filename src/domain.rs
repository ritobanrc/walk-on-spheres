use crate::math::*;

pub trait Levelset: Sync {
    fn phi(&self, x: Vector) -> Real;
}

pub struct Circle {
    pub radius: Real,
}

impl Levelset for Circle {
    fn phi(&self, x: Vector) -> Real {
        x.norm() - self.radius
    }
}

pub struct Rect {
    pub size: Vector,
}

impl Levelset for Rect {
    fn phi(&self, x: Vector) -> Real {
        let d = x.abs() - self.size;
        d.component_max(&Vector::zeros()).magnitude() + f64::max(d.x, d.y).min(0.)
    }
}
