use crate::{utils::SetUniform, *};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T: Copy + Into<f32>> Vec3<T> {
    pub fn length(&self) -> f32 {
        (*self * *self).sqrt().into()
    }

    pub fn normalize(&self) -> Vec3<f32> {
        let len = self.length();
        Vec3 {
            x: (self.x.into() / len).into(),
            y: (self.y.into() / len).into(),
            z: (self.z.into() / len).into(),
        }
    }

    pub fn cross(&self, other: Vec3<T>) -> Vec3<f32> {
        Vec3 {
            x: (self.y.into() * other.z.into() - self.z.into() * other.y.into()).into(),
            y: (self.z.into() * other.x.into() - self.x.into() * other.z.into()).into(),
            z: (self.x.into() * other.y.into() - self.y.into() * other.x.into()).into(),
        }
    }
}

impl SetUniform for Vec3<f32> {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform3f(location, self.x, self.y, self.z);
        }
    }
}

impl SetUniform for Vec3<i32> {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform3i(location, self.x, self.y, self.z);
        }
    }
}

impl SetUniform for Vec3<u32> {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform3ui(location, self.x, self.y, self.z);
        }
    }
}

impl_vec_ops_add!(Vec3<T>, x, y, z);
impl_vec_ops_sub!(Vec3<T>, x, y, z);
impl_vec_ops_mul_number!(Vec3<T>, x, y, z);
impl_vec_ops_mul_vec!(Vec3<T>, x, y, z);
impl_vec_ops_div_number!(Vec3<T>, x, y, z);
impl_vec_ops_neg!(Vec3<T>, x, y, z);
impl_vec_ops_add_assign!(Vec3<T>, x, y, z);
impl_vec_ops_sub_assign!(Vec3<T>, x, y, z);
impl_vec_ops_mul_assign_number!(Vec3<T>, x, y, z);
impl_vec_ops_div_assign_number!(Vec3<T>, x, y, z);
