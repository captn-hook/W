use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use std::fmt::Debug;

#[derive(Component, Debug, Clone, Reflect, Default)]
pub struct Cursor3d;


pub fn move_cursor<E: Debug + Clone + Reflect>(
    mut pointer_events: EventReader<Pointer<Move>>,
    //query for cursor
    mut cursor_query: Query<(&Cursor3d, &mut Transform)>,
) {
    for event in pointer_events.iter() {
        for (_cursor, mut transform) in cursor_query.iter_mut() {
            //event.hit.position: Some(Vec3), same for normal, get both
            if let (Some(position), Some(normal)) = (event.hit.position, event.hit.normal) {
                //set cursor transform to hit position + normal
                let position = position + normal * 0.1;
                transform.translation = position;
                //set cursor rotation to normal
                transform.rotation = Quat::from_rotation_arc(Vec3::Y, normal);
            }
        }
    }
}

pub fn add_cursor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((Cursor3d, PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule { radius: 0.1, depth: 0.2, ..Default::default() })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    }));
}