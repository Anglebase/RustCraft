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
