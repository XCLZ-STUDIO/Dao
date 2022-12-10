use imgui::*;
use dao::entities::Particle;
use dao::physic_traits::{Intensity, Interoperable, Posable};
use dao::utils::Point;

mod support;

fn main() {
    let delta_t = 0.5;

    let mut p0 = Particle::new(
        Point::new([300.0, 300.0, 0.0]),
        Intensity::new([0.0, 1.0, 0.0]),
        Intensity::new([0.0, 0.0, 0.0]),
        200.0,
        Intensity::new([0.0, 0.0, 0.0]),
        Intensity::new([0.0, 0.0, 0.0])
    );

    let mut p1 = Particle::new(
        Point::new([400.0, 300.0, 0.0]),
        Intensity::new([0.0, -1.0, 0.0]),
        Intensity::new([0.0, 0.0, 0.0]),
        200.0,
        Intensity::new([0.0, 0.0, 0.0]),
        Intensity::new([0.0, 0.0, 0.0])
    );

    let system = support::init(file!());
    let mut value = 1.0;
    system.main_loop(move |_, ui| {
        ui.window("Hello world")
            .size([800.0, 500.0], Condition::FirstUseEver)
            .build(|| {
                let fg_draw_list = ui.get_foreground_draw_list();

                {
                    // let [w, h] = ui.io().display_size;
                    let x = p0.position().get(0);
                    let y = p0.position().get(1);
                    fg_draw_list
                        .add_circle([*x, *y], 10.0, [1.0, 0.0, 0.0])
                        .thickness(1.0)
                        .build();
                }

                {
                    // let [w, h] = ui.io().display_size;
                    let x = p1.position().get(0);
                    let y = p1.position().get(1);
                    fg_draw_list
                        .add_circle([*x, *y], 10.0, [1.0, 0.0, 0.0])
                        .thickness(1.0)
                        .build();
                }

                p0.interact(&p1);
                p1.interact(&p0);

                p0.update(delta_t);
                p1.update(delta_t);
            });
    });
}