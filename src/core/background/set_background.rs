use bevy::prelude::*;

pub fn set_background(mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = Color::BLACK;
}
