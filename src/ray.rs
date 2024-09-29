use crate::vec3::F64vec3;

struct Ray {
    orig: F64vec3,
    dir: F64vec3,
}

impl Ray {
    fn at(&self, t: f64) -> F64vec3 {
        self.orig + self.dir * t
    }
}
