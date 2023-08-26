
//mod postprocess;
mod ortho;

use bevy_mod_picking::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin { // Hot reloading shader
                watch_for_changes: bevy::asset::ChangeWatcher::with_delay(bevy::utils::Duration::from_millis(200)),
                ..default()
            }),
            DefaultPickingPlugins,
            //postprocess::PostProcessPlugin,
        ))                
        .add_systems(Startup, ortho::setup)
        //.add_systems(Update, (postprocess::rotate, postprocess::update_settings))
        .run();
}