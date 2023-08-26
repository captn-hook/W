use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_mod_picking::prelude::*;

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
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    commands.spawn(( //this is the parent for all moveable cubes
        //add computed visibility to the cube
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..Default::default()
        },
        PickableBundle::default(),
        RaycastPickTarget::default(),
    
        On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
            //translate x and z axis
            let divisor = 200.0;
            transform.translation.x += drag.delta.x / divisor;
            transform.translation.z += drag.delta.y / divisor;

        }),
        On::<Pointer<Click>>::target_commands_mut(|click, target_commands| {
            if click.target != click.listener() && click.button == PointerButton::Secondary {
                target_commands.despawn();
            }
        }),
    ))
    .with_children(|parent| {
        for i in 1..=5 {
            parent.spawn((
                default_cube(Transform::from_xyz(0.0, 1.0 + 0.5 * i as f32, 0.0), 0.5, &mut meshes, &mut materials),
            ));
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