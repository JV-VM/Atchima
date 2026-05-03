use crate::{
    config::components::{
        AnimationMapping, Collider, EntityConfig, Facing, LoadedEntityConfig, PendingAnimationLoad,
        YSort, create_entity_animations,
    },
    game::player::Player,
};
use bevy::{math::ops::abs, prelude::*};
use bevy_spritesheet_animation::prelude::*;
#[derive(Component)]
pub struct Tree;
fn set_entity_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle: Handle<EntityConfig> =
        asset_server.load::<EntityConfig>("RON_files/atchima.setup.ron");
    commands.spawn((
        Player,
        Facing {
            suffix: "_DOWN".into(),
        },
        YSort,
        LoadedEntityConfig(handle),
        PendingAnimationLoad,
        Transform::from_translation(Vec3::ZERO),
        Collider {
            size: Vec2::new(32.0, 32.0),
        },
    ));
}

fn add_entity_animation_idle(
    mut commands: Commands,
    query: Query<(Entity, &AnimationMapping), Without<SpritesheetAnimation>>,
) {
    for (entity, animation_mapping) in query {
        if let Some((_, animation_library_id)) = animation_mapping
            .0
            .iter()
            .find(|(k, _)| k.contains("IDLE_DOWN"))
        {
            commands
                .entity(entity)
                .insert(SpritesheetAnimation::new(animation_library_id.clone()));
        }
    }
}
#[derive(Debug)]
pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, set_entity_resources).add_systems(
            Update,
            (create_entity_animations, add_entity_animation_idle),
        );
    }
}
