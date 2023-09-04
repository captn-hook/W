use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use std::fmt::Debug;
use crate::cursor::*;

#[derive(Component, Debug, Clone, Reflect)]
pub struct Moveable {
    pub is_movable: bool,
}

impl Default for Moveable {
    fn default() -> Self {
        Self { is_movable: false } // By default, entities are moveable
    }
}

impl Moveable {
    pub fn new(is_movable: bool) -> Self {
        Self { is_movable }
    }
}

pub fn default_cube(
    transform: Transform,
    size: f32,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> (Moveable, PbrBundle, PickableBundle, RaycastPickTarget, On<Pointer<DragStart>>, On<Pointer<DragEnd>>, On<Pointer<Drag>>) {
    (
        Moveable::default(),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size })),
            material: materials.add(Color::RED.into()),
            transform,
            ..Default::default()
        },
        PickableBundle::default(),
        RaycastPickTarget::default(),
        On::<Pointer<DragStart>>::target_insert((Pickable::IGNORE, Moveable::new(true))), // Disable picking
        On::<Pointer<DragEnd>>::target_insert((Pickable::default (), Moveable::default ())), // Enable picking
        //on drag -> move cube to cursor position
        //get cursor3d and just print it for now
        On::<Pointer<Drag>>::run(move_to_cursor_position::<Drag>),
    )
}

pub fn move_to_cursor_position<E: Debug + Clone + Reflect>(
    _pointer_events: EventReader<Pointer<Drag>>,
    cursor_query: Query<(&Cursor3d, &Transform), Without<Moveable>>,
    mut moveable_query: Query<(&Moveable, &mut Transform)>,
) {
    for (_cursor, cursor_transform) in cursor_query.iter() {
        for (_moveable, mut moveable_transform) in moveable_query.iter_mut() {
            if !_moveable.is_movable {
                continue;
            } 
            moveable_transform.translation = cursor_transform.translation;
        }
    }
}