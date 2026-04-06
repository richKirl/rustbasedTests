use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[repr(C)] // Гарантирует порядок полей x, y, z как в C
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec4d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vec4d {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            w: 0.0,
        }
    }
    pub fn length(&self) -> f64 {
        self.dot(*self).sqrt()
    }
    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 0.0 {
            *self / len // Использует вашу реализацию Div для f64
        } else {
            Self::new(0.0, 0.0, 0.0, 0.0) // Избегаем деления на 0
        }
    }
}

impl Add for Vec4d {
    type Output = Self; // Что получится в итоге (тоже Vec3)

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}
impl Add<f64> for Vec4d {
    type Output = Self; // Что получится в итоге (тоже Vec3)

    fn add(self, other: f64) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
            w: self.w + other,
        }
    }
}
impl Sub for Vec4d {
    type Output = Self; // Что получится в итоге (тоже Vec3)

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}
impl Sub<f64> for Vec4d {
    type Output = Self; // Что получится в итоге (тоже Vec3)

    fn sub(self, other: f64) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
            w: self.w - other,
        }
    }
}
impl Mul for Vec4d {
    type Output = Self; // Что получится в итоге (тоже Vec3)

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }
}
impl Mul<f64> for Vec4d {
    type Output = Self; // Что получится в итоге (тоже Vec3)

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}
impl Div for Vec4d {
    type Output = Self; // Что получится в итоге (тоже Vec3)

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            w: self.w / other.w,
        }
    }
}
impl Div<f64> for Vec4d {
    type Output = Self; // Что получится в итоге (тоже Vec3)

    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}
impl AddAssign for Vec4d {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}
impl AddAssign<f64> for Vec4d {
    fn add_assign(&mut self, scalar: f64) {
        self.x += scalar;
        self.y += scalar;
        self.z += scalar;
    }
}
impl SubAssign for Vec4d {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}
impl SubAssign<f64> for Vec4d {
    fn sub_assign(&mut self, scalar: f64) {
        self.x -= scalar;
        self.y -= scalar;
        self.z -= scalar;
    }
}
impl MulAssign for Vec4d {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}
impl MulAssign<f64> for Vec4d {
    fn mul_assign(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl DivAssign for Vec4d {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}
impl DivAssign<f64> for Vec4d {
    fn div_assign(&mut self, scalar: f64) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

impl Neg for Vec4d {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot() {
        let v1 = Vec4d::new(1.0, 0.0, 0.0, 0.0);
        let v2 = Vec4d::new(0.0, 1.0, 0.0, 0.0);
        assert_eq!(v1.dot(v2), 0.0);
    }

    #[test]
    fn test_length() {
        let v = Vec4d::new(3.0, 4.0, 0.0, 0.0);
        assert_eq!(v.length(), 5.0);
    }
}
