use bevy::{ecs::relationship::Relationship, prelude::*};
use bevy_ecs_tiled::prelude::{TilePos, TiledMap, TiledMapAsset, TiledName};

use crate::{
    config::components::{Collider, EntityAssetCache, EntityConfig, LoadedEntityConfig, YSort},
    game::{
        entities::{EntityPlugin, Tree},
        player::{Player, PlayerPlugin},
    },
};
#[derive(Component)]
pub struct MainCamera;

fn map_spawning(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("maps/first_map.tmx");

    commands.spawn(TiledMap(handle));

    commands.spawn((Camera2d::default(), MainCamera));
}

fn center_player_on_map(
    mut player_query: Query<&mut Transform, With<Player>>,
    map_query: Query<&TiledMap>,
    tiled_assets: Res<Assets<TiledMapAsset>>,
    mut was_centered: Local<bool>,
) {
    if *was_centered {
        return;
    }

    let Ok(mut player_trans) = player_query.single_mut() else {
        info!("Waiting Player...");
        return;
    };
    let Ok(tiled_handle) = map_query.single() else {
        info!("Waiting Map...");
        return;
    };
    let Some(map_data) = tiled_assets.get(&tiled_handle.0) else {
        info!("Waiting for the Asset from Map to finish loading...");
        return;
    };
    let world_width = map_data.map.width as f32 * map_data.map.tile_width as f32;
    let world_height = map_data.map.height as f32 * map_data.map.tile_height as f32;

    player_trans.translation.x = world_width / 2.0;
    player_trans.translation.y = world_height / 2.0;
    *was_centered = true;

    info!(
        "Player centralized on map: {}x{}",
        world_width, world_height
    );
}
pub fn y_sorting_system(mut query: Query<(&mut Transform, Option<&Player>), With<YSort>>) {
    for (mut transform, is_player) in &mut query {
        let y_pos = transform.translation.y;

        let y_pivot = if is_player.is_some() {
            y_pos - 60.0
        } else {
            y_pos
        };

        transform.translation.z = -y_pivot * 0.001;
    }
}
#[derive(Component)]
pub struct Collisions;
fn setup_tree_objects(
    mut commands: Commands,
    query: Query<(Entity, &ChildOf), Without<YSort>>,
    names: Query<&TiledName>,
) {
    for (entity, parent) in &query {
        if let Ok(layer_name) = names.get(parent.get()) {
            if layer_name.0 == "trees" {
                commands.entity(entity).insert((YSort, Tree));
            }
        }
    }
}

fn sync_player_collider(
    mut player_query: Query<(&mut Collider, &LoadedEntityConfig), With<Player>>,
    entity_configs: Res<Assets<EntityConfig>>,
) {
    for (mut collider, loaded) in &mut player_query {
        if let Some(config) = entity_configs.get(&loaded.0) {
            if collider.size != config.collider_size {
                collider.size = config.collider_size;
            }
        }
    }
}
fn setup_collision_walls(
    mut commands: Commands,
    query: Query<(Entity, &ChildOf, &TilePos), Without<Collider>>,
    layer_names: Query<&TiledName>,
    map_query: Query<&TiledMap>,
    tiled_assets: Res<Assets<TiledMapAsset>>,
) {
    let Ok(tiled_map) = map_query.single() else {
        return;
    };
    let Some(map_data) = tiled_assets.get(&tiled_map.0) else {
        return;
    };

    let tile_size = Vec2::new(
        map_data.map.tile_width as f32,
        map_data.map.tile_height as f32,
    );

    for (entity, child_of, tile_pos) in &query {
        let parent_entity = child_of.parent();

        if let Ok(name) = layer_names.get(parent_entity) {
            if name.0 == "Shadows" {
                let pos = Vec3::new(
                    tile_pos.x as f32 * tile_size.x,
                    tile_pos.y as f32 * tile_size.y,
                    0.0,
                );

                commands.entity(entity).insert((
                    Collider { size: tile_size },
                    Collisions,
                    Transform::from_translation(pos),
                    GlobalTransform::default(),
                ));
            }
        }
    }
}
pub struct WorldManagementPlugin;

impl Plugin for WorldManagementPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EntityAssetCache>()
            .add_systems(Startup, map_spawning)
            .add_systems(
                Update,
                (
                    center_player_on_map,
                    setup_tree_objects,
                    sync_player_collider,
                    setup_collision_walls,
                ),
            )
            .add_systems(PostUpdate, y_sorting_system)
            .add_plugins((EntityPlugin, PlayerPlugin));
    }
}
