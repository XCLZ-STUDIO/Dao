use num::{Float, One};
use crate::physic_traits::*;
use crate::utils::Point;

pub struct Particle<T: Float, const DIM: usize> {
    pos: Point<T, DIM>,
    vel: Intensity<T, DIM>,
    acc: Intensity<T, DIM>,
    mass: T,
    elec: Intensity<T, DIM>,
    mag: Intensity<T, DIM>,
}

impl<T: Float, const DIM: usize> Particle<T, DIM> {
    pub fn new(
        pos: Point<T, DIM>,
        vel: Intensity<T, DIM>,
        acc: Intensity<T, DIM>,
        mass: T,
        elec: Intensity<T, DIM>,
        mag: Intensity<T, DIM>,
    ) -> Self {
        Self { pos, vel, acc, mass, elec, mag }
    }
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

impl<T: Float, const DIM: usize> Magable<T, DIM> for Particle<T, DIM> {
    fn mag_intensity(&self, _p: Point<T, DIM>) -> Intensity<T, DIM> {
        self.mag
    }
}

macro_rules! define_interoperable_for_dim {
    ($ty:tt, $dim:expr, $($indexes:tt),+) => {
        impl<T: $ty> Interoperable<T, T, $dim> for Particle<T, $dim> {
            fn interact(&mut self, obj: &Particle<T, $dim>) {
                let sel_pos = self.position();
                let obj_pos = obj.position();
                let dis_2_3 = ((T::zero()$(
                    +(sel_pos[$indexes] - obj_pos[$indexes]).powi(2)
                )*)).sqrt().powi(3);

                let g: T = One::one();

                let gra_acc_dlt_mod = g * obj.mas_intensity(obj.pos) / dis_2_3;
                let acc = Intensity::new([
                    $(
                        gra_acc_dlt_mod * (obj_pos[$indexes] - sel_pos[$indexes])
                    ),+
                ]);
                self.acc = acc;
            }

            fn update(&mut self, t: T) {
                self.vel = Intensity::new([
                    $(
                        self.vel[$indexes] + self.acc[$indexes] * t
                    ),+
                ]);

                let tmp_i = Point::new([
                    $(
                        self.pos[$indexes] + self.vel[$indexes] * t
                    ),+
                ]);
                self.pos = tmp_i;
            }
        }
    }
}

define_interoperable_for_dim!(Float, 2, 0, 1);
define_interoperable_for_dim!(Float, 3, 0, 1, 2);
define_interoperable_for_dim!(Float, 4, 0, 1, 2, 3);
