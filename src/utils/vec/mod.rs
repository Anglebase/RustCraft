mod vec2;
mod vec3;
mod vec4;

pub use vec2::Vec2;
pub use vec3::Vec3;
pub use vec4::Vec4;

#[macro_export]
macro_rules! impl_vec_ops_add {
    ($type:ty, $($field:ident),+) => {
        use std::ops::Add;
        impl<T: Add<Output = T> + Copy> Add for $type {
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
        use std::ops::Sub;
        impl<T: Sub<Output = T> + Copy> Sub for $type {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                Self {
                    $($field: self.$field - other.$field,)+
                }
            }
        }
    };
}
