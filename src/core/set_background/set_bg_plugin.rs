use bevy::prelude::*;

pub struct SetBackgroundPlugin;

impl Plugin for SetBackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, set_bg_tmp);
        //app.add_systems(Startup, set_bg)
        //  .add_systems(Update, resize_bg);
    }
}

//#[derive(Component)]
//struct GameBackground;
//
//fn set_bg(asset_server: Res<AssetServer>, mut commands: Commands) {
//    commands.spawn((
//        Sprite {
//            image: asset_server.load("bg_image.png"),
//            ..Default::default()
//        },
//        Transform::default(),
//        GameBackground,
//    ));
//}
//
//fn resize_bg(mut bg: Single<&mut Sprite, With<GameBackground>>, window: Single<&Window>) {
//    bg.custom_size = Some(Vec2::new(window.width(), window.height()));
//}

fn set_bg_tmp(mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = Color::BLACK;
}
