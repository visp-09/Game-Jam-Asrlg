use crate::prelude::*;
use legion::systems::CommandBuffer;

#[system]
#[read_component(Enemy)]

pub fn new_room(ecs: &SubWorld, commands: &mut CommandBuffer, #[resource] map: &mut Map) {
    let query = <&Enemy>::query().iter(ecs).count();
    if query == 0 {
        map.build_random_room();
        let v = map.get_position_of_enemies();
        spawn_entities(commands, &mut map.rng, &v);
    }
}
