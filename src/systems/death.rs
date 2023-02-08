use legion::systems::CommandBuffer;

use crate::prelude::*;

#[system(for_each)]
#[read_component(Dead)]
#[read_component(Enemy)]
pub fn enemy_death(entity: &Entity, _enemy: &Enemy, _dead: &Dead, commands: &mut CommandBuffer) {
    commands.remove(*entity);
}

#[system(for_each)]
#[read_component(Dead)]
#[read_component(Player)]
pub fn player_death(
    _ecs: &SubWorld,
    entity: &Entity,
    _player: &Player,
    _dead: &Dead,
    commands: &mut CommandBuffer,
) {
    commands.remove(*entity);
}

#[system(for_each)]
#[read_component(Dead)]
#[read_component(Projectile)]
pub fn projectile_death(
    entity: &Entity,
    _projectile: &Projectile,
    _dead: &Dead,
    commands: &mut CommandBuffer,
) {
    commands.remove(*entity);
}
