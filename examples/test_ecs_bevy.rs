use std::any::{Any};
use std::fmt::Debug;
use bevy::prelude::{App, Bundle, Commands, Component, Query, With};
// use bevy_app::App;
// use bevy_ecs::component::Component;
// use bevy_ecs::system::{Commands, Query};
use dao::entities::Particle;
use dao::physic_traits::{Intensity, Interoperable, Posable};
use dao::utils::Point;

#[derive(Component, Debug)]
struct Posablee<T: Debug> {
    x: T,
}

#[derive(Component, Debug)]
struct Massablee;

fn setup(mut commands: Commands) {
    let particles = [
        Particle::new(
            Point::new([0.0, 0.0, 0.0]),
            Intensity::new([0.0, 0.0, 0.0]),
            Intensity::new([0.0, 0.0, 0.0]),
            0.51,
            Intensity::new([0.0, 0.0, 0.0]),
            Intensity::new([0.0, 0.0, 0.0]),
        ), Particle::new(
            Point::new([-10.0, 10.0, 0.0]),
            Intensity::new([0.0, 0.0, 0.2]),
            Intensity::new([0.0, 0.0, 0.0]),
            2.0,
            Intensity::new([0.0, 0.0, 0.0]),
            Intensity::new([0.0, 0.0, 0.0]),
        ), Particle::new(
            Point::new([0.0, 10.0, 0.0]),
            Intensity::new([0.0, 0.1, -0.2]),
            Intensity::new([0.0, 0.0, 0.0]),
            2.0,
            Intensity::new([0.0, 0.0, 0.0]),
            Intensity::new([0.0, 0.0, 0.0]),
        ), Particle::new(
            Point::new([20.0, 10.0, 0.0]),
            Intensity::new([0.0, -0.1, -0.1]),
            Intensity::new([0.0, 0.0, 0.0]),
            2.0,
            Intensity::new([0.0, 0.0, 0.0]),
            Intensity::new([0.0, 0.0, 0.0]),
        )];

    for particle in particles {
        commands.spawn(ParticleBundle {
            posable: Posablee {
                x: 1
            },
            massable: Massablee {},
        });
    }
}

#[derive(Bundle)]
struct ParticleBundle<T: Debug + Send + Sync + 'static> {
    posable: Posablee<T>,
    massable: Massablee,
}

fn update<T: Debug + Send + Sync + 'static>(mut query: Query<(&mut Posablee<T>, Option<&mut Massablee>)>/*, time: Res<Time>*/) {
    let mut combinations = query.iter_combinations_mut();
    while let Some([mut entity1, mut entity2]) = combinations.fetch_next() {
        let (pos1, mas1) = entity1;
        let (pos2, mas2) = entity2;
        println!("{:?}, {:?}", pos1, pos1);
        println!("{:?}, {:?}", pos2, pos2);
    }

    for (posable, massable) in &query {
        println!("{:?}", posable);
    }
    // for mut ball in &mut query {
    //     ball.0.update(0.01);
    //     println!("{:?}", ball.0.position());
    // }
}


fn main() {
    let mut app = App::new();

    app.add_startup_system(setup)
        .add_system(update::<f32>)
        .add_system(update::<i32>);

    app.world.spawn(ParticleBundle {
        posable: Posablee {
            x: 2
        },
        massable: Massablee {},
    });

    app.update();
    app.update();
    app.update();

    println!("over!");

    // for entity in app.world.iter_entities() {
    //     if let Some(ball) = app.world.get::<Ball>(entity) {
    //         println!("{:?}", ball.0.position());
    //     }
    // }
}

