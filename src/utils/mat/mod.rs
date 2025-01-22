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
        impl<T> std::ops::Add for $type
        where
            T: Copy + std::ops::Add<Output = T> + From<f32>,
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
        impl<T> std::ops::Sub for $type
        where
            T: Copy + std::ops::Sub<Output = T> + From<f32>,
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
        impl<T: Copy + std::ops::Mul<Output = T> + From<f32>> std::ops::Mul<T> for $type {
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

#[macro_export]
macro_rules! impl_mat_mul_mat {
    ($type:ty, $rows:expr, $cols:expr, $type2:ty, $rows2:expr, $cols2:expr, $retype:ty) => {
        impl<T> std::ops::Mul<$type2> for $type
        where
            T: Copy + From<f32>,
            T: std::ops::Mul<Output = T> + std::ops::AddAssign,
        {
            type Output = Self;

            fn mul(self, other: $type2) -> Self::Output {
                let mut result = <$retype>::default();
                for i in 0..$rows {
                    for j in 0..$cols2 {
                        for k in 0..$cols {
                            result.data[i][j] += self.data[i][k] * other.data[k][j];
                        }
                    }
                }
                result
            }
        }
    };
    ($type:ty, $rows:expr, $cols:expr) => {
        $crate::impl_mat_mul_mat!($type, $rows, $cols, $type, $rows, $cols, $type);
    };
}
