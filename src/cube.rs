use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

pub fn default_cube(
    transform: Transform,
    size: f32,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> (PbrBundle, PickableBundle, RaycastPickTarget, On<Pointer<DragStart>>, On<Pointer<DragEnd>>, On<Pointer<Drag>>/*, On<Pointer<DragOver>> */) {
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
        On::<Pointer<Drag>>::target_component_mut::<Transform>(drag_move),
        // On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
        //     let unit = 30.0;
        //     let sensitivity = [0.15, 0.25];
        //     //gets the raycast of the drag
        //     //info!("{:?}", drag);
        //     let x = drag.delta.x * sensitivity[0];
        //     let y = drag.delta.y * sensitivity[1];
        //     transform.translation.x += (y + x) / unit;
        //     transform.translation.z += (y - x) / unit;
        // }),
    )
}


#[derive(Event)]
struct PlaceCube(Entity, f32);

impl From<ListenerInput<Pointer<Down>>> for PlaceCube {
    fn from(event: ListenerInput<Pointer<Down>>) -> Self {
        PlaceCube(event.target, event.hit.depth)
    }
}
//Fn<(&mut bevy_mod_picking::prelude::ListenerInput<bevy_mod_picking::prelude::Pointer<bevy_mod_picking::prelude::Drag>>, &mut bevy::prelude::Transform)>
fn drag_move(
    drag: &ListenerInput<Pointer<Drag>>,
    transform: &mut Transform,
) {
    let unit = 30.0;
    let sensitivity = [0.15, 0.25];
    //gets the raycast of the drag
    //info!("{:?}", drag);
    let x = drag.delta.x * sensitivity[0];
    let y = drag.delta.y * sensitivity[1];
    transform.translation.x += (y + x) / unit;
    transform.translation.z += (y - x) / unit;
}
