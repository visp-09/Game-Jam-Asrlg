use legion::systems::CommandBuffer;

use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(
    ecs: &mut SubWorld, 
    commands: &mut CommandBuffer
) {
    let attack_messages = <(Entity, &WantsToAttack)>::query()
        .iter(ecs)
        .map(|(message_entity, message)|{
            (*message_entity, message.attacker, message.victim, message.damage)
        })
        .collect::<Vec<(Entity, Entity, Entity, f32)>>();
        
    for (message_entity, _ , victim, damage) in attack_messages
    {
        if let Err(_) = ecs.entry_ref(victim) {
            commands.remove(message_entity);
            continue;
        }

        if let Ok(mut health) =  ecs
        .entry_mut(victim)
        .unwrap()
        .get_component_mut::<Health>()
        {
            health.current -= damage;
            if health.current < 1.0 {
                commands.add_component(victim, Dead{});
                //commands.remove(victim);
            }
        }
        commands.remove(message_entity);
    }
}
