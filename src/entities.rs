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

impl<T: Float> Interoperable<T, T, 3> for Particle<T, 3> {
    fn interact(&mut self, obj: &Particle<T, 3>) {
        let sel_pos = self.position();
        let obj_pos = obj.position();

        let dis_2_3 = ((*sel_pos.get(0) - *obj_pos.get(0)).powi(2)
            + (*sel_pos.get(1) - *obj_pos.get(1)).powi(2)
            + (*sel_pos.get(2) - *obj_pos.get(2)).powi(2)).sqrt().powi(3);

        let mut g: T = One::one();

        let gra_acc_dlt_mod = g * obj.mas_intensity(obj.pos) / dis_2_3;
        let acc = Intensity::new([
            gra_acc_dlt_mod * (*obj_pos.get(0) - *sel_pos.get(0)),
            gra_acc_dlt_mod * (*obj_pos.get(1) - *sel_pos.get(1)),
            gra_acc_dlt_mod * (*obj_pos.get(2) - *sel_pos.get(2))
        ]);
        self.acc = acc;
    }

    fn update(&mut self, t: T) {
        self.vel = Intensity::new([
            *self.vel.get(0) + *self.acc.get(0) * t,
            *self.vel.get(1) + *self.acc.get(1) * t,
            *self.vel.get(2) + *self.acc.get(2) * t
        ]);

        let tmp_i = Point::new([
            *self.pos.get(0) + *self.vel.get(0) * t,
            *self.pos.get(1) + *self.vel.get(1) * t,
            *self.pos.get(2) + *self.vel.get(2) * t
        ]);
        self.pos = tmp_i;
    }
}
