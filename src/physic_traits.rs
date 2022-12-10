use num::Float;
use crate::entities::Particle;
use crate::utils::Point;

#[derive(Copy, Clone, Debug)]
pub struct Intensity<T: Float, const DIM: usize> {
    dims: [T; DIM],
}

impl<T: Float, const DIM: usize> Intensity<T, DIM> {
    pub fn new(dims: [T; DIM]) -> Self {
        Self { dims }
    }

    pub fn get(&self, i: usize) -> &T {
        &self.dims[i]
    }
}

pub trait Posable<T: Float, const DIM: usize> {
    fn position(&self) -> &Point<T, DIM>;
}

pub trait Magable<T: Float, const DIM: usize> {
    fn mag_intensity(&self, p: Point<T, DIM>) -> Intensity<T, DIM>;
}

pub trait Electrable<T: Float, const DIM: usize> {
    fn ele_intensity(&self, p: Point<T, DIM>) -> Intensity<T, DIM>;
}

pub trait Massable<T: Float, const DIM: usize> {
    fn mas_intensity(&self, p: Point<T, DIM>) -> T;
}

pub trait Fieldable<T: Float, const DIM: usize> {
    fn mas_intensity(&self, p: Point<T, DIM>) -> Intensity<T, DIM>;
}

pub trait Movable<T: Float, const DIM: usize>: Posable<T, DIM> {
    fn velocity(&self) -> &Intensity<T, DIM>;
    fn acceleration(&self) -> &Intensity<T, DIM>;
}

pub trait Interoperable<T: Float, TIME, const DIM: usize> {
    fn interact(&mut self, obj: &Particle<T, DIM>);
    fn update(&mut self, t: TIME);
}
