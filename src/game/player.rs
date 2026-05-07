use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{TiledMap, TiledMapAsset};
use bevy_spritesheet_animation::prelude::SpritesheetAnimation;

use crate::{
    config::components::{
        AnimationMapping, Collider, EntityConfig, Facing, LoadedEntityConfig,
    },
    game::world::{Collisions, MainCamera},
};

#[derive(Component)]
pub struct Player;
fn move_player(
    mut player_query: Query<
        (
            &mut Transform,
            &LoadedEntityConfig,
            Option<&mut SpritesheetAnimation>,
            &mut Facing,
            Option<&AnimationMapping>,
            &Collider,
        ),
        With<Player>,
    >,
    wall_query: Query<(&GlobalTransform, &Collider), (With<Collisions>, Without<Player>)>,
    entity_config: Res<Assets<EntityConfig>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok((mut transform, loaded, opt_animation, mut facing, opt_mapping, p_collider)) =
        player_query.single_mut()
    else {
        return;
    };
    let Some(config) = entity_config.get(&loaded.0) else {
        return;
    };

    let mut direction = Vec3::ZERO;
    let mut is_moving = false;
    if input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
        facing.suffix = "_UP".into();
        is_moving = true;
    }
    if input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
        facing.suffix = "_DOWN".into();
        is_moving = true;
    }
    if input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
        facing.suffix = "_LEFT".into();
        is_moving = true;
    }
    if input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
        facing.suffix = "_RIGHT".into();
        is_moving = true;
    }
    let player_center_offset = Vec3::new(0.0, p_collider.size.y / 2.0, 0.0);
    if is_moving {
        if let Some(dir_norm) = direction.try_normalize() {
            let velocity = dir_norm * config.speed * time.delta_secs();
            let next_x = transform.translation + Vec3::new(velocity.x, 0.0, 0.0);
            let mut collision_x = false;
            for (w_trans, w_collider) in &wall_query {
                if check_collision(
                    next_x + player_center_offset,
                    p_collider.size,
                    w_trans.translation(),
                    w_collider.size,
                ) {
                    collision_x = true;
                    break;
                }
            }
            if !collision_x {
                transform.translation.x = next_x.x;
            }
            let next_y = transform.translation + Vec3::new(0.0, velocity.y, 0.0);
            let mut collision_y = false;
            for (w_trans, w_collider) in &wall_query {
                if check_collision(
                    next_y + player_center_offset,
                    p_collider.size,
                    w_trans.translation(),
                    w_collider.size,
                ) {
                    collision_y = true;
                    break;
                }
            }
            if !collision_y {
                transform.translation.y = next_y.y;
            }
        }
    }
    if let (Some(mut animation), Some(mapping)) = (opt_animation, opt_mapping) {
        let state = if is_moving { "MOVING" } else { "IDLE" };
        let anim_key = format!("{}_{}{}", config.name, state, facing.suffix);

        if let Some(new_handle) = mapping.0.get(&anim_key) {
            if animation.animation.id() != new_handle.id() {
                animation.switch(new_handle.clone());
            }
        }
    }
}

fn check_collision(pos_a: Vec3, size_a: Vec2, pos_b: Vec3, size_b: Vec2) -> bool {
    let a_min = pos_a.truncate() - size_a / 2.0;
    let a_max = pos_a.truncate() + size_a / 2.0;
    let b_min = pos_b.truncate() - size_b / 2.0;
    let b_max = pos_b.truncate() + size_b / 2.0;

    a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y
}

fn camera_follow_system(
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    map_query: Query<&TiledMap>,
    tiled_assets: Res<Assets<TiledMapAsset>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };
    let Ok(tiled_handle) = map_query.single() else {
        return;
    };
    let Some(map_data) = tiled_assets.get(&tiled_handle.0) else {
        return;
    };

    let world_width = map_data.map.width as f32 * map_data.map.tile_width as f32;
    let world_height = map_data.map.height as f32 * map_data.map.tile_height as f32;

    let viewport_width = 1280.0;
    let viewport_height = 720.0;

    let half_width = viewport_width / 2.0;
    let half_height = viewport_height / 2.0;

    let min_x = half_width;
    let max_x = world_width - half_width;
    let min_y = half_height;
    let max_y = world_height - half_height;

    camera_transform.translation.x = player_transform.translation.x.clamp(min_x, max_x);
    camera_transform.translation.y = player_transform.translation.y.clamp(min_y, max_y);
}
#[derive(Debug)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_player, camera_follow_system));
    }
}
