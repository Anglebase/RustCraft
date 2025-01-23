mod vec2;
mod vec3;
mod vec4;

pub use vec2::Vec2;
pub use vec3::Vec3;
pub use vec4::Vec4;

#[macro_export]
macro_rules! impl_vec_ops_add {
    ($type:ty, $($field:ident),+) => {
        impl<T: std::ops::Add<Output = T> + Copy> std::ops::Add for $type {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                Self {
                    $($field: self.$field + other.$field,)+
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_vec_ops_sub {
    ($type:ty, $($field:ident),+) => {
        impl<T: std::ops::Sub<Output = T> + Copy> std::ops::Sub for $type {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                Self {
                    $($field: self.$field - other.$field,)+
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_vec_ops_mul_number {
    ($type:ty, $($field:ident),+) => {
        impl<T: std::ops::Mul<Output = T> + Copy> std::ops::Mul<T> for $type {
            type Output = Self;

            fn mul(self, other: T) -> Self {
                Self {
                    $($field: self.$field * other,)+
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_vec_ops_mul_vec {
    ($type:ty, $($field:ident),+) => {
        impl<T> std::ops::Mul for $type
        where
            T: Copy + Into<f32>,
        {
            type Output = f32;

            fn mul(self, other: Self) -> Self::Output {
                let mut result = 0.0;
                $(
                    result += self.$field.into() * other.$field.into();
                )+
                result
            }
        }
    };
}

#[macro_export]
macro_rules! impl_vec_ops_div_number {
    ($type:ty, $($field:ident),+) => {
        impl<T: std::ops::Div<Output = T> + Copy> std::ops::Div<T> for $type {
            type Output = Self;

            fn div(self, other: T) -> Self {
                Self {
                    $($field: self.$field / other,)+
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_vec_ops_neg {
    ($type:ty, $($field:ident),+) => {
        impl<T: std::ops::Neg<Output = T> + Copy> std::ops::Neg for $type {
            type Output = Self;

            fn neg(self) -> Self {
                Self {
                    $($field: -self.$field,)+
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_vec_ops_add_assign {
    ($type:ty, $($field:ident),+) => {
        impl<T: std::ops::AddAssign + Copy> std::ops::AddAssign for $type {
            fn add_assign(&mut self, other: Self) {
                $(
                    self.$field += other.$field;
                )+
            }
        }
    };
}

#[macro_export]
macro_rules! impl_vec_ops_sub_assign {
    ($type:ty, $($field:ident),+) => {
        impl<T: std::ops::SubAssign + Copy> std::ops::SubAssign for $type {
            fn sub_assign(&mut self, other: Self) {
                $(
                    self.$field -= other.$field;
                )+
            }
        }
    };
}

#[macro_export]
macro_rules! impl_vec_ops_mul_assign_number {
    ($type:ty, $($field:ident),+) => {
        impl<T: std::ops::MulAssign + Copy> std::ops::MulAssign<T> for $type {
            fn mul_assign(&mut self, other: T) {
                $(
                    self.$field *= other;
                )+
            }
        }
    };
}

#[macro_export]
macro_rules! impl_vec_ops_div_assign_number {
    ($type:ty, $($field:ident),+) => {
        impl<T: std::ops::DivAssign + Copy> std::ops::DivAssign<T> for $type {
            fn div_assign(&mut self, other: T) {
                $(
                    self.$field /= other;
                )+
            }
        }
    };
}