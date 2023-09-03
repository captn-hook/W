show a grid onscreen under the cursor in radius

//this moves a entity with pickable and transform components on the drag event
On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
    transform.translation.x += drag.delta.x; // Make the square follow the mouse
    transform.translation.y -= drag.delta.y;
}),

//this adds a command on entities with pickable
On::<Pointer<Drop>>::commands_mut(|event, commands| {
    commands.entity(event.dropped).insert(Spin(FRAC_PI_2)); // Spin dropped entity
    commands.entity(event.target).insert(Spin(-FRAC_PI_2)); // Spin dropped-on entity
}),

//this is a component that can be added as a command
#[derive(Component)]
pub struct Spin(f32);

pub fn spin(mut square: Query<(&mut Spin, &mut Transform)>) {
    for (mut spin, mut transform) in square.iter_mut() {
        transform.rotation = Quat::from_rotation_z(spin.0);
        let delta = -spin.0.clamp(-1.0, 1.0) * 0.05;
        spin.0 += delta;
    }
}


//! This example demonstrates how backends can be mixed and matched, specifically with egui. Here,
//! we are using the egui backend, which is enabled automatically in `DefaultPickingPlugins` when
//! the "egui_backend" feature is enabled. The egui backend will automatically apply a `NoDeselect`
//! component to the egui entity, which allows you to interact with the UI without deselecting
//! anything in the 3d scene.
use bevy_egui::{
    egui::{self, ScrollArea},
    EguiContexts, EguiPlugin,
};