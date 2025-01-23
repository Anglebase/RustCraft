mod mat2;
mod mat2x3;
mod mat2x4;
mod mat3;
mod mat3x2;
mod mat3x4;
mod mat4;
mod mat4x2;
mod mat4x3;

pub use mat2::Mat2;
pub use mat2x3::Mat2x3;
pub use mat2x4::Mat2x4;
pub use mat3::Mat3;
pub use mat3x2::Mat3x2;
pub use mat3x4::Mat3x4;
pub use mat4::Mat4;
pub use mat4x2::Mat4x2;
pub use mat4x3::Mat4x3;

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
            type Output = $retype;

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

#[macro_export]
macro_rules! impl_mat_index {
    ($type:ty, $rows:expr, $cols:expr) => {
        impl<T> std::ops::Index<usize> for $type {
            type Output = [T; $cols];

            fn index(&self, index: usize) -> &Self::Output {
                &self.data[index]
            }
        }

        impl<T> std::ops::IndexMut<usize> for $type {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.data[index]
            }
        }
    };
}

#[macro_export]
macro_rules! impl_mat_ops_div_number {
    ($type:ty, $rows:expr, $cols:expr) => {
        impl<T: Copy + std::ops::Div<Output = T> + From<f32>> std::ops::Div<T> for $type {
            type Output = Self;

            fn div(self, other: T) -> Self::Output {
                let mut result = Self::default();
                for i in 0..$rows {
                    for j in 0..$cols {
                        result.data[i][j] = self.data[i][j] / other;
                    }
                }
                result
            }
        }
    };
}

#[macro_export]
macro_rules! impl_mat_ops_neg {
    ($type:ty, $rows:expr, $cols:expr) => {
        impl<T: Copy + std::ops::Neg<Output = T> + From<f32>> std::ops::Neg for $type {
            type Output = Self;

            fn neg(self) -> Self::Output {
                let mut result = Self::default();
                for i in 0..$rows {
                    for j in 0..$cols {
                        result.data[i][j] = -self.data[i][j];
                    }
                }
                result
            }
        }
    };
}

#[macro_export]
macro_rules! impl_mat_ops_add_assign {
    ($type:ty, $rows:expr, $cols:expr) => {
        impl<T: Copy + std::ops::AddAssign + From<f32>> std::ops::AddAssign for $type {
            fn add_assign(&mut self, other: Self) {
                for i in 0..$rows {
                    for j in 0..$cols {
                        self.data[i][j] += other.data[i][j];
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_mat_ops_sub_assign {
    ($type:ty, $rows:expr, $cols:expr) => {
        impl<T: Copy + std::ops::SubAssign + From<f32>> std::ops::SubAssign for $type {
            fn sub_assign(&mut self, other: Self) {
                for i in 0..$rows {
                    for j in 0..$cols {
                        self.data[i][j] -= other.data[i][j];
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_mat_ops_mul_assign_number {
    ($type:ty, $rows:expr, $cols:expr) => {
        impl<T: Copy + std::ops::MulAssign + From<f32>> std::ops::MulAssign<T> for $type {
            fn mul_assign(&mut self, other: T) {
                for i in 0..$rows {
                    for j in 0..$cols {
                        self.data[i][j] *= other;
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_mat_ops_div_assign_number {
    ($type:ty, $rows:expr, $cols:expr) => {
        impl<T: Copy + std::ops::DivAssign + From<f32>> std::ops::DivAssign<T> for $type {
            fn div_assign(&mut self, other: T) {
                for i in 0..$rows {
                    for j in 0..$cols {
                        self.data[i][j] /= other;
                    }
                }
            }
        }
    };
}

impl_mat_mul_mat!(Mat2x3<T>, 2, 3, Mat3x2<T>, 3, 2, Mat2<T>);
impl_mat_mul_mat!(Mat3x4<T>, 3, 4, Mat4x3<T>, 4, 3, Mat3<T>);
impl_mat_mul_mat!(Mat2x4<T>, 2, 4, Mat4x2<T>, 4, 2, Mat2<T>);
impl_mat_mul_mat!(Mat4x3<T>, 4, 3, Mat3x4<T>, 3, 4, Mat4<T>);
impl_mat_mul_mat!(Mat3x2<T>, 3, 2, Mat2x3<T>, 2, 3, Mat3<T>);
impl_mat_mul_mat!(Mat4x2<T>, 4, 2, Mat2x4<T>, 2, 4, Mat4<T>);
impl_mat_mul_mat!(Mat2x3<T>, 2, 3, Mat3<T>, 3, 1, Mat2x3<T>);
impl_mat_mul_mat!(Mat3x4<T>, 3, 4, Mat4<T>, 4, 1, Mat3x4<T>);
impl_mat_mul_mat!(Mat2x4<T>, 2, 4, Mat4<T>, 4, 1, Mat2x4<T>);
impl_mat_mul_mat!(Mat4x3<T>, 4, 3, Mat3<T>, 3, 1, Mat4x3<T>);
impl_mat_mul_mat!(Mat3x2<T>, 3, 2, Mat2<T>, 2, 1, Mat3x2<T>);
impl_mat_mul_mat!(Mat4x2<T>, 4, 2, Mat2<T>, 2, 1, Mat4x2<T>);
impl_mat_mul_mat!(Mat2x3<T>, 2, 3, Mat3x4<T>, 4, 3, Mat2x4<T>);
impl_mat_mul_mat!(Mat3x4<T>, 3, 4, Mat4x2<T>, 2, 4, Mat3x2<T>);
impl_mat_mul_mat!(Mat2x4<T>, 2, 4, Mat4x3<T>, 3, 4, Mat2x3<T>);
impl_mat_mul_mat!(Mat4x3<T>, 4, 3, Mat3x2<T>, 2, 3, Mat4x2<T>);
impl_mat_mul_mat!(Mat3x2<T>, 3, 2, Mat2x4<T>, 4, 2, Mat3x4<T>);
impl_mat_mul_mat!(Mat4x2<T>, 4, 2, Mat2x3<T>, 3, 2, Mat4x3<T>);
impl_mat_mul_mat!(Mat2x3<T>, 2, 3, Mat4x3<T>, 4, 3, Mat2x4<T>);
impl_mat_mul_mat!(Mat3x4<T>, 3, 4, Mat2x4<T>, 2, 4, Mat3x2<T>);
impl_mat_mul_mat!(Mat2x4<T>, 2, 4, Mat3x4<T>, 3, 4, Mat2x3<T>);
impl_mat_mul_mat!(Mat4x3<T>, 4, 3, Mat2x3<T>, 2, 3, Mat4x2<T>);
impl_mat_mul_mat!(Mat3x2<T>, 3, 2, Mat4x2<T>, 4, 2, Mat3x4<T>);
impl_mat_mul_mat!(Mat4x2<T>, 4, 2, Mat3x2<T>, 3, 2, Mat4x3<T>);
impl_mat_mul_mat!(Mat2x3<T>, 2, 3, Mat4x2<T>, 4, 2, Mat2x4<T>);
impl_mat_mul_mat!(Mat3x4<T>, 3, 4, Mat2x3<T>, 2, 3, Mat3x2<T>);
impl_mat_mul_mat!(Mat4x3<T>, 4, 3, Mat2x4<T>, 2, 4, Mat4x2<T>);
impl_mat_mul_mat!(Mat3x2<T>, 3, 2, Mat4x3<T>, 4, 3, Mat3x4<T>);
