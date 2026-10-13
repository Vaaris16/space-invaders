use bevy::prelude::*;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
