use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

pub fn default_cube(
    transform: Transform,
    size: f32,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> (PbrBundle, PickableBundle, RaycastPickTarget, On<Pointer<DragStart>>, On<Pointer<DragEnd>>, On<Pointer<Drag>>) {
    (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size })),
            material: materials.add(Color::RED.into()),
            transform,
            ..Default::default()
        },
        PickableBundle::default(),
        RaycastPickTarget::default(),
        On::<Pointer<DragStart>>::target_insert(Pickable::IGNORE), // Disable picking
        On::<Pointer<DragEnd>>::target_insert(Pickable::default ()), // Re-enable picking
        //drag_move needs the pointer to get hit info
        On::<Pointer<Drag>>::run(place_at_hit),
    )
}

pub fn place_at_hit(
    event: Listener<Pointer<Drag>>,
) {
    info!("place_at_hit {:?}", event);
}