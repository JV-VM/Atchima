mod config;
mod game;
use crate::config::components::{AnimationMapping, EntityConfig};
use crate::game::world::WorldManagementPlugin;
use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_ecs_tiled::tiled::TiledPlugin;
use bevy_spritesheet_animation::plugin::SpritesheetAnimationPlugin;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_resource::<AnimationMapping>()
        .add_plugins(TiledPlugin::default())
        .add_plugins(RonAssetPlugin::<EntityConfig>::new(&["setup.ron"]))
        .add_plugins(SpritesheetAnimationPlugin)
        .add_plugins(WorldManagementPlugin)
        .run();
}
