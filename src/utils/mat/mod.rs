mod mat2;
mod mat3;
mod mat4;
// pub mod mat2x3;
// pub mod mat3x2;
// pub mod mat2x4;
// pub mod mat4x2;
// pub mod mat3x4;
// pub mod mat4x3;

pub use mat2::Mat2;
pub use mat3::Mat3;
pub use mat4::Mat4;

#[macro_export]
macro_rules! impl_mat_ops_add {
    ($type:ty, $rows:expr, $cols:expr) => {
        use std::ops::Add;
        impl<T> Add for $type
        where
            T: Copy + Add<Output = T> + From<f32>,
        {
            type Output = Self;

            fn add(self, other: Self) -> Self::Output {
                let mut result = Self::default();
                for i in 0..$rows {
                    for j in 0..$cols {
                        result.data[i][j] = self.data[i][j] + other.data[i][j];
                    }
                }
                result
            }
        }
    };
}

#[macro_export]
macro_rules! impl_mat_ops_sub {
    ($type:ty, $rows:expr, $cols:expr) => {
        use std::ops::Sub;
        impl<T> Sub for $type
        where
            T: Copy + Sub<Output = T> + From<f32>,
        {
            type Output = Self;

            fn sub(self, other: Self) -> Self::Output {
                let mut result = Self::default();
                for i in 0..$rows {
                    for j in 0..$cols {
                        result.data[i][j] = self.data[i][j] - other.data[i][j];
                    }
                }
                result
            }
        }
    };
}

#[macro_export]
macro_rules! impl_mat_ops_mul_number {
    ($type:ty, $rows:expr, $cols:expr) => {
        use std::ops::Mul;

        impl<T: Copy + Mul<Output = T> + From<f32>> Mul<T> for $type {
            type Output = Self;

            fn mul(self, other: T) -> Self::Output {
                let mut result = Self::default();
                for i in 0..$rows {
                    for j in 0..$cols {
                        result.data[i][j] = self.data[i][j] * other;
                    }
                }
                result
            }
        }
    };
}
