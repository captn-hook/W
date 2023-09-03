use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

mod setup;
mod cube;

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
        .add_systems(Startup, setup::setup)
        //.add_systems(Update, dropd::spin)
        .run();
}
