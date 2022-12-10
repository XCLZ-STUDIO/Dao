#[derive(Copy, Clone, Debug)]
pub struct Point<T, const DIM: usize> {
    dims: [T; DIM],
}

impl<T, const DIM: usize> Point<T, DIM> {
    pub fn new(dims: [T; DIM]) -> Self {
        Self { dims }
    }

    pub fn get(&self, i: usize) -> &T {
        &self.dims[i]
    }
}