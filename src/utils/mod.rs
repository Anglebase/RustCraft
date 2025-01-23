/// 设置着色器uniform变量的trait
pub trait SetUniform {
    fn give(&self, location: i32);
}

impl SetUniform for f32 {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform1f(location, *self);
        }
    }
}

impl SetUniform for f64 {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform1d(location, *self);
        }
    }
}

impl SetUniform for i32 {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform1i(location, *self);
        }
    }
}

impl SetUniform for u32 {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform1ui(location, *self);
        }
    }
}

mod mat;
pub mod math;
mod types;
mod uniform_f32;
mod uniform_f64;
mod uniform_i32;
mod uniform_u32;
pub use types::*;

/// 角度制转弧度制
/// 
/// # 参数
/// 
/// - `degrees` 角度制角度
/// 
/// # 返回值
/// 
/// 弧度制角度
/// 
/// # 示例
/// 
/// ```
/// use rustcraft::utils::radian;
/// 
/// let radian = radian(90.0);
/// assert_eq!(radian, std::f32::consts::PI / 2.0);
/// ```
pub fn radian(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

/// 生成摄像机位置变换矩阵
/// 
/// # 参数
/// 
/// - `eye` 摄像机位置
/// - `target` 目标位置
/// - `up` 上向量
/// 
/// # 返回值
/// 
/// 位置变换矩阵
/// 
/// # 示例
/// 
/// ```
/// use rustcraft::utils::look_at;
/// use rustcraft::utils::Vec3;
/// 
/// let eye = Vec3::from([0.0, 0.0, 1.0]);
/// let target = Vec3::from([0.0, 0.0, 0.0]);
/// let up = Vec3::from([0.0, 1.0, 0.0]);
/// let matrix = look_at(eye, target, up);
/// ```
pub fn look_at(eye: Vec3<f32>, target: Vec3<f32>, up: Vec3<f32>) -> Mat4<f32> {
    let z = (eye - target).normalize(); // 计算z轴方向向量
    let x = up.cross(z).normalize(); // 计算x轴方向向量
    let y = z.cross(x); // 计算y轴方向向量

    let translation = [
        [1.0, 0.0, 0.0, -eye.x()],
        [0.0, 1.0, 0.0, -eye.y()],
        [0.0, 0.0, 1.0, -eye.z()],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let rotation = [
        [*x.x(), *x.y(), *x.z(), 0.0],
        [*y.x(), *y.y(), *y.z(), 0.0],
        [*z.x(), *z.y(), *z.z(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];

    Mat4::from(translation) * Mat4::from(rotation)
}

/// 生成透视投影矩阵
/// 
/// # 参数
/// 
/// - `fov` 视角
/// - `aspect` 纵横比
/// - `z_near` 近裁切面
/// - `z_far` 远裁切面
/// 
/// # 返回值
/// 
/// 透视投影矩阵
/// 
/// # 示例
/// 
/// ```
/// use rustcraft::utils::perspective;
/// 
/// let matrix = perspective(45.0, 1.0, 0.1, 100.0);
/// ```
pub fn perspective(fov: f32, aspect: f32, z_near: f32, z_far: f32) -> Mat4<f32> {
    let f = 1.0 / (fov / 2.0).tan();
    let mut result = [[0.0; 4]; 4];

    result[0][0] = f / aspect;
    result[1][1] = f;
    result[2][2] = (z_far + z_near) / (z_near - z_far);
    result[2][3] = (2.0 * z_far * z_near) / (z_near - z_far);
    result[3][2] = -1.0;

    Mat4::from(result)
}
