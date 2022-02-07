use std::ops;

use rand::{
    prelude::{SmallRng, ThreadRng},
    Rng,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3 {
    elements: [f64; 3],
}

impl Vec3 {
    pub fn new() -> Self {
        Self { elements: [0.0; 3] }
    }

    pub fn from_coords(x: f64, y: f64, z: f64) -> Self {
        return Self {
            elements: [x, y, z],
        };
    }

    pub fn x(&self) -> f64 {
        return self.elements[0];
    }

    pub fn y(&self) -> f64 {
        return self.elements[1];
    }

    pub fn z(&self) -> f64 {
        return self.elements[2];
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.elements.iter().map(|x| x * x).sum()
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.elements
            .iter()
            .zip(other.elements)
            .map(|(x, y)| x * y)
            .sum()
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        return Vec3::from_coords(
            self.elements[1] * other.elements[2] - self.elements[2] * other.elements[1],
            self.elements[2] * other.elements[0] - self.elements[0] * other.elements[2],
            self.elements[0] * other.elements[1] - self.elements[1] * other.elements[0],
        );
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn random(min: f64, max: f64, rng: &mut ThreadRng) -> Vec3 {
        let x: f64 = rng.gen_range(min..=max);
        let y: f64 = rng.gen_range(min..=max);
        let z: f64 = rng.gen_range(min..=max);

        return Vec3::from_coords(x, y, z);
    }

    pub fn random_in_unit_sphere(rng: &mut SmallRng) -> Vec3 {
        loop {
            let random = Vec3::from_coords(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
                * 2.0
                - Vec3::new();
            if random.length_squared() >= 1.0 {
                continue;
            }
            return random;
        }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        let mut result = Vec3::new();
        for (i, (lval, rval)) in self.elements.iter().zip(rhs.elements).enumerate() {
            result.elements[i] = lval + rval;
        }
        result
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        let mut result = Vec3::new();
        for (i, (lval, rval)) in self.elements.iter().zip(rhs.elements).enumerate() {
            result.elements[i] = lval - rval;
        }
        result
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = *self - rhs
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::from_coords(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        return self * (1f64 / rhs);
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        let mut result = Vec3::new();
        for (i, val) in self.elements.iter().enumerate() {
            result.elements[i] = -val;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_delta {
        ($x:expr, $y:expr, $d:expr) => {
            if $x - $y > $d {
                panic!();
            }
        };
    }

    #[test]
    fn test_dot() {
        let u = Vec3::from_coords(1.0, 2.0, 3.0);
        let v = Vec3::from_coords(4.0, -5.0, 6.0);

        assert_eq!(u.dot(&v), 12f64)
    }

    #[test]
    fn test_cross() {
        let u = Vec3::from_coords(1.0, 2.0, 3.0);
        let v = Vec3::from_coords(4.0, -5.0, 6.0);

        let expected = Vec3::from_coords(27.0, 6.0, -13.0);
        assert_eq!(u.cross(&v), expected)
    }

    #[test]
    fn test_unit() {
        let u = Vec3::from_coords(4.0, 7.0, 2.0);
        let unit = u.unit();

        assert_delta!(unit.x(), 0.481, 0.01);
        assert_delta!(unit.y(), 0.842, 0.01);
        assert_delta!(unit.z(), 0.240, 0.01);
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;
