use super::*;

pub trait SignedDistance {
    fn dist(&self, point: &Vec3) -> f64;
}

pub struct Sphere {
    pub pos: Vec3,
    pub radius: f64,
}

impl SignedDistance for Sphere {
    fn dist(&self, point: &Vec3) -> f64 {
        point.dist(&self.pos) - self.radius
    }
}
