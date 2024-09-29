use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct F64vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl ops::Mul<f64> for F64vec3 {
    type Output = F64vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        F64vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Add<F64vec3> for F64vec3 {
    type Output = F64vec3;
    fn add(self, rhs: F64vec3) -> Self::Output {
        F64vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl PartialEq for F64vec3 {
    fn eq(&self, other: &Self) -> bool {
        const EPS: f64 = f32::EPSILON as f64;
        (self.x - other.x).abs() < EPS
            && (self.y - other.y).abs() < EPS
            && (self.z - other.z).abs() < EPS
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3::F64vec3;

    #[test]
    fn test_add_vec3_to_vec3() {
        assert_eq!(
            F64vec3 {
                x: 1.5,
                y: 24.51,
                z: 12.4
            } + F64vec3 {
                x: 1.1,
                y: -4.5,
                z: 2.6
            },
            F64vec3 {
                x: 2.6,
                y: 20.01,
                z: 15.0
            }
        );
    }

    #[test]
    fn test_mul_f64_vec3() {
        assert_eq!(
            F64vec3 {
                x: 1.0,
                y: -13.2,
                z: 0.0
            } * 2.4,
            F64vec3 {
                x: 2.4,
                y: -31.68,
                z: 0.0
            }
        );
    }
}
