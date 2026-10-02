use bevy::prelude::*;

use crate::core::{
    camera::camera_plugin::CameraPlugin, set_background::set_bg_plugin::SetBackgroundPlugin,
};

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SetBackgroundPlugin, CameraPlugin));
    }
}
