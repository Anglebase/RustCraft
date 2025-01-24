use std::ops::{AddAssign, Div, Mul, Sub};

use super::*;

impl<T, const M: usize> Vec<T, M>
where
    T: Copy + Default + Into<f32>,
    T: AddAssign + Mul<Output = T>,
{
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
