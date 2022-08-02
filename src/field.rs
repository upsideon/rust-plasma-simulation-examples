use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};

use ndarray::{Array3, ScalarOperand};
use num_traits::identities::Zero;

use crate::mesh::Dimensions;

#[derive(Debug)]
pub struct Field<T: Clone + Zero> {
    data: Array3<T>,
}

impl<T: Clone + Zero> Index<[usize; 3]> for Field<T> {
    type Output = T;

    fn index(&self, indices: [usize; 3]) -> &Self::Output {
        &self.data[indices]
    }
}

impl<T: Clone + Zero> IndexMut<[usize; 3]> for Field<T> {
    fn index_mut(&mut self, indices: [usize; 3]) -> &mut Self::Output {
        &mut self.data[indices]
    }
}

impl<T: Clone + Zero + Add<U, Output = T>, U: ScalarOperand> Add<U> for Field<T> {
    type Output = Self;

    fn add(self, addend: U) -> Self {
        let data = self.data + addend;

        Self { data: data }
    }
}

impl<T: Clone + Zero + Sub<U, Output = T>, U: ScalarOperand> Sub<U> for Field<T> {
    type Output = Self;

    fn sub(self, subtrahend: U) -> Self {
        let data = self.data - subtrahend;

        Self { data: data }
    }
}

impl<T: Clone + Zero + Mul<U, Output = T>, U: ScalarOperand> Mul<U> for Field<T> {
    type Output = Self;

    fn mul(self, multiplier: U) -> Self {
        let data = self.data * multiplier;

        Self { data: data }
    }
}

impl<T: Clone + Zero + Div<U, Output = T>, U: ScalarOperand> Div<U> for Field<T> {
    type Output = Self;

    fn div(self, divisor: U) -> Self {
        let data = self.data / divisor;

        Self { data: data }
    }
}

impl<T: Clone + Zero> Field<T> {
    pub fn new(dimensions: Dimensions) -> Self {
        let shape: (usize, usize, usize) = dimensions.into();
        let data = Array3::<T>::zeros(shape);

        Field { data: data }
    }
}
