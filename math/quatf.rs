use crate::math::mat4vf::Mat4vf;
use crate::math::vec3f::Vec3f;
use crate::math::vec4f::Vec4f;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quatf {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quatf {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
    pub fn identity() -> Self {
        // Поворот на 0 градусов: вектор (0,0,0), скаляр 1
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }
    pub fn conjugate(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }
    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
                w: self.w / len,
            }
        } else {
            Self::identity()
        }
    }
    // Создание кватерниона из оси и угла (в радианах)
    pub fn from_axis_angle(axis: Vec3f, angle: f32) -> Self {
        let (sin, cos) = (angle * 0.5).sin_cos();
        let axis = axis.normalize();
        Self {
            x: axis.x * sin,
            y: axis.y * sin,
            z: axis.z * sin,
            w: cos,
        }
    }
    pub fn slerp(self, mut other: Self, t: f32) -> Self {
        let mut dot = self.dot(other);

        // Если dot отрицательный, Slerp пойдет по длинному пути.
        // Инвертируем один кватернион, чтобы всегда идти по кратчайшему.
        if dot < 0.0 {
            other = -other;
            dot = -dot;
        }

        if dot > 0.9995 {
            // Если кватернионы почти равны, используем обычный Lerp для скорости
            return (self + (other - self) * t).normalize();
        }

        let theta_0 = dot.acos();
        let theta = theta_0 * t;
        let sin_theta = theta.sin();
        let sin_theta_0 = theta_0.sin();

        let s0 = (theta_0 - theta).sin() / sin_theta_0;
        let s1 = sin_theta / sin_theta_0;

        (self * s0) + (other * s1)
    }
}
impl Add for Quatf {
    type Output = Self;

    fn add(self, r: Self) -> Self {
        Self {
            x: self.x + r.x,
            y: self.y + r.y,
            z: self.z + r.z,
            w: self.w + r.w,
        }
    }
}
impl Sub for Quatf {
    type Output = Self;

    fn sub(self, r: Self) -> Self {
        Self {
            x: self.x - r.x,
            y: self.y - r.y,
            z: self.z - r.z,
            w: self.w - r.w,
        }
    }
}
impl Mul for Quatf {
    type Output = Self;
    fn mul(self, r: Self) -> Self {
        Self {
            x: self.w * r.x + self.x * r.w + self.y * r.z - self.z * r.y,
            y: self.w * r.y - self.x * r.z + self.y * r.w + self.z * r.x,
            z: self.w * r.z + self.x * r.y - self.y * r.x + self.z * r.w,
            w: self.w * r.w - self.x * r.x - self.y * r.y - self.z * r.z,
        }
    }
}
impl Neg for Quatf {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }
}
impl Mul<f32> for Quatf {
    type Output = Self;

    fn mul(self, r: f32) -> Self {
        Self {
            x: self.x * r,
            y: self.y * r,
            z: self.z * r,
            w: self.w * r,
        }
    }
}
impl Div<f32> for Quatf {
    type Output = Self;

    fn div(self, r: f32) -> Self {
        Self {
            x: self.x / r,
            y: self.y / r,
            z: self.z / r,
            w: self.w / r,
        }
    }
}
impl From<Quatf> for Mat4vf {
    fn from(q: Quatf) -> Self {
        let x2 = q.x + q.x;
        let y2 = q.y + q.y;
        let z2 = q.z + q.z;
        let xx = q.x * x2;
        let xy = q.x * y2;
        let xz = q.x * z2;
        let yy = q.y * y2;
        let yz = q.y * z2;
        let zz = q.z * z2;
        let wx = q.w * x2;
        let wy = q.w * y2;
        let wz = q.w * z2;

        Self {
            cols: [
                Vec4f::new(1.0 - (yy + zz), xy + wz, xz - wy, 0.0),
                Vec4f::new(xy - wz, 1.0 - (xx + zz), yz + wx, 0.0),
                Vec4f::new(xz + wy, yz - wx, 1.0 - (xx + yy), 0.0),
                Vec4f::new(0.0, 0.0, 0.0, 1.0),
            ],
        }
    }
}

#[test]
fn test_quat_rotation() {
    let axis = Vec3f::new(0.0, 1.0, 0.0);
    let q = Quatf::from_axis_angle(axis, std::f32::consts::FRAC_PI_2);
    let m: Mat4vf = q.into();

    let v = Vec4f::new(0.0, 0.0, 1.0, 1.0);
    let res = m * v;

    // Проверяем близость к -1.0, а не строгое равенство
    assert!(
        (res.x - (1.0)).abs() < 0.0001,
        "Expected x ~ -1.0, got {}",
        res.x
    );
    assert!(res.z.abs() < 0.0001);
}
