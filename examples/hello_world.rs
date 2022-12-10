use std::mem::swap;
use dao::entities::Particle;
use dao::physic_traits::{Intensity, Interoperable, Posable};
use dao::utils::Point;
use itertools::{Itertools, max, min};

mod support;

fn main() {
    let delta_t = 0.5;

    let mut particles = vec![
        Particle::new(
            Point::new([300.0, 300.0, 0.0]),
            Intensity::new([0.0, 0.0, 0.1]),
            Intensity::new([0.0, 0.0, 0.0]),
            200.0,
            Intensity::new([0.0, 0.0, 0.0]),
            Intensity::new([0.0, 0.0, 0.0]),
        ), Particle::new(
            Point::new([400.0, 300.0, 0.0]),
            Intensity::new([0.0, -1.0, 0.0]),
            Intensity::new([0.0, 0.0, 0.0]),
            200.0,
            Intensity::new([0.0, 0.0, 0.0]),
            Intensity::new([0.0, 0.0, 0.0]),
        ), Particle::new(
            Point::new([500.0, 300.0, 0.0]),
            Intensity::new([0.0, 1.0, -0.1]),
            Intensity::new([0.0, 0.0, 0.0]),
            200.0,
            Intensity::new([0.0, 0.0, 0.0]),
            Intensity::new([0.0, 0.0, 0.0]),
        )];

    let system = support::init(file!());
    system.main_loop(move |_, ui| {
        for particle in (0..particles.len()).into_iter().permutations(2) {
            let p_min = min(&particle).unwrap();
            let p_max = max(&particle).unwrap();

            let p0 = particles.split_at_mut(p_min + 1);
            let p1 = p0.0.last_mut().unwrap();
            let p2 = p0.1.get_mut(p_max - p_min - 1).unwrap();

            if particle[0] > particle[1] { swap(p1, p2) }

            p1.interact(p2);
        }
        for particle in &mut particles {
            particle.update(delta_t);
        }

        let fg_draw_list = ui.get_foreground_draw_list();

        for particle in &particles {
            let x = *particle.position().get(0) as f32;
            let y = *particle.position().get(1) as f32;

            fg_draw_list
                .add_circle([x, y], 10.0, [1.0, 0.0, 0.0])
                .filled(true)
                .thickness(1.0)
                .build();
        }
    });
}
