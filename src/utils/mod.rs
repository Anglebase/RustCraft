pub trait SetUniform {
    fn give(&self, location: i32);
}

impl SetUniform for i32 {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform1i(location, *self);
        }
    }
}

impl SetUniform for f32 {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform1f(location, *self);
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
mod vec;
pub use mat::*;
pub use vec::*;

pub fn tranlate2<T: Copy + From<f32>>(x: T, y: T) -> Mat3<T> {
    let mut mat = Mat3::<T>::new();
    mat[0][2] = x;
    mat[1][2] = y;
    mat
}

pub fn scale2<T: Copy + From<f32>>(x: T, y: T) -> Mat3<T> {
    let mut mat = Mat3::<T>::default();
    mat[0][0] = x;
    mat[1][1] = y;
    mat
}

pub fn rotate2(angle: f32) -> Mat3<f32> {
    let mut mat = Mat3::<f32>::default();
    let c: f32 = f32::cos(angle);
    let s: f32 = f32::sin(c);
    mat[0][0] = c;
    mat[0][1] = -s;
    mat[1][0] = s;
    mat[1][1] = c;
    mat
}

pub fn tranlate3<T: Copy + From<f32>>(x: T, y: T, z: T) -> Mat4<T> {
    let mut mat = Mat4::<T>::new();
    mat[0][3] = x;
    mat[1][3] = y;
    mat[2][3] = z;
    mat
}

pub fn scale3<T: Copy + From<f32>>(x: T, y: T, z: T) -> Mat4<T> {
    let mut mat = Mat4::<T>::default();
    mat[0][0] = x;
    mat[1][1] = y;
    mat[2][2] = z;
    mat
}

pub fn rotate3(angle: f32, axis: Vec3<f32>) -> Mat4<f32> {
    let mut result = Mat4::<f32>::new();
    let s = angle.sin();
    let c = angle.cos();
    let nor = axis * (1.0 / axis.length());
    let k_array = [
        [0.0, -nor.z, nor.y],
        [nor.z, 0.0, -nor.x],
        [-nor.y, nor.x, 0.0],
    ];
    let k = Mat3::from(k_array);
    let r = Mat3::new() + k * s + k * k * (1.0 - c);
    for i in 0..3 {
        for j in 0..3 {
            result[i][j] = r[i][j];
        }
    }
    result
}

pub fn radian(angle: f32) -> f32 {
    angle * std::f32::consts::PI / 180.0
}
