use std::any::{Any};
use std::fmt::Debug;
use bevy::prelude::{App, Commands, Component, Query};
// use bevy_app::App;
// use bevy_ecs::component::Component;
// use bevy_ecs::system::{Commands, Query};
use dao::entities::Particle;
use dao::physic_traits::{Intensity, Interoperable, Posable};
use dao::utils::Point;

#[derive(Component)]
struct Ball(Particle<f32, 3>);

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
        commands.spawn(Ball(particle));
    }
}

fn update(mut query: Query<&mut Ball>/*, time: Res<Time>*/) {
    let mut combinations = query.iter_combinations_mut();
    while let Some([mut ball1, mut ball2]) = combinations.fetch_next() {
        ball1.0.interact(&ball2.0);
        ball2.0.interact(&ball1.0);
    }

    for mut ball in &mut query {
        ball.0.update(0.01);
        println!("{:?}", ball.0.position());
    }
}


fn main() {
    let mut app = App::new();

    app.add_startup_system(setup)
        .add_system(update);

    app.update();
    app.update();
    app.update();

    println!("over!");

    for entity in app.world.iter_entities() {
        if let Some(ball) = app.world.get::<Ball>(entity) {
            println!("{:?}", ball.0.position());
        }
    }
}

