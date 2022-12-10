use dao::entities::Particle;
use dao::physic_traits::{Intensity, Interoperable, Posable};
use dao::utils::Point;

fn main() {
    let delta_t = 1.0;

    let mut p0 = Particle::new(
        Point::new([1.0, 2.0, 3.0]),
        Intensity::new([0.0, 0.0, 0.0]),
        Intensity::new([0.0, 0.0, 0.0]),
        20.0,
        Intensity::new([0.0, 0.0, 0.0]),
        Intensity::new([0.0, 0.0, 0.0]),
    );

    let mut p1 = Particle::new(
        Point::new([10.0, 3.0, -3.0]),
        Intensity::new([0.0, 0.0, 0.0]),
        Intensity::new([0.0, 0.0, 0.0]),
        20.0,
        Intensity::new([0.0, 0.0, 0.0]),
        Intensity::new([0.0, 0.0, 0.0]),
    );

    println!("({} {} {})", p0.position().get(0), p0.position().get(1), p0.position().get(2));
    println!("({} {} {})", p1.position().get(0), p1.position().get(1), p1.position().get(2));

    p0.interact(&p1);
    p1.interact(&p0);

    p0.update(delta_t);
    p1.update(delta_t);

    println!("({} {} {})", p0.position().get(0), p0.position().get(1), p0.position().get(2));
    println!("({} {} {})", p1.position().get(0), p1.position().get(1), p1.position().get(2));

    println!("Hello, Dao!");
}

