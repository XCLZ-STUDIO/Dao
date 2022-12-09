use num::{Float, One};

#[derive(Copy, Clone, Debug)]
pub struct Point<T, const DIM: usize> {
    dims: [T; DIM],
}

#[derive(Copy, Clone, Debug)]
pub struct Intensity<T: Float, const DIM: usize> {
    dims: [T; DIM],
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

pub struct Particle<T: Float, const DIM: usize> {
    pos: Point<T, DIM>,
    vel: Intensity<T, DIM>,
    acc: Intensity<T, DIM>,
    mass: T,
    elec: Intensity<T, DIM>,
    mag: Intensity<T, DIM>,
}

impl<T: Float, const DIM: usize> Posable<T, DIM> for Particle<T, DIM> {
    fn position(&self) -> &Point<T, DIM> {
        &self.pos
    }
}

impl<T: Float, const DIM: usize> Movable<T, DIM> for Particle<T, DIM> {
    fn velocity(&self) -> &Intensity<T, DIM> {
        &self.vel
    }

    fn acceleration(&self) -> &Intensity<T, DIM> {
        &self.acc
    }
}

impl<T: Float, const DIM: usize> Massable<T, DIM> for Particle<T, DIM> {
    fn mas_intensity(&self, _p: Point<T, DIM>) -> T {
        self.mass
    }
}

impl<T: Float, const DIM: usize> Electrable<T, DIM> for Particle<T, DIM> {
    fn ele_intensity(&self, _p: Point<T, DIM>) -> Intensity<T, DIM> {
        self.elec
    }
}

impl<T: Float> Interoperable<T, T, 3> for Particle<T, 3> {
    fn interact(&mut self, obj: &Particle<T, 3>) {
        let sel_pos = &self.position().dims;
        let obj_pos = &obj.position().dims;

        let dis_2_3 = ((sel_pos[0] - obj_pos[0]).powi(2)
            + (sel_pos[1] - obj_pos[1]).powi(2)
            + (sel_pos[2] - obj_pos[2].powi(2))).sqrt().powi(3);

        let mut g: T = One::one();

        let gra_acc_dlt_mod = g * obj.mas_intensity(obj.pos) / dis_2_3;
        let acc = Intensity {
            dims: [
                gra_acc_dlt_mod * (sel_pos[0] - obj_pos[0]),
                gra_acc_dlt_mod * (sel_pos[1] - obj_pos[1]),
                gra_acc_dlt_mod * (sel_pos[2] - obj_pos[2])
            ]
        };
        self.acc = acc;
    }

    fn update(&mut self, t: T) {
        let tmp_i = Intensity {
            dims: [
                self.vel.dims[0] + self.acc.dims[0] * t,
                self.vel.dims[1] + self.acc.dims[1] * t,
                self.vel.dims[2] + self.acc.dims[2] * t
            ]
        };
        self.vel = tmp_i;

        let tmp_i = Point {
            dims: [
                self.pos.dims[0] + self.vel.dims[0] * t,
                self.pos.dims[1] + self.vel.dims[1] * t,
                self.pos.dims[2] + self.vel.dims[2] * t
            ]
        };
        self.pos = tmp_i;
    }
}

fn main() {
    let delta_t = 1.0;

    let mut p0 = Particle {
        pos: Point { dims: [1.0, 2.0, 3.0] },
        vel: Intensity { dims: [0.0, 0.0, 0.0] },
        acc: Intensity { dims: [0.0, 0.0, 0.0] },
        mass: 20.0,
        elec: Intensity { dims: [0.0, 0.0, 0.0] },
        mag: Intensity { dims: [0.0, 0.0, 0.0] },
    };

    let mut p1 = Particle {
        pos: Point { dims: [10.0, 3.0, -3.0] },
        vel: Intensity { dims: [0.0, 0.0, 0.0] },
        acc: Intensity { dims: [0.0, 0.0, 0.0] },
        mass: 20.0,
        elec: Intensity { dims: [0.0, 0.0, 0.0] },
        mag: Intensity { dims: [0.0, 0.0, 0.0] },
    };

    println!("({} {} {})", p0.position().dims[0], p0.position().dims[1], p0.position().dims[2]);
    println!("({} {} {})", p1.position().dims[0], p1.position().dims[1], p1.position().dims[2]);

    p0.interact(&p1);
    p1.interact(&p0);

    p0.update(delta_t);
    p1.update(delta_t);

    println!("({} {} {})", p0.position().dims[0], p0.position().dims[1], p0.position().dims[2]);
    println!("({} {} {})", p1.position().dims[0], p1.position().dims[1], p1.position().dims[2]);

    println!("Hello, Dao!");
}
