use super::*;

pub trait SignedDistance {
    fn dist(&self, point: &Vec3) -> f64;
    fn norm(&self, point: &Vec3) -> Vec3;
}

pub struct Sphere {
    pub pos: Vec3,
    pub radius: f64,
}

impl SignedDistance for Sphere {
    fn dist(&self, point: &Vec3) -> f64 {
        point.dist(&self.pos) - self.radius
    }

    fn norm(&self, point: &Vec3) -> Vec3 {
        Vec3 {
            x: (point.x - self.pos.x),
            y: (point.y - self.pos.y),
            z: (point.z - self.pos.z),
        }
        .normalized()
    }
}

pub struct Intersection {
    pub fields: Vec<Box<dyn SignedDistance>>,
}

impl SignedDistance for Intersection {
    fn dist(&self, point: &Vec3) -> f64 {
        self.fields
            .iter()
            .map(|f| f.dist(point))
            .reduce(f64::max)
            .unwrap_or(0.0)
    }

    fn norm(&self, point: &Vec3) -> Vec3 {
        let i = self
            .fields
            .iter()
            .map(|f| f.dist(point))
            .zip(0..)
            .reduce(|(dista, j), (dist, i)| match dist > dista {
                true => (dist, i),
                false => (dista, j),
            })
            .unwrap_or((0.0, 0))
            .1;
        self.fields[i].norm(point)
    }
}

pub struct Difference {
    pub field1: Box<dyn SignedDistance>,
    pub field2: Box<dyn SignedDistance>,
}

impl SignedDistance for Difference {
    fn dist(&self, point: &Vec3) -> f64 {
        f64::max(self.field1.dist(point), -self.field2.dist(point))
    }

    fn norm(&self, point: &Vec3) -> Vec3 {
        self.field1.norm(point)
    }
}
