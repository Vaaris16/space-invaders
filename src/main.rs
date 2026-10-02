use bevy::prelude::*;

use crate::core::core_plugin::CorePlugin;

mod core;

#[derive(States, Debug, Default, Hash, PartialEq, Eq, Clone)]
pub enum GameState {
    #[default]
    SplashScreen,
    Game,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CorePlugin))
        .init_state::<GameState>()
        .run();
}
