use bevy::prelude::*;

use crate::core::{background::set_background::set_background, player::spawn_player::spawn_player};

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Startup, set_background);
    }
}
