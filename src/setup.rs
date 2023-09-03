use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_mod_picking::prelude::*;
//use std::fmt::Debug;
//include cube fn from cube.rs
use crate::cube::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3dBundle {
        projection: OrthographicProjection {
                scale: 3.0,
                scaling_mode: ScalingMode::FixedVertical(2.0),
                ..default()
            }
            .into(),
            transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        RaycastPickCamera::default(),
    ));
    // plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(5.0).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        PickableBundle::default(),
        RaycastPickTarget::default(),
    ))
    .with_children(|parent| {
        for i in 1..=5 {
            parent.spawn(default_cube(Transform::from_xyz(0.0, 1.0 * i as f32, 0.0), 1.0, &mut meshes, &mut materials));
        }
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

// //get move cursor event and set cursor transform
// pub fn print<E: Debug + Clone + Reflect>(
//     mut pointer_events: EventReader<Pointer<Move>>,
// ) {
//     for event in pointer_events.iter() {
//        info!("{event}");
//     }
//}
