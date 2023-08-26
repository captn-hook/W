use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
//takes a mutable reference to meshes, and materials to add the cube to the scene
pub fn default_cube(transform: Transform, size: f32, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) -> (PbrBundle, PickableBundle, RaycastPickTarget) {
    (
    PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size })),
        material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
        transform,
        ..Default::default()
    },
    PickableBundle::default(),
    RaycastPickTarget::default(),
    )
}