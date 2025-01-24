use std::sync::Mutex;

pub use {gl, glfw, image, json};

mod app;
pub mod camera;
pub(crate) mod gl_utils;
pub mod log;
mod model;
mod shader;
mod texture;

pub use app::{App, AppBuilder, TimeType};
pub use camera::CAMERA_SYSTEM;
pub use model::MODEL_MANAGER;
pub use shader::SHADER_MANAGER;
pub use texture::TEXTURE_MANAGER;

/// 全局对象包装器
/// 它利用 Mutex 的内部可变性实现全局对象的简洁访问
pub struct RustCraftWrapper<T> {
    data: Mutex<T>,
}

impl<T> RustCraftWrapper<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: Mutex::new(data),
        }
    }

    /// 向内部数据应用一个函数
    /// 该函数接收内部数据的可变引用，允许对其进行修改
    ///
    /// # 参数 Parameters
    /// - `f`: 一个函数，接收内部数据的可变引用作为参数
    ///
    /// # 注解 Note
    /// 在运行函数的过程中，互斥锁处于锁定状态，直至函数执行完毕；
    /// 在函数内不应再次调用此函数，否则会导致死锁
    ///
    /// # 示例 Examples
    /// ```
    /// use rustcraft::RustCraftWrapper;
    /// use lazy_static::lazy_static;
    ///
    /// lazy_static! {
    ///     static ref RUSTCRAFT: RustCraftWrapper<i32> = RustCraftWrapper::new(42);
    /// }
    ///
    /// fn main() {
    ///     RUSTCRAFT.apply(|data| {
    ///         *data += 1;
    ///     });
    ///     RUSTCRAFT.apply(|data| {
    ///         assert_eq!(*data, 43);
    ///     });
    /// }
    /// ```
    pub fn apply<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
    {
        match self.data.lock() {
            Ok(mut data) => f(&mut data),
            Err(e) => println!("Error: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref RUSTCRAFT: RustCraftWrapper<i32> = RustCraftWrapper::new(42);
    }

    #[test]
    fn test_rustcraft_wrapper() {
        RUSTCRAFT.apply(|data| {
            *data += 1;
        });
        assert_eq!(*RUSTCRAFT.data.lock().unwrap(), 43);
    }

    #[test]
    fn test_not_static() {
        let mut s = 0;
        RUSTCRAFT.apply(|data| {
            s = *data;
        });
        assert_eq!(s, 42);
    }
}
