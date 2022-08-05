use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Sub};

use ndarray::{Array3, ArrayBase, Dim, OwnedRepr, ScalarOperand};
use num_traits::identities::Zero;

use crate::mesh::Dimensions;
use crate::vector::Vec3;

#[derive(Clone, Debug)]
pub struct Field<T: Copy + Clone + Zero + Mul<f64> + AddAssign<<T as Mul<f64>>::Output>> {
    data: Array3<T>,
    shape: (usize, usize, usize),
}

impl<T: Copy + Clone + Zero + Mul<f64> + AddAssign<<T as Mul<f64>>::Output>> Field<T> {
    pub fn new(dimensions: Dimensions) -> Self {
        let shape: (usize, usize, usize) = dimensions.into();
        let data = Array3::<T>::zeros(shape);

        Field {
            data: data,
            shape: shape,
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.shape.0 {
            for j in 0..self.shape.1 {
                for k in 0..self.shape.2 {
                    self.data[[i, j, k]].set_zero();
                }
            }
        }
    }

    pub fn scatter(&mut self, logical_coordinate: Vec3, value: T) {
        let lc = logical_coordinate;

        if lc.x < 0.0
            || lc.x >= (self.shape.0 - 1) as f64
            || lc.y < 0.0
            || lc.y >= (self.shape.1 - 1) as f64
            || lc.z < 0.0
            || lc.z >= (self.shape.2 - 1) as f64
        {
            return;
        }

        let i = lc.x as usize;
        let j = lc.y as usize;
        let k = lc.z as usize;

        let di = lc.x - i as f64;
        let dj = lc.y - j as f64;
        let dk = lc.z - k as f64;

        self.data[[i, j, k]] += value * ((1.0 - di) * (1.0 - dj) * (1.0 - dk));
        self.data[[i + 1, j, k]] += value * (di * (1.0 - dj) * (1.0 - dk));
        self.data[[i + 1, j + 1, k]] += value * (di * dj * (1.0 - dk));
        self.data[[i, j + 1, k]] += value * ((1.0 - di) * dj * (1.0 - dk));
        self.data[[i, j, k + 1]] += value * ((1.0 - di) * (1.0 - dj) * dk);
        self.data[[i + 1, j, k + 1]] += value * (di * (1.0 - dj) * dk);
        self.data[[i + 1, j + 1, k + 1]] += value * (di * dj * dk);
        self.data[[i, j + 1, k + 1]] += value * ((1.0 - di) * dj * dk);
    }
}

impl<T: Copy + Clone + Zero + Mul<f64> + AddAssign<<T as Mul<f64>>::Output>> AddAssign
    for Field<T>
{
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            data: self.data.clone() + other.data.clone(),
            shape: self.shape,
        }
    }
}

impl<
        T: Copy + Clone + Zero + Mul<f64> + Div + Div<Output = T> + AddAssign<<T as Mul<f64>>::Output>,
    > Div for Field<T>
{
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        let mut new_field = Field {
            data: Array3::<T>::zeros(self.shape),
            shape: self.shape,
        };

        for i in 0..self.shape.0 {
            for j in 0..self.shape.1 {
                for k in 0..self.shape.2 {
                    new_field[[i, j, k]] = self[[i, j, k]] / other[[i, j, k]];
                }
            }
        }

        new_field
    }
}

impl<T: Copy + Clone + Zero + Mul<f64> + AddAssign<<T as Mul<f64>>::Output>> Index<[usize; 3]>
    for Field<T>
{
    type Output = T;

    fn index(&self, indices: [usize; 3]) -> &Self::Output {
        &self.data[indices]
    }
}

impl<T: Copy + Clone + Zero + Mul<f64> + AddAssign<<T as Mul<f64>>::Output>> IndexMut<[usize; 3]>
    for Field<T>
{
    fn index_mut(&mut self, indices: [usize; 3]) -> &mut Self::Output {
        &mut self.data[indices]
    }
}

impl<
        T: Copy + Clone + Zero + Add<U, Output = T> + Mul<f64> + AddAssign<<T as Mul<f64>>::Output>,
        U: ScalarOperand,
    > Add<U> for Field<T>
{
    type Output = Self;

    fn add(self, addend: U) -> Self {
        let data = self.data + addend;

        Self {
            data: data,
            shape: self.shape,
        }
    }
}

impl<
        T: Copy + Clone + Zero + Sub<U, Output = T> + Mul<f64> + AddAssign<<T as Mul<f64>>::Output>,
        U: ScalarOperand,
    > Sub<U> for Field<T>
{
    type Output = Self;

    fn sub(self, subtrahend: U) -> Self {
        let data = self.data - subtrahend;

        Self {
            data: data,
            shape: self.shape,
        }
    }
}

impl<
        T: Copy + Clone + Zero + Mul<U, Output = T> + Mul<f64> + AddAssign<<T as Mul<f64>>::Output>,
        U: ScalarOperand,
    > Mul<U> for Field<T>
{
    type Output = Self;

    fn mul(self, multiplier: U) -> Self {
        let data = self.data * multiplier;

        Self {
            data: data,
            shape: self.shape,
        }
    }
}

impl<
        T: Copy + Clone + Zero + Div<U, Output = T> + Mul<f64> + AddAssign<<T as Mul<f64>>::Output>,
        U: ScalarOperand,
    > Div<U> for Field<T>
{
    type Output = Self;

    fn div(self, divisor: U) -> Self {
        let data = self.data / divisor;

        Self {
            data: data,
            shape: self.shape,
        }
    }
}
