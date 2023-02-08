use legion::systems::CommandBuffer;

use crate::prelude::*;

#[system(for_each)]
#[read_component(WantsToMove)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &Map,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.destination) {
        commands.add_component(*entity, want_move.destination);
        commands.remove_component::<WantsToMove>(*entity);
    }
}
