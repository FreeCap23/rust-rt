use glam::{self, DVec3};

#[allow(dead_code)]
struct Ray {
    orig: DVec3,
    dir: DVec3,
}

impl Ray {
    #[allow(dead_code)]
    fn at(&self, t: f64) -> DVec3 {
        self.orig + self.dir * t
    }
}
