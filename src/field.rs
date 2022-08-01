use std::ops;

use ndarray::prelude::*;
use ndarray::{Array3, Dim};
use num_traits::identities::Zero;

use crate::mesh::Dimensions;

#[derive(Debug)]
pub struct Field<T: Clone + Zero> {
    data: Array3<T>,
}

impl<T: Clone + Zero> ops::Index<[usize; 3]> for Field<T> {
    type Output = T;

    fn index(&self, indices: [usize; 3]) -> &Self::Output {
        &self.data[indices]
    }
}

impl<T: Clone + Zero> Field<T> {
    pub fn new(dimensions: Dimensions) -> Self {
        let shape: (usize, usize, usize) = dimensions.into();
        let data = Array3::<T>::zeros(shape);

        Field { data: data }
    }
}
