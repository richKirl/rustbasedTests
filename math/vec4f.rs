use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[repr(C)] // Гарантирует порядок полей x, y, z как в C
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4f {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
    pub fn dot(&self, other: Self) -> f32 {
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
    pub fn length(&self) -> f32 {
        self.dot(*self).sqrt()
    }
    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 0.0 {
            *self / len // Использует вашу реализацию Div для f32
        } else {
            Self::new(0.0, 0.0, 0.0, 0.0) // Избегаем деления на 0
        }
    }
}

impl Add for Vec4f {
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
impl Add<f32> for Vec4f {
    type Output = Self; // Что получится в итоге (тоже Vec3)

    fn add(self, other: f32) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
            w: self.w + other,
        }
    }
}
impl Sub for Vec4f {
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
impl Sub<f32> for Vec4f {
    type Output = Self; // Что получится в итоге (тоже Vec3)

    fn sub(self, other: f32) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
            w: self.w - other,
        }
    }
}
impl Mul for Vec4f {
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
impl Mul<f32> for Vec4f {
    type Output = Self; // Что получится в итоге (тоже Vec3)

    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}
impl Div for Vec4f {
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
impl Div<f32> for Vec4f {
    type Output = Self; // Что получится в итоге (тоже Vec3)

    fn div(self, other: f32) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}
impl AddAssign for Vec4f {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}
impl AddAssign<f32> for Vec4f {
    fn add_assign(&mut self, scalar: f32) {
        self.x += scalar;
        self.y += scalar;
        self.z += scalar;
    }
}
impl SubAssign for Vec4f {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}
impl SubAssign<f32> for Vec4f {
    fn sub_assign(&mut self, scalar: f32) {
        self.x -= scalar;
        self.y -= scalar;
        self.z -= scalar;
    }
}
impl MulAssign for Vec4f {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}
impl MulAssign<f32> for Vec4f {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl DivAssign for Vec4f {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}
impl DivAssign<f32> for Vec4f {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

impl Neg for Vec4f {
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
        let v1 = Vec4f::new(1.0, 0.0, 0.0, 0.0);
        let v2 = Vec4f::new(0.0, 1.0, 0.0, 0.0);
        assert_eq!(v1.dot(v2), 0.0);
    }

    #[test]
    fn test_length() {
        let v = Vec4f::new(3.0, 4.0, 0.0, 0.0);
        assert_eq!(v.length(), 5.0);
    }
}
// let my_vec = Vec4f { x: 1.0, y: 2.0, z: 3.0 };

// // Получаем указатель на первый элемент как на f32
// let ptr = &my_vec as *const Vec4f as *const f32;

// unsafe {
//     gl::BufferData(
//         gl::ARRAY_BUFFER,
//         std::mem::size_of::<Vec4f>() as isize,
//         ptr as *const _,
//         gl::STATIC_DRAW,
//     );
// }
