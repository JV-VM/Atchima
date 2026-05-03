use bevy::{ecs::system::SystemParam, platform::collections::HashMap, prelude::*, sprite::Anchor};
use bevy_spritesheet_animation::prelude::*;
use serde::Deserialize;

use crate::game::player::Player;

#[derive(Resource, Default)]
pub struct EntityAssetCache {
    pub entries: HashMap<String, (Sprite, AnimationMapping)>,
}
#[derive(Component)]
pub struct YSort;

#[derive(SystemParam)]
pub struct AnimationTools<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub entity_config: Res<'w, Assets<EntityConfig>>,
    pub asset_server: Res<'w, AssetServer>,
    pub atlas_layouts: ResMut<'w, Assets<TextureAtlasLayout>>,
    pub animations: ResMut<'w, Assets<Animation>>,
    pub cache: ResMut<'w, EntityAssetCache>,
}
#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
}
#[derive(Component, Resource)]
pub struct LoadedEntityConfig(pub Handle<EntityConfig>);

#[derive(Deserialize, Asset, Resource, TypePath)]
pub struct EntityConfig {
    pub speed: f32,
    pub name: String,
    pub collider_size: Vec2,
    pub animation_set: AnimationSet,
}

#[derive(Component)]
pub struct PendingAnimationLoad;

#[derive(Deserialize, Component)]
pub struct AnimationSet {
    pub path_image: String,
    pub tilesize_x: u32,
    pub tilesize_y: u32,
    pub columns: usize,
    pub rows: usize,
    pub animations: HashMap<String, AnimationFrameSet>,
}
#[derive(Deserialize, Component)]
pub struct AnimationFrameSet {
    pub start_frame_ptr: usize,
    pub end_frame_ptr: usize,
}

#[derive(Component, Default)]
pub struct Facing {
    pub suffix: String,
}
#[derive(Component, Resource, Clone, Default)]
pub struct AnimationMapping(pub HashMap<String, Handle<Animation>>);

pub fn create_entity_animations(
    query: Query<(Entity, &LoadedEntityConfig), With<PendingAnimationLoad>>,
    mut tools: AnimationTools,
) {
    for (entity, loaded_entity) in &query {
        let Some(_info) = &tools.entity_config.get(&loaded_entity.0) else {
            continue;
        };
        let path_image = _info.animation_set.path_image.to_string();
        let entity_name = &_info.name;
        let spritesheet = Spritesheet::new(
            &tools.asset_server.load::<Image>(path_image),
            _info.animation_set.columns,
            _info.animation_set.rows,
        );
        let mut res_animations = HashMap::new();
        if let Some((sprite_info, animation_mapping_info)) = tools.cache.entries.get(entity_name) {
            helper_entity_add(
                &mut tools.commands,
                entity,
                sprite_info.clone(),
                animation_mapping_info.clone(),
            );
            continue;
        }

        looping_creations(
            &mut tools.animations,
            _info,
            spritesheet.clone(),
            &mut res_animations,
        );

        let mut sprite = spritesheet
            .with_size_hint(
                _info.animation_set.columns as u32 * _info.animation_set.tilesize_x,
                _info.animation_set.rows as u32 * _info.animation_set.tilesize_y,
            )
            .sprite(&mut tools.atlas_layouts);
        let animation_mapping = AnimationMapping(res_animations);
        helper_entity_add(
            &mut tools.commands,
            entity,
            sprite.clone(),
            animation_mapping.clone(),
        );
        tools
            .cache
            .entries
            .insert(entity_name.clone(), (sprite, animation_mapping));
        info!("Animations Applied to entity{:?}", entity);
    }
}

fn helper_entity_add(
    commands: &mut Commands,
    entity: Entity,
    mut sprite: Sprite,
    animation_mapping: AnimationMapping,
) {
    commands
        .entity(entity)
        .insert((sprite, animation_mapping))
        .remove::<PendingAnimationLoad>();
}

fn looping_creations(
    animations: &mut ResMut<Assets<Animation>>,
    _info: &EntityConfig,
    spritesheet: Spritesheet,
    res_animations: &mut HashMap<String, Handle<Animation>>,
) {
    let facing_var = ["_UP", "_RIGHT", "_LEFT", "_DOWN"];
    for (name, sets) in &_info.animation_set.animations {
        for face_idx in 0..4 {
            let animation = spritesheet
                .create_animation()
                .add_partial_row(face_idx, sets.start_frame_ptr..sets.end_frame_ptr)
                .set_duration(AnimationDuration::PerFrame(250))
                .set_repetitions(AnimationRepeat::Loop)
                .build();

            let animation_handle = animations.add(animation);

            let animation_string = format!("{}_{}{}", _info.name, name, facing_var[face_idx]);
            eprint!("Animation Identified: {}\n", animation_string);
            res_animations.insert(animation_string, animation_handle.clone());
        }
    }
}
