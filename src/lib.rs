use std::sync::Mutex;

pub mod app;
pub(crate) mod gl_utils;
pub mod log;
pub mod model;
pub mod shader;
pub mod texture;
pub mod utils;

pub struct RustCraftWrapper<T> {
    data: Mutex<T>,
}

impl<T> RustCraftWrapper<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: Mutex::new(data),
        }
    }

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
