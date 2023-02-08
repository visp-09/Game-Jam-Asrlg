use legion::systems::CommandBuffer;

use crate::prelude::*;

#[system(for_each)]
#[read_component(Score)]
#[read_component(Enemy)]

pub fn score_board(ecs: &SubWorld, entity: &Entity, score: &Score, commands: &mut CommandBuffer) {
    let query = <&Enemy>::query().iter(ecs).count();
    if query == 0 {
        commands.add_component(
            *entity,
            Score {
                initial: score.initial + 1,
            },
        );
    }
}
