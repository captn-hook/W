use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

mod setup;
mod cube;
mod cursor;
mod grid;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(low_latency_window_plugin()),
            DefaultPickingPlugins
                .build()//,
                .disable::<DebugPickingPlugin>()
                .disable::<DefaultHighlightingPlugin>(),
            bevy_egui::EguiPlugin,
        ))
        .add_systems(Startup, (
            setup::setup,
            cursor::add_cursor,
        ))
        .add_systems(Update, (
            cursor::move_cursor::<events::Move>,
            grid::grid,
        ))
        .run();
}
