use std::ops::{Deref, DerefMut};

#[derive(Copy, Clone, Debug)]
pub struct Point<T, const DIM: usize> {
    dims: [T; DIM],
}

impl<T, const DIM: usize> Point<T, DIM> {
    pub fn new(dims: [T; DIM]) -> Self {
        Self { dims }
    }
}

impl<T, const DIM: usize> Deref for Point<T, DIM> {
    type Target = [T; DIM];

    fn deref(&self) -> &Self::Target {
        &self.dims
    }
}

impl<T, const DIM: usize> DerefMut for Point<T, DIM> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dims
    }
}
