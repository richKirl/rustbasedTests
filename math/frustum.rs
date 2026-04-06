use crate::math::mat4vf::Mat4vf;
use crate::math::vec3f::Vec3f;
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane {
    pub normal: Vec3f,
    pub distance: f32,
}

impl Plane {
    pub fn distance_to_point(&self, point: Vec3f) -> f32 {
        self.normal.dot(point) + self.distance
    }
}

pub struct Frustum {
    pub planes: [Plane; 6], // Left, Right, Bottom, Top, Near, Far
}

impl Frustum {
    pub fn from_matrix(m: &Mat4vf) -> Self {
        // Берем колонки для удобства
        let c0 = m.cols[0];
        let c1 = m.cols[1];
        let c2 = m.cols[2];
        let c3 = m.cols[3];

        let mut planes = [Plane {
            normal: Vec3f::new(0.0, 0.0, 0.0),
            distance: 0.0,
        }; 6];

        // Внутренняя функция теперь принимает целиком колонку, которую мы складываем с 4-й
        let extract = |col: crate::math::vec4f::Vec4f, sign: f32| -> Plane {
            let n = Vec3f::new(
                c3.x + sign * col.x,
                c3.y + sign * col.y,
                c3.z + sign * col.z,
            );
            let d = c3.w + sign * col.w;
            let length = n.length();

            Plane {
                normal: n / length,
                distance: d / length,
            }
        };

        planes[0] = extract(c0, -1.0); // Left  (было 1.0)
        planes[1] = extract(c0, 1.0); // Right (было -1.0)
        planes[2] = extract(c1, -1.0); // Bottom (было 1.0)
        planes[3] = extract(c1, 1.0); // Top    (было -1.0)
        planes[4] = extract(c2, -1.0); // Near
        planes[5] = extract(c2, 1.0); // Far

        Self { planes }
    }
    pub fn is_sphere_visible(&self, center: Vec3f, radius: f32) -> bool {
        for plane in &self.planes {
            if plane.distance_to_point(center) < -radius {
                return false; // Сфера полностью за плоскостью
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::mat4vf::Mat4vf;
    use crate::math::vec3f::Vec3f;

    #[test]
    fn test_frustum_visibility() {
        // 1. Создаем матрицу перспективы: FOV 90 градусов, Aspect 1.0, Near 0.1, Far 100.0
        let projection = Mat4vf::perspective(90.0f32.to_radians(), 1.0, 0.1, 1000.0);

        // 2. Извлекаем фрустум
        let frustum = Frustum::from_matrix(&projection);

        // // 3. Тест: точка ПРЯМО ПЕРЕД камерой (в OpenGL вперед это -Z)
        let center_in = Vec3f::new(0.0, 0.0, -5.0);
        assert!(
            frustum.is_sphere_visible(center_in, 5.0),
            "Сфера должна быть видна перед камерой1"
        );

        // 4. Тест: точка ПОЗАДИ камеры (положительный Z)
        let center_out = Vec3f::new(0.0, 0.0, 5.0);
        assert!(
            !frustum.is_sphere_visible(center_out, 1.0),
            "Сфера позади камеры не должна быть видна2"
        );

        // 5. Тест: точка ДАЛЕКО за плоскостью Far
        let center_far = Vec3f::new(0.0, 0.0, -150.0);
        assert!(
            !frustum.is_sphere_visible(center_far, 1.0),
            "Сфера слишком далеко не должна быть видна3"
        );
    }
}
