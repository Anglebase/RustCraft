/// 一个 M 行 N 列的行主序矩阵
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Mat<T, const M: usize, const N: usize> {
    data: [[T; N]; M],
}

use std::fmt::Debug;
impl<T, const M: usize, const N: usize> Debug for Mat<T, M, N>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Mat<{:?}, {:?}> {{", M, N)?;
        for i in 0..M {
            write!(f, "| ")?;
            for j in 0..N {
                write!(f, "{:^9?}", self.data[i][j])?;
            }
            writeln!(f, " |")?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<T, const M: usize, const N: usize> From<[[T; N]; M]> for Mat<T, M, N> {
    fn from(value: [[T; N]; M]) -> Self {
        Self { data: value }
    }
}

impl<T, const M: usize, const N: usize> Into<[[T; N]; M]> for Mat<T, M, N> {
    fn into(self) -> [[T; N]; M] {
        self.data
    }
}

impl<T, const M: usize, const N: usize> Mat<T, M, N>
where
    T: Default + Copy,
{
    /// 创建一个 M 行 N 列的零矩阵
    pub fn new() -> Self {
        Self {
            data: [[T::default(); N]; M],
        }
    }
}

impl<T, const M: usize, const N: usize> Default for Mat<T, M, N>
where
    T: Copy + Default,
{
    fn default() -> Self {
        Self {
            data: [[T::default(); N]; M],
        }
    }
}

use std::ops::Add;
impl<T, const M: usize, const N: usize> Add for Mat<T, M, N>
where
    T: Copy + Add<Output = T> + Default,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Self::new();
        for i in 0..M {
            for j in 0..N {
                result.data[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }
        result
    }
}

use std::ops::Sub;
impl<T, const M: usize, const N: usize> Sub for Mat<T, M, N>
where
    T: Copy + Sub<Output = T> + Default,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Self::new();
        for i in 0..M {
            for j in 0..N {
                result.data[i][j] = self.data[i][j] - rhs.data[i][j];
            }
        }
        result
    }
}

use std::ops::Mul;
impl<T, const M: usize, const N: usize> Mul<T> for Mat<T, M, N>
where
    T: Copy + Mul<Output = T> + Default,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let mut result = Self::new();
        for i in 0..M {
            for j in 0..N {
                result.data[i][j] = self.data[i][j] * rhs;
            }
        }
        result
    }
}

use std::ops::Div;
impl<T, const M: usize, const N: usize> Div<T> for Mat<T, M, N>
where
    T: Copy + Div<Output = T> + Default,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let mut result = Self::new();
        for i in 0..M {
            for j in 0..N {
                result.data[i][j] = self.data[i][j] / rhs;
            }
        }
        result
    }
}

use std::ops::AddAssign;
impl<T, const M: usize, const N: usize> AddAssign for Mat<T, M, N>
where
    T: Copy + Default + AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..M {
            for j in 0..N {
                self.data[i][j] += rhs.data[i][j];
            }
        }
    }
}

use std::ops::SubAssign;
impl<T, const M: usize, const N: usize> SubAssign for Mat<T, M, N>
where
    T: Copy + Default + SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..M {
            for j in 0..N {
                self.data[i][j] -= rhs.data[i][j];
            }
        }
    }
}

use std::ops::MulAssign;
impl<T, const M: usize, const N: usize> MulAssign<T> for Mat<T, M, N>
where
    T: Copy + Default + MulAssign,
{
    fn mul_assign(&mut self, rhs: T) {
        for i in 0..M {
            for j in 0..N {
                self.data[i][j] *= rhs;
            }
        }
    }
}

use std::ops::DivAssign;
impl<T, const M: usize, const N: usize> DivAssign<T> for Mat<T, M, N>
where
    T: Copy + Default + DivAssign,
{
    fn div_assign(&mut self, rhs: T) {
        for i in 0..M {
            for j in 0..N {
                self.data[i][j] /= rhs;
            }
        }
    }
}

use std::ops::Index;
impl<T, const M: usize, const N: usize> Index<usize> for Mat<T, M, N> {
    type Output = [T; N];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

use std::ops::IndexMut;
impl<T, const M: usize, const N: usize> IndexMut<usize> for Mat<T, M, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

use std::ops::Neg;
impl<T, const M: usize, const N: usize> Neg for Mat<T, M, N>
where
    T: Copy + Neg<Output = T> + Default,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut result = Self::new();
        for i in 0..M {
            for j in 0..N {
                result.data[i][j] = -self.data[i][j];
            }
        }
        result
    }
}

impl<T, const M: usize, const K: usize, const N: usize> Mul<Mat<T, K, N>> for Mat<T, M, K>
where
    T: Copy + Default,
    T: Mul<Output = T> + AddAssign,
{
    type Output = Mat<T, M, N>;

    fn mul(self, rhs: Mat<T, K, N>) -> Self::Output {
        let mut result = Mat::<T, M, N>::new();
        for i in 0..M {
            for j in 0..N {
                for k in 0..K {
                    result.data[i][j] += self.data[i][k] * rhs.data[k][j];
                }
            }
        }
        result
    }
}

impl<T, const M: usize, const N: usize> Mat<T, M, N>
where
    T: Copy + Default,
{
    /// 生成当前矩阵的转置矩阵
    #[allow(non_snake_case)]
    pub fn T(&self) -> Mat<T, N, M> {
        let mut result = Mat::<T, N, M>::new();
        for i in 0..M {
            for j in 0..N {
                result.data[j][i] = self.data[i][j];
            }
        }
        result
    }

    pub fn transpose(&self) -> Mat<T, N, M> {
        self.T()
    }
}

impl<T, const M: usize> Mat<T, M, M>
where
    T: Copy + From<f32>,
{
    /// 生成单位矩阵
    #[allow(non_snake_case)]
    pub fn I() -> Self {
        let mut result = Self {
            data: [[0.0.into(); M]; M],
        };
        for i in 0..M {
            result.data[i][i] = T::from(1.0);
        }
        result
    }

    pub fn identity() -> Self {
        Self::I()
    }
}

impl<T, const M: usize, const N: usize> Mat<T, M, N> {
    /// 返回矩阵的行数
    pub fn rows(&self) -> usize {
        M
    }
    /// 返回矩阵的列数
    pub fn cols(&self) -> usize {
        N
    }
    /// 返回矩阵的元素个数
    pub fn count(&self) -> usize {
        M * N
    }
}
