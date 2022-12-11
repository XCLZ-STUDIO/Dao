use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::prelude::shape::UVSphere;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use dao::entities::Particle;
use dao::physic_traits::{Intensity, Interoperable, Massable, Posable};
use dao::utils::Point;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(update)
        .run();
}

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Ball(Particle<f32, 3>);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let particles = [
        Particle::new(
            Point::new([0.0, 0.0, 0.0]),
            Intensity::new([0.0, 0.0, 0.0]),
            Intensity::new([0.0, 0.0, 0.0]),
            0.5,
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

    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    for particle in particles {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(UVSphere { radius: particle.mas_intensity(Point::new([0., 0., 0.])), ..default() }.into()),
                material: debug_material.clone(),
                transform: Transform::from_xyz(
                    *particle.position().get(0),
                    *particle.position().get(1),
                    *particle.position().get(2),
                ),
                ..default()
            },
            Ball(particle),
        ));
    }

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 12000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0., 20., 0.),
        ..default()
    });

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane { size: 500. }.into()),
        material: materials.add(Color::SILVER.into()),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 30., 40.)
            .looking_at(Vec3::new(0., 10., 0.), Vec3::Y),
        ..default()
    });
}

const DELTA_T: f32 = 0.1;

fn update(mut query: Query<(&mut Transform, &mut Ball)>/*, time: Res<Time>*/) {
    let mut combinations = query.iter_combinations_mut();
    while let Some([(mut trans1, mut ball1), (mut trans2, mut ball2)]) = combinations.fetch_next() {
        ball1.0.interact(&ball2.0);
        ball2.0.interact(&ball1.0);
        ball1.0.update(DELTA_T);
        ball2.0.update(DELTA_T);
        trans1.translation.x = *ball1.0.position().get(0);
        trans1.translation.y = *ball1.0.position().get(1);
        trans1.translation.z = *ball1.0.position().get(2);
        trans2.translation.x = *ball2.0.position().get(0);
        trans2.translation.y = *ball2.0.position().get(1);
        trans2.translation.z = *ball2.0.position().get(2);
    }
}


/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
}

