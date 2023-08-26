use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

pub fn default_cube(
    transform: Transform,
    size: f32,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> (PbrBundle, PickableBundle, RaycastPickTarget) {
    (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size })),
            material: materials.add(Color::RED.into()),
            transform,
            ..Default::default()
        },
        PickableBundle::default(),
        RaycastPickTarget::default(),
    )
}