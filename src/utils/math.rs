use std::ops::{AddAssign, Div, Mul, Sub};

use super::*;

impl<T, const M: usize, const N: usize> Mat<T, M, N>
where
    T: Copy + Into<f32> + Sub<Output = T>,
{
    /// 容许浮点误差等比较
    pub fn eq_with_epsilon(&self, other: &Self, epsilon: f32) -> bool {
        for i in 0..M {
            for j in 0..N {
                let diff: f32 = (self[i][j] - other[i][j]).into();
                if diff.abs() > epsilon {
                    return false;
                }
            }
        }
        true
    }
}

impl<T, const M: usize> Vec<T, M>
where
    T: Copy + Default + Into<f32>,
    T: AddAssign + Mul<Output = T>,
{
    /// 计算向量的模长
    pub fn norm(&self) -> f32 {
        let s: f32 = (*self * self.transpose())[0][0].into();
        s.sqrt().into()
    }
}

impl<T, const M: usize> Vec<T, M>
where
    T: Copy + Default + Into<f32>,
    T: AddAssign + Mul<Output = T> + Div<Output = T>,
    T: From<f32>,
{
    /// 将向量归一化
    pub fn normalize(&self) -> Self {
        let norm = self.norm();
        if norm == 0.0 {
            return Self::new();
        }
        *self / norm.into()
    }
}

impl<T> Vec3<T>
where
    T: Copy + Default + Into<f32>,
    T: Mul<Output = T> + Sub<Output = T>,
{
    /// 计算两个向量的叉乘
    pub fn cross(&self, other: Self) -> Self {
        let mut result = Self::new();
        result.set_x(*self.y() * *other.z() - *self.z() * *other.y());
        result.set_y(*self.z() * *other.x() - *self.x() * *other.z());
        result.set_z(*self.x() * *other.y() - *self.y() * *other.x());
        result
    }
}

/// 生成二维旋转变换矩阵
///
/// # 参数
///
/// - `angle` 旋转角度，单位为弧度
///
/// # 返回值
///
/// 旋转矩阵
pub fn rotate2(angle: f32) -> Mat3<f32> {
    let c = angle.cos();
    let s = angle.sin();
    Mat3::from([[c, -s, 0.0], [s, c, 0.0], [0.0, 0.0, 1.0]])
}

/// 生成二维平移变换矩阵
///
/// # 参数
///
/// - `v` 平移向量
///
/// # 返回值
///
/// 平移矩阵
pub fn tranlate2(v: Vec2<f32>) -> Mat3<f32> {
    Mat3::from([[1.0, 0.0, *v.x()], [0.0, 1.0, *v.y()], [0.0, 0.0, 1.0]])
}

/// 生成二维缩放变换矩阵
///
/// # 参数
///
/// - `v` 缩放因子
///
/// # 返回值
///
/// 缩放矩阵
pub fn scale2(v: Vec2<f32>) -> Mat3<f32> {
    Mat3::from([[*v.x(), 0.0, 0.0], [0.0, *v.y(), 0.0], [0.0, 0.0, 1.0]])
}

/// 生成三维旋转变换矩阵
///
/// # 参数
///
/// - `angle` 旋转角度，单位为弧度
/// - `axis` 旋转轴
///
/// # 返回值
///
/// 旋转矩阵
pub fn rotate3(angle: f32, axis: Vec3<f32>) -> Mat4<f32> {
    let mut result = Mat4::I();
    let axis = axis.normalize();
    let c = angle.cos();
    let s = angle.sin();
    let k = Mat::from([
        [0.0, -*axis.z(), *axis.y()],
        [*axis.z(), 0.0, -*axis.x()],
        [-*axis.y(), *axis.x(), 0.0],
    ]);
    let r = Mat3::I() + k * s + (k * k) * (1.0 - c);
    for i in 0..3 {
        for j in 0..3 {
            result[i][j] = r[i][j];
        }
    }
    result
}

/// 生成三维平移变换矩阵
///
/// # 参数
///
/// - `v` 平移向量
///
/// # 返回值
///
/// 平移矩阵
pub fn tranlate3(v: Vec3<f32>) -> Mat4<f32> {
    Mat4::from([
        [1.0, 0.0, 0.0, *v.x()],
        [0.0, 1.0, 0.0, *v.y()],
        [0.0, 0.0, 1.0, *v.z()],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

/// 生成三维缩放变换矩阵
///
/// # 参数
///
/// - `v` 缩放因子
///
/// # 返回值
///
/// 缩放矩阵
pub fn scale3(v: Vec3<f32>) -> Mat4<f32> {
    Mat4::from([
        [*v.x(), 0.0, 0.0, 0.0],
        [0.0, *v.y(), 0.0, 0.0],
        [0.0, 0.0, *v.z(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

/// 生成三维绕x轴旋转变换矩阵
///
/// # 参数
///
/// - `angle` 旋转角度，单位为弧度
///
/// # 返回值
///
/// 旋转矩阵
pub fn rotate3_x(angle: f32) -> Mat4<f32> {
    let c = angle.cos();
    let s = angle.sin();
    Mat4::from([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, c, -s, 0.0],
        [0.0, s, c, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

/// 生成三维绕y轴旋转变换矩阵
///
/// # 参数
///
/// - `angle` 旋转角度，单位为弧度
///
/// # 返回值
///
/// 旋转矩阵
pub fn rotate3_y(angle: f32) -> Mat4<f32> {
    let c = angle.cos();
    let s = angle.sin();
    Mat4::from([
        [c, 0.0, s, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-s, 0.0, c, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

/// 生成三维绕z轴旋转变换矩阵
///
/// # 参数
///
/// - `angle` 旋转角度，单位为弧度
///
/// # 返回值
///
/// 旋转矩阵
pub fn rotate3_z(angle: f32) -> Mat4<f32> {
    let c = angle.cos();
    let s = angle.sin();
    Mat4::from([
        [c, -s, 0.0, 0.0],
        [s, c, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;
    const SQRT_3: f32 = 1.732_050_807_568_877_2;

    #[test]
    fn test_norm_happy_path() {
        let v = Vec3::from([3.0, 4.0, 0.0]);
        assert_eq!(v.norm(), 5.0);

        let v = Vec3::from([1.0, 1.0, 1.0]);
        assert_eq!(v.norm(), SQRT_3);

        let v = Vec3::from([0.0, 0.0, 5.0]);
        assert_eq!(v.norm(), 5.0);
    }

    #[test]
    fn test_norm_zero_vector() {
        let v = Vec3::<f32>::new();
        assert_eq!(v.norm(), 0.0);
    }

    #[test]
    fn test_normalize_happy_path() {
        let v = Vec3::from([3.0, 4.0, 0.0]);
        let normalized = v.normalize();
        assert_eq!(normalized, Vec3::from([0.6, 0.8, 0.0]));

        let v = Vec3::from([1.0, 1.0, 1.0]);
        let normalized = v.normalize();
        assert_eq!(
            normalized,
            Vec3::from([1.0 / SQRT_3, 1.0 / SQRT_3, 1.0 / SQRT_3])
        );

        let v = Vec3::from([0.0, 0.0, 5.0]);
        let normalized = v.normalize();
        assert_eq!(normalized, Vec3::from([0.0, 0.0, 1.0]));
    }

    #[test]
    fn test_normalize_zero_vector() {
        let v = Vec3::<f32>::new();
        let normalized = v.normalize();
        assert_eq!(normalized, Vec3::new());
    }

    #[test]
    fn test_cross_happy_path() {
        let v1 = Vec3::from([1.0, 0.0, 0.0]);
        let v2 = Vec3::from([0.0, 1.0, 0.0]);
        let cross = v1.cross(v2);
        assert_eq!(cross, Vec3::from([0.0, 0.0, 1.0]));

        let v1 = Vec3::from([1.0, 2.0, 3.0]);
        let v2 = Vec3::from([4.0, 5.0, 6.0]);
        let cross = v1.cross(v2);
        assert_eq!(cross, Vec3::from([-3.0, 6.0, -3.0]));
    }

    #[test]
    fn test_cross_parallel_vectors() {
        let v1 = Vec3::from([1.0, 2.0, 3.0]);
        let v2 = Vec3::from([2.0, 4.0, 6.0]);
        let cross = v1.cross(v2);
        assert_eq!(cross, Vec3::new());
    }

    #[test]
    fn test_rotate2_happy_path() {
        let angle = PI / 2.0;
        let matrix = rotate2(angle);
        assert!(matrix.eq_with_epsilon(
            &Mat3::from([[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]]),
            1e-6
        ));
    }

    #[test]
    fn test_rotate2_zero_angle() {
        let angle = 0.0;
        let matrix = rotate2(angle);
        assert_eq!(
            matrix,
            Mat3::from([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
        );
    }

    #[test]
    fn test_rotate2_full_rotation() {
        let angle = 2.0 * PI;
        let matrix = rotate2(angle);
        assert!(matrix.eq_with_epsilon(
            &Mat3::from([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]),
            1e-6
        ));
    }

    #[test]
    fn test_translate2_happy_path() {
        let v = Vec2::from([1.0, 2.0]);
        let matrix = tranlate2(v);
        assert_eq!(
            matrix,
            Mat3::from([[1.0, 0.0, 1.0], [0.0, 1.0, 2.0], [0.0, 0.0, 1.0]])
        );
    }

    #[test]
    fn test_translate2_zero_vector() {
        let v = Vec2::new();
        let matrix = tranlate2(v);
        assert_eq!(
            matrix,
            Mat3::from([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
        );
    }

    #[test]
    fn test_scale2_happy_path() {
        let v = Vec2::from([2.0, 3.0]);
        let matrix = scale2(v);
        assert_eq!(
            matrix,
            Mat3::from([[2.0, 0.0, 0.0], [0.0, 3.0, 0.0], [0.0, 0.0, 1.0]])
        );
    }

    #[test]
    fn test_scale2_zero_vector() {
        let v = Vec2::from([0.0, 0.0]);
        let matrix = scale2(v);
        assert_eq!(
            matrix,
            Mat3::from([[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 1.0]])
        );
    }

    #[test]
    fn test_rotate3_happy_path() {
        let angle = PI / 2.0;
        let axis = Vec3::from([0.0, 0.0, 1.0]);
        let matrix = rotate3(angle, axis);
        assert_eq!(
            matrix,
            Mat4::from([
                [0.0, -1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ])
        );
    }

    #[test]
    fn test_rotate3_zero_angle() {
        let angle = 0.0;
        let axis = Vec3::from([1.0, 0.0, 0.0]);
        let matrix = rotate3(angle, axis);
        assert!(matrix.eq_with_epsilon(&Mat4::I(), 1e-6));
    }

    #[test]
    fn test_rotate3_full_rotation() {
        let angle = 2.0 * PI;
        let axis = Vec3::from([1.0, 0.0, 0.0]);
        let matrix = rotate3(angle, axis);
        assert!(matrix.eq_with_epsilon(&Mat4::I(), 1e-6));
    }

    #[test]
    fn test_rotate3_x_happy_path() {
        let angle = PI / 2.0;
        let matrix = rotate3_x(angle);
        assert!(matrix.eq_with_epsilon(
            &Mat4::from([
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, -1.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]),
            1e-6
        ));
    }

    #[test]
    fn test_rotate3_x_zero_angle() {
        let angle = 0.0;
        let matrix = rotate3_x(angle);
        assert!(matrix.eq_with_epsilon(&Mat4::I(), 1e-6));
    }

    #[test]
    fn test_rotate3_x_full_rotation() {
        let angle = 2.0 * PI;
        let matrix = rotate3_x(angle);
        assert!(matrix.eq_with_epsilon(&Mat4::I(), 1e-6));
    }

    #[test]
    fn test_rotate3_y_happy_path() {
        let angle = PI / 2.0;
        let matrix = rotate3_y(angle);
        assert!(matrix.eq_with_epsilon(
            &Mat4::from([
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]),
            1e-6
        ));
    }

    #[test]
    fn test_rotate3_y_zero_angle() {
        let angle = 0.0;
        let matrix = rotate3_y(angle);
        assert!(matrix.eq_with_epsilon(&Mat4::I(), 1e-6));
    }

    #[test]
    fn test_rotate3_y_full_rotation() {
        let angle = 2.0 * PI;
        let matrix = rotate3_y(angle);
        assert!(matrix.eq_with_epsilon(&Mat4::I(), 1e-6));
    }

    #[test]
    fn test_rotate3_z_happy_path() {
        let angle = PI / 2.0;
        let matrix = rotate3_z(angle);
        assert!(matrix.eq_with_epsilon(
            &Mat4::from([
                [0.0, -1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]),
            1e-6
        ));
    }

    #[test]
    fn test_rotate3_z_zero_angle() {
        let angle = 0.0;
        let matrix = rotate3_z(angle);
        assert!(matrix.eq_with_epsilon(&Mat4::I(), 1e-6));
    }

    #[test]
    fn test_rotate3_z_full_rotation() {
        let angle = 2.0 * PI;
        let matrix = rotate3_z(angle);
        assert!(matrix.eq_with_epsilon(&Mat4::I(), 1e-6));
    }

    #[test]
    fn test_translate3_happy_path() {
        let v = Vec3::from([1.0, 2.0, 3.0]);
        let matrix = tranlate3(v);
        assert_eq!(
            matrix,
            Mat4::from([
                [1.0, 0.0, 0.0, 1.0],
                [0.0, 1.0, 0.0, 2.0],
                [0.0, 0.0, 1.0, 3.0],
                [0.0, 0.0, 0.0, 1.0],
            ])
        );
    }

    #[test]
    fn test_translate3_zero_vector() {
        let v = Vec3::new();
        let matrix = tranlate3(v);
        assert!(matrix.eq_with_epsilon(&Mat4::I(), 1e-6));
    }

    #[test]
    fn test_scale3_happy_path() {
        let v = Vec3::from([2.0, 3.0, 4.0]);
        let matrix = scale3(v);
        assert_eq!(
            matrix,
            Mat4::from([
                [2.0, 0.0, 0.0, 0.0],
                [0.0, 3.0, 0.0, 0.0],
                [0.0, 0.0, 4.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ])
        );
    }

    #[test]
    fn test_scale3_zero_vector() {
        let v = Vec3::from([0.0, 0.0, 0.0]);
        let matrix = scale3(v);
        assert_eq!(
            matrix,
            Mat4::from([
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ])
        );
    }
}
