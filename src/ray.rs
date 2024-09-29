use crate::vec3::F64vec3;

#[allow(dead_code)]
struct Ray {
    orig: F64vec3,
    dir: F64vec3,
}

impl Ray {
    #[allow(dead_code)]
    fn at(&self, t: f64) -> F64vec3 {
        self.orig + self.dir * t
    }
}
