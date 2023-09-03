use bevy::prelude::*;

#[derive(Component)]
pub struct Cursor;


pub fn cursor(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> PbrBundle {
    PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule {
            radius: 0.1,
            depth: 0.2,
            ..Default::default()
        })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        ..Default::default()
    }
}
