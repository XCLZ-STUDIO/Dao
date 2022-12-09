#[derive(Copy, Clone)]
pub struct Point<T, const DIM: usize> {
    dims: [T; DIM],
}

#[derive(Copy, Clone)]
pub struct Intensity<T, const DIM: usize> {
    dims: [T; DIM],
}

pub trait Posable<T, const DIM: usize> {
    fn position(&self) -> &Point<T, DIM>;
}

pub trait Magable<T: Copy + Clone, const DIM: usize> {
    fn mag_intensity(&self, p: Point<T, DIM>) -> Intensity<T, DIM>;
}

pub trait Electrable<T: Copy + Clone, const DIM: usize> {
    fn ele_intensity(&self, p: Point<T, DIM>) -> Intensity<T, DIM>;
}

pub trait Massable<T: Copy + Clone, const DIM: usize> {
    fn mas_intensity(&self, p: Point<T, DIM>) -> T;
}

pub trait Fieldable<T: Copy + Clone, const DIM: usize> {
    fn mas_intensity(&self, p: Point<T, DIM>) -> Intensity<T, DIM>;
}

pub trait Movable<T: Copy + Clone, const DIM: usize>: Posable<T, DIM> {
    fn velocity(&self) -> &Intensity<T, DIM>;
    fn acceleration(&self) -> &Intensity<T, DIM>;
}

struct Particle<T, const DIM: usize> {
    pos: Point<T, DIM>,
    vel: Intensity<T, DIM>,
    acc: Intensity<T, DIM>,
    mass: T,
    elec: Intensity<T, DIM>,
    mag: Intensity<T, DIM>,
}

impl<T, const DIM: usize> Posable<T, DIM> for Particle<T, DIM> {
    fn position(&self) -> &Point<T, DIM> {
        &self.pos
    }
}

impl<T: Copy + Clone, const DIM: usize> Movable<T, DIM> for Particle<T, DIM> {
    fn velocity(&self) -> &Intensity<T, DIM> {
        &self.vel
    }

    fn acceleration(&self) -> &Intensity<T, DIM> {
        &self.acc
    }
}

impl<T: Copy + Clone, const DIM: usize> Massable<T, DIM> for Particle<T, DIM> {
    fn mas_intensity(&self, _p: Point<T, DIM>) -> T {
        self.mass
    }
}

impl<T: Copy + Clone, const DIM: usize> Electrable<T, DIM> for Particle<T, DIM> {
    fn ele_intensity(&self, _p: Point<T, DIM>) -> Intensity<T, DIM> {
        self.elec
    }
}

fn main() {
    let p = Particle {
        pos: Point { dims: [1.0, 2.0, 3.0] },
        vel: Intensity { dims: [0.0, 0.0, 0.0] },
        acc: Intensity { dims: [0.0, 0.0, 0.0] },
        mass: 20.0,
        elec: Intensity { dims: [0.0, 0.0, 0.0] },
        mag: Intensity { dims: [0.0, 0.0, 0.0] },
    };

    let pos = p.position();

    println!("{}", pos.dims[0]);
    println!("Hello, Dao!");
}
