use std::f32::consts::FRAC_PI_2;


use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_mod_picking::prelude::*;
use gloo_console::log;

//use cube::*;
pub fn cube(transform: Transform, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>) -> PbrBundle {
    PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform,
        ..default()
    }
    //.with(PickableBundle::default());
}
// set up a simple 3D scene
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    commands.spawn(Camera3dBundle {
        projection: OrthographicProjection {
            scale: 3.0,
            scaling_mode: ScalingMode::FixedVertical(2.0),
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cubes
    commands.spawn((
        cube(Transform::from_xyz(0.0, 0.5, 0.0), &mut meshes, &mut materials),
        //PickableBundle::default(), // <- Makes the mesh pickable.
        On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
            transform.rotate_local_y(drag.delta.x / 50.0);
            log!("dragging");
        }),
        //hover_color: Color::rgb(0.5, 0.5, 0.5), set k
        // On::<Pointer<Focus>>::log_cursor_in(),
        // On::<Pointer<DragStart>>::target_insert(Pickable::IGNORE), // Disable picking
        // On::<Pointer<DragEnd>>::target_insert(Pickable::default()), // Re-enable picking
        // On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
        //     transform.translation.x += drag.delta.x; // Make the square follow the mouse
        //     transform.translation.y -= drag.delta.y;
        // }),
        // On::<Pointer<Drop>>::commands_mut(|event, commands| {
        //     commands.entity(event.dropped).insert(Spin(FRAC_PI_2)); // Spin dropped entity
        //     commands.entity(event.target).insert(Spin(-FRAC_PI_2)); // Spin dropped-on entity
        // }),
    ));
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(3.0, 8.0, 5.0),
        ..default()
    });
}

#[derive(Component)]
struct Spin(f32);

fn spin(mut square: Query<(&mut Spin, &mut Transform)>) {
    for (mut spin, mut transform) in square.iter_mut() {
        transform.rotation = Quat::from_rotation_z(spin.0);
        let delta = -spin.0.clamp(-1.0, 1.0) * 0.05;
        spin.0 += delta;
    }
}