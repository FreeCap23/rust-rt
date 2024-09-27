use crate::vec3::Vec3;

struct Ray {
    orig: Vec3,
    dir: Vec3,
}

impl Ray {
    fn at(&self, t: f64) -> Vec3 {
        self.orig + self.dir * t
    }
}
