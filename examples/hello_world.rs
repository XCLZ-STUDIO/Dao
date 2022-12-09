use imgui::*;
use dao::*;

mod support;

fn main() {
    let delta_t = 0.1;

    let mut p0 = Particle::new(
        Point::new([10.0, 20.0, 0.0]),
        Intensity::new([1.0, 0.0, 0.0]),
        Intensity::new([0.0, 0.0, 0.0]),
        2000.0,
        Intensity::new([0.0, 0.0, 0.0]),
        Intensity::new([0.0, 0.0, 0.0])
    );

    let mut p1 = Particle::new(
        Point::new([100.0, 100.0, 0.0]),
        Intensity::new([0.0, 2.0, 0.0]),
        Intensity::new([0.0, 0.0, 0.0]),
        2000.0,
        Intensity::new([0.0, 0.0, 0.0]),
        Intensity::new([0.0, 0.0, 0.0])
    );

    let system = support::init(file!());
    let mut value = 1.0;
    system.main_loop(move |_, ui| {
        ui.window("Hello world")
            .size([300.0, 110.0], Condition::FirstUseEver)
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