use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[repr(C)] // Гарантирует порядок полей x, y, z как в C
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x: x, y: y, z: z }
    }
    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
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
            Self::new(0.0, 0.0, 0.0) // Избегаем деления на 0
        }
    }
}

impl Add for Vec3d {
    type Output = Self; // Что получится в итоге (тоже Vec3)

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Add<f64> for Vec3d {
    type Output = Self; // Что получится в итоге (тоже Vec3)

    fn add(self, other: f64) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}
impl Sub for Vec3d {
    type Output = Self; // Что получится в итоге (тоже Vec3)

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl Sub<f64> for Vec3d {
    type Output = Self; // Что получится в итоге (тоже Vec3)

    fn sub(self, other: f64) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}
impl Mul for Vec3d {
    type Output = Self; // Что получится в итоге (тоже Vec3)

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}
impl Mul<f64> for Vec3d {
    type Output = Self; // Что получится в итоге (тоже Vec3)

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
impl Div for Vec3d {
    type Output = Self; // Что получится в итоге (тоже Vec3)

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}
impl Div<f64> for Vec3d {
    type Output = Self; // Что получится в итоге (тоже Vec3)

    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}
impl AddAssign for Vec3d {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}
impl AddAssign<f64> for Vec3d {
    fn add_assign(&mut self, scalar: f64) {
        self.x += scalar;
        self.y += scalar;
        self.z += scalar;
    }
}
impl SubAssign for Vec3d {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}
impl SubAssign<f64> for Vec3d {
    fn sub_assign(&mut self, scalar: f64) {
        self.x -= scalar;
        self.y -= scalar;
        self.z -= scalar;
    }
}
impl MulAssign for Vec3d {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}
impl MulAssign<f64> for Vec3d {
    fn mul_assign(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl DivAssign for Vec3d {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}
impl DivAssign<f64> for Vec3d {
    fn div_assign(&mut self, scalar: f64) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

impl Neg for Vec3d {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot() {
        let v1 = Vec3d::new(1.0, 0.0, 0.0);
        let v2 = Vec3d::new(0.0, 1.0, 0.0);
        assert_eq!(v1.dot(v2), 0.0);
    }

    #[test]
    fn test_length() {
        let v = Vec3d::new(3.0, 4.0, 0.0);
        assert_eq!(v.length(), 5.0);
    }
}
// let my_vec = Vec3d { x: 1.0, y: 2.0, z: 3.0 };

// // Получаем указатель на первый элемент как на f64
// let ptr = &my_vec as *const Vec3d as *const f64;

// unsafe {
//     gl::BufferData(
//         gl::ARRAY_BUFFER,
//         std::mem::size_of::<Vec3d>() as isize,
//         ptr as *const _,
//         gl::STATIC_DRAW,
//     );
// }
