pub use super::mat::Mat;

pub type Vec<T, const N: usize> = Mat<T, 1, N>;

pub type Vec2<T> = Vec<T, 2>;
pub type Vec3<T> = Vec<T, 3>;
pub type Vec4<T> = Vec<T, 4>;

pub type Mat2<T> = Mat<T, 2, 2>;
pub type Mat3<T> = Mat<T, 3, 3>;
pub type Mat4<T> = Mat<T, 4, 4>;

pub type Mat2x3<T> = Mat<T, 2, 3>;
pub type Mat2x4<T> = Mat<T, 2, 4>;
pub type Mat3x2<T> = Mat<T, 3, 2>;
pub type Mat3x4<T> = Mat<T, 3, 4>;
pub type Mat4x2<T> = Mat<T, 4, 2>;
pub type Mat4x3<T> = Mat<T, 4, 3>;

impl<T> Vec2<T> {
    pub fn x(&self) -> &T {
        &self[0][0]
    }
    pub fn y(&self) -> &T {
        &self[0][1]
    }
    pub fn set_x(&mut self, value: T) {
        self[0][0] = value;
    }
    pub fn set_y(&mut self, value: T) {
        self[0][1] = value;
    }
}

impl<T> Vec3<T> {
    pub fn x(&self) -> &T {
        &self[0][0]
    }
    pub fn y(&self) -> &T {
        &self[0][1]
    }
    pub fn z(&self) -> &T {
        &self[0][2]
    }
    pub fn set_x(&mut self, value: T) {
        self[0][0] = value;
    }
    pub fn set_y(&mut self, value: T) {
        self[0][1] = value;
    }
    pub fn set_z(&mut self, value: T) {
        self[0][2] = value;
    }
}

impl<T> Vec4<T> {
    pub fn x(&self) -> &T {
        &self[0][0]
    }
    pub fn y(&self) -> &T {
        &self[0][1]
    }
    pub fn z(&self) -> &T {
        &self[0][2]
    }
    pub fn w(&self) -> &T {
        &self[0][3]
    }
    pub fn set_x(&mut self, value: T) {
        self[0][0] = value;
    }
    pub fn set_y(&mut self, value: T) {
        self[0][1] = value;
    }
    pub fn set_z(&mut self, value: T) {
        self[0][2] = value;
    }
    pub fn set_w(&mut self, value: T) {
        self[0][3] = value;
    }
}

impl<T: Copy> Vec2<T> {
    pub fn xy(&self) -> Self {
        *self
    }
}

impl<T: Copy> Vec3<T> {
    pub fn xyz(&self) -> Self {
        *self
    }
    pub fn xy(&self) -> Vec2<T> {
        Vec2::from([[self[0][0], self[0][1]]])
    }
    pub fn yz(&self) -> Vec2<T> {
        Vec2::from([[self[0][1], self[0][2]]])
    }
}

impl<T: Copy> Vec4<T> {
    pub fn xyzw(&self) -> Self {
        *self
    }
    pub fn xy(&self) -> Vec2<T> {
        Vec2::from([[self[0][0], self[0][1]]])
    }
    pub fn yz(&self) -> Vec2<T> {
        Vec2::from([[self[0][1], self[0][2]]])
    }
    pub fn zw(&self) -> Vec2<T> {
        Vec2::from([[self[0][2], self[0][3]]])
    }
    pub fn xyz(&self) -> Vec3<T> {
        Vec3::from([[self[0][0], self[0][1], self[0][2]]])
    }
    pub fn yzw(&self) -> Vec3<T> {
        Vec3::from([[self[0][1], self[0][2], self[0][3]]])
    }
}

/// (Vec2, T) -> Vec3
impl<T: Copy> From<(Vec2<T>, T)> for Vec3<T> {
    fn from((v, z): (Vec2<T>, T)) -> Self {
        Vec3::from([[v[0][0], v[0][1], z]])
    }
}
/// (T, Vec2) -> Vec3
impl<T: Copy> From<(T, Vec2<T>)> for Vec3<T> {
    fn from((x, v): (T, Vec2<T>)) -> Self {
        Vec3::from([[x, v[0][0], v[0][1]]])
    }
}
/// (Vec2, T, T) -> Vec4
impl<T: Copy> From<(Vec2<T>, T, T)> for Vec4<T> {
    fn from((v, z, w): (Vec2<T>, T, T)) -> Self {
        Vec4::from([[v[0][0], v[0][1], z, w]])
    }
}
/// (T, Vec2, T) -> Vec4
impl<T: Copy> From<(T, Vec2<T>, T)> for Vec4<T> {
    fn from((x, v, w): (T, Vec2<T>, T)) -> Self {
        Vec4::from([[x, v[0][0], v[0][1], w]])
    }
}
/// (T, T, Vec2) -> Vec4
impl<T: Copy> From<(T, T, Vec2<T>)> for Vec4<T> {
    fn from((x, y, v): (T, T, Vec2<T>)) -> Self {
        Vec4::from([[x, y, v[0][0], v[0][1]]])
    }
}
/// (Vec2, Vec2) -> Vec4
impl<T: Copy> From<(Vec2<T>, Vec2<T>)> for Vec4<T> {
    fn from((v1, v2): (Vec2<T>, Vec2<T>)) -> Self {
        Vec4::from([[v1[0][0], v1[0][1], v2[0][0], v2[0][1]]])
    }
}
/// (Vec3, T) -> Vec4
impl<T: Copy> From<(Vec3<T>, T)> for Vec4<T> {
    fn from((v, w): (Vec3<T>, T)) -> Self {
        Vec4::from([[v[0][0], v[0][1], v[0][2], w]])
    }
}
/// (T, Vec3) -> Vec4
impl<T: Copy> From<(T, Vec3<T>)> for Vec4<T> {
    fn from((x, v): (T, Vec3<T>)) -> Self {
        Vec4::from([[x, v[0][0], v[0][1], v[0][2]]])
    }
}
/// (T, T) -> Vec2
impl From<(i32, i32)> for Vec2<i32> {
    fn from((x, y): (i32, i32)) -> Self {
        Vec2::from([[x, y]])
    }
}
/// (T, T, T) -> Vec3
impl From<(i32, i32, i32)> for Vec3<i32> {
    fn from((x, y, z): (i32, i32, i32)) -> Self {
        Vec3::from([[x, y, z]])
    }
}
/// (T, T, T, T) -> Vec4
impl<T: Copy> From<(T, T, T, T)> for Vec4<T> {
    fn from((x, y, z, w): (T, T, T, T)) -> Self {
        Vec4::from([[x, y, z, w]])
    }
}
/// [T;2] -> Vec2
impl<T: Copy> From<[T; 2]> for Vec2<T> {
    fn from(value: [T; 2]) -> Self {
        Vec2::from([[value[0], value[1]]])
    }
}
/// [T;3] -> Vec3
impl<T: Copy> From<[T; 3]> for Vec3<T> {
    fn from(value: [T; 3]) -> Self {
        Vec3::from([[value[0], value[1], value[2]]])
    }
}
/// [T;4] -> Vec4
impl<T: Copy> From<[T; 4]> for Vec4<T> {
    fn from(value: [T; 4]) -> Self {
        Vec4::from([[value[0], value[1], value[2], value[3]]])
    }
}
