use crate::math::vec3f::Vec3f;
use crate::math::vec4f::Vec4f;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat4vf {
    pub cols: [Vec4f; 4],
}

impl Mat4vf {
    // Единичная матрица (Identity) — ничего не меняет
    pub fn identity() -> Self {
        Self {
            cols: [
                Vec4f::new(1.0, 0.0, 0.0, 0.0),
                Vec4f::new(0.0, 1.0, 0.0, 0.0),
                Vec4f::new(0.0, 0.0, 1.0, 0.0),
                Vec4f::new(0.0, 0.0, 0.0, 1.0),
            ],
        }
    }
    pub fn translate_3f(x: f32, y: f32, z: f32) -> Self {
        let mut mat = Self::identity();
        // В Column-Major четвертая колонка (индекс 3) отвечает за перенос
        mat.cols[3] = Vec4f::new(x, y, z, 1.0);
        mat
    }

    // Для удобства, если уже есть Vec3f
    pub fn translate_v3f(v: Vec3f) -> Self {
        Self::translate_3f(v.x, v.y, v.z)
    }
    pub fn scale_3f(x: f32, y: f32, z: f32) -> Self {
        Self {
            cols: [
                Vec4f::new(x, 0.0, 0.0, 0.0),
                Vec4f::new(0.0, y, 0.0, 0.0),
                Vec4f::new(0.0, 0.0, z, 0.0),
                Vec4f::new(0.0, 0.0, 0.0, 1.0),
            ],
        }
    }
    pub fn scale_v3f(v: Vec3f) -> Self {
        Self {
            cols: [
                Vec4f::new(v.x, 0.0, 0.0, 0.0),
                Vec4f::new(0.0, v.y, 0.0, 0.0),
                Vec4f::new(0.0, 0.0, v.z, 0.0),
                Vec4f::new(0.0, 0.0, 0.0, 1.0),
            ],
        }
    }
    pub fn look_at(eye: Vec3f, center: Vec3f, up: Vec3f) -> Self {
        let f = (center - eye).normalize(); // Направление "вперед"
        let s = f.cross(up).normalize(); // Направление "вправо"
        let u = s.cross(f); // Направление "вверх" (ортогональное)

        let mut res = Self::identity();
        // Заполняем ориентацию (верхние 3x3)
        res.cols[0] = Vec4f::new(s.x, u.x, -f.x, 0.0);
        res.cols[1] = Vec4f::new(s.y, u.y, -f.y, 0.0);
        res.cols[2] = Vec4f::new(s.z, u.z, -f.z, 0.0);
        // Добавляем смещение камеры (позиция eye)
        res.cols[3] = Vec4f::new(-s.dot(eye), -u.dot(eye), f.dot(eye), 1.0);
        res
    }
    pub fn perspective(fovy: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fovy / 2.0).tan();
        let mut res = Self {
            cols: [Vec4f::new(0.0, 0.0, 0.0, 0.0); 4],
        };

        res.cols[0].x = f / aspect;
        res.cols[1].y = f;

        // Третья колонка (индекс 2)
        res.cols[2].z = (far + near) / (near - far);
        res.cols[2].w = -1.0; // Это критично для деления на W в шейдере

        // Четвертая колонка (индекс 3)
        res.cols[3].z = (2.0 * far * near) / (near - far);
        res.cols[3].w = 0.0; // Здесь должен быть ноль

        res
    }
    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let mut res = Self::identity();

        res.cols[0].x = 2.0 / (right - left);
        res.cols[1].y = 2.0 / (top - bottom);
        res.cols[2].z = -2.0 / (far - near);

        res.cols[3].x = -(right + left) / (right - left);
        res.cols[3].y = -(top + bottom) / (top - bottom);
        res.cols[3].z = -(far + near) / (far - near);

        res
    }
}

impl Mul<Vec4f> for Mat4vf {
    type Output = Vec4f;

    fn mul(self, v: Vec4f) -> Vec4f {
        // Каждая колонка умножается на свой компонент вектора
        (self.cols[0] * v.x) + (self.cols[1] * v.y) + (self.cols[2] * v.z) + (self.cols[3] * v.w)
    }
}
impl Mul for Mat4vf {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            cols: [
                self * other.cols[0], // Умножаем левую матрицу на каждую колонку правой
                self * other.cols[1],
                self * other.cols[2],
                self * other.cols[3],
            ],
        }
    }
}
