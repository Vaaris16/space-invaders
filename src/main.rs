use bevy::prelude::*;

use crate::core::core_plugin::CorePlugin;

mod core;

fn main() {
    App::new().add_plugins((DefaultPlugins, CorePlugin)).run();
}
