use crate::math::quatf::Quatf;
use crate::math::vec3f::Vec3f;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DualQuat {
    pub real: Quatf,
    pub dual: Quatf,
}

impl DualQuat {
    pub fn new(rotation: Quatf, translation: Vec3f) -> Self {
        let r = rotation.normalize();
        let t = Quatf::new(translation.x, translation.y, translation.z, 0.0);
        Self {
            real: r,
            // Попробуйте этот порядок (t перед r):
            dual: (t * r) * 0.5,
        }
    }

    pub fn identity() -> Self {
        Self {
            real: Quatf::identity(),
            dual: Quatf::new(0.0, 0.0, 0.0, 0.0), // Нулевой дуальный компонент
        }
    }
    pub fn conjugate(&self) -> Self {
        Self {
            real: self.real.conjugate(),
            dual: self.dual.conjugate(),
        }
    }
    pub fn get_translation(&self) -> Vec3f {
        // Формула: 2 * dual * conjugate(real)
        let t_quat = (self.dual * self.real.conjugate()) * 2.0;
        Vec3f::new(t_quat.x, t_quat.y, t_quat.z)
    }
    pub fn sclerp(self, other: Self, t: f32) -> Self {
        let mut dot = self.real.dot(other.real);
        let mut other_adj = other;

        if dot < 0.0 {
            other_adj.real = -other.real;
            other_adj.dual = -other.dual;
        }

        // Линейная интерполяция обоих компонентов с последующей нормализацией
        // Для Dual Quaternions это дает отличный результат (алгоритм DLB)
        let res = Self {
            real: self.real + (other_adj.real - self.real) * t,
            dual: self.dual + (other_adj.dual - self.dual) * t,
        };

        res.normalize()
    }

    pub fn normalize(self) -> Self {
        let mag = self.real.length();
        if mag > 0.0 {
            Self {
                real: self.real / mag,
                dual: self.dual / mag,
            }
        } else {
            Self::identity()
        }
    }
}
impl Mul for DualQuat {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            real: self.real * other.real,
            dual: (self.real * other.dual) + (self.dual * other.real),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::quatf::Quatf;
    use crate::math::vec3f::Vec3f;
    use std::f32::consts::FRAC_PI_2;

    #[test]
    fn test_dual_quat_identity() {
        let dq = DualQuat::identity();
        let t = dq.get_translation();

        assert!(t.x.abs() < 0.0001);
        assert!(t.y.abs() < 0.0001);
        assert!(t.z.abs() < 0.0001);
        assert_eq!(dq.real, Quatf::identity());
    }
    #[test]
    fn test_dual_quat_translation_and_rotation() {
        let start = DualQuat::identity(); // Начинаем из нуля

        let rotation = Quatf::from_axis_angle(Vec3f::new(0.0, 1.0, 0.0), FRAC_PI_2);
        let translation = Vec3f::new(10.0, 0.0, 0.0);
        let end = DualQuat::new(rotation, translation); // Идем в (10, 0, 0)

        // 1. Проверяем извлечение из конечной точки
        let extracted_t = end.get_translation();
        assert!((extracted_t.x - 10.0).abs() < 0.0001);

        // 2. Проверяем СЕРЕДИНУ (между start и end)
        let mid = start.sclerp(end, 0.5);
        let pos = mid.get_translation();

        // Теперь здесь ДОЛЖНО быть 5.0
        assert!((pos.x - 5.0).abs() < 0.001, "Expected 5.0, got {}", pos.x);
    }
    // #[test]
    // fn test_dual_quat_translation_and_rotation() {
    //     // 1. Задаем поворот (90 град по Y) и смещение (10 по X)
    //     let rotation = Quatf::from_axis_angle(Vec3f::new(0.0, 1.0, 0.0), FRAC_PI_2);
    //     let translation = Vec3f::new(10.0, 0.0, 0.0);
    //     let end = DualQuat::new(rotation, translation);
    //     // 2. Создаем DualQuat
    //     let dq = DualQuat::new(rotation, translation);

    //     // 3. Проверяем извлечение позиции
    //     let extracted_t = dq.get_translation();
    //     let mid = dq.sclerp(end, 0.5);
    //     let pos = mid.get_translation();
    //     //println!("{:?}", mid.get_translation());
    //     assert!((pos.x - 5.0).abs() < 0.001);

    //     assert!(
    //         (extracted_t.x - 10.0).abs() < 0.0001,
    //         "Expected x=10, got {}",
    //         extracted_t.x
    //     );

    //     // 4. Проверяем извлечение поворота
    //     assert!((dq.real.x - rotation.x).abs() < 0.0001);
    //     assert!((dq.real.w - rotation.w).abs() < 0.0001);
    // }
}
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::math::quatf::Quatf;
//     use crate::math::vec3f::Vec3f;
//     use std::f32::consts::PI;

//     #[test]
//     fn test_dual_quat_sclerp_full_transform() {
//         // 1. Начальная точка: без поворота, позиция (0, 0, 0)
//         let start = DualQuat::identity();

//         // 2. Конечная точка: поворот на 180 градусов по Y, позиция (10, 0, 0)
//         let rotation_end = Quatf::from_axis_angle(Vec3f::new(0.0, 1.0, 0.0), PI);
//         let translation_end = Vec3f::new(10.0, 0.0, 0.0);
//         let end = DualQuat::new(rotation_end, translation_end);

//         // 3. Интерполяция на 50% (t = 0.5)
//         let mid = start.sclerp(end, 0.5);

//         // 4. Проверяем позицию: должна быть (5, 0, 0)
//         let pos = mid.get_translation();
//         println!("{:?}", mid.get_translation());
//         assert!((pos.x - 5.0).abs() < 0.001, "Expected x=5.0, got {}", pos.x);
//         assert!(pos.y.abs() < 0.001);
//         assert!(pos.z.abs() < 0.001);

//         // 5. Проверяем поворот: на t=0.5 угол должен быть 90 градусов (PI/2)
//         // В кватернионе поворота по Y на 90 градусов компонента W (cos(angle/2)) должна быть ~0.707
//         let expected_w = (PI / 4.0).cos(); // cos(45 градусов)
//         assert!(
//             (mid.real.w - expected_w).abs() < 0.001,
//             "Expected w={}, got {}",
//             expected_w,
//             mid.real.w
//         );
//     }
// #[test]
// fn test_dual_quat_sclerp_90_deg() {
//     let start = DualQuat::identity();
//     // Поворот на 90 градусов, а не на 180
//     let rotation_end =
//         Quatf::from_axis_angle(Vec3f::new(0.0, 1.0, 0.0), std::f32::consts::FRAC_PI_2);
//     let translation_end = Vec3f::new(10.0, 0.0, 0.0);
//     let end = DualQuat::new(rotation_end, translation_end);

//     let mid = start.sclerp(end, 0.5);
//     let pos = mid.get_translation();
//     println!("{:?}", mid.get_translation());
//     // На угле 90 градусов DLB должен отработать гораздо точнее
//     assert!((pos.x - 5.0).abs() < 0.001);
//     assert!(pos.z.abs() < 0.001);
// }
// }
