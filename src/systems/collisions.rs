use legion::systems::CommandBuffer;

use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[read_component(Projectile)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());

    let projectiles = <(Entity, &Point, &Projectile)>::query().filter(component::<Projectile>()).iter(ecs).collect::<Vec<(&Entity, &Point, &Projectile)>>();

    // Projectile x Player
    players.for_each(ecs, |(player, pos)| {
        for (proj_entity, proj_pos, proj_comp) in &projectiles {
            if **proj_pos == *pos {
                commands.push((WantsToAttack{ attacker: **proj_entity, victim: *player,  damage: proj_comp.damage},));
                commands.add_component(**proj_entity, Dead{});
            }
        }
    });

    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

    // Projectile x Enemy
    for (proj_entity, proj_pos, proj_comp) in &projectiles {
        enemies
        .iter(ecs)
        .filter(|(_, pos)| **pos == **proj_pos)
        .for_each(|(enemy_entity, _)| {
            commands.push((WantsToAttack{ attacker: **proj_entity, victim: *enemy_entity,  damage: proj_comp.damage},));
            commands.add_component(**proj_entity, Dead{});
        });
    }

    // Player x Enemy
    let player_collection = players.iter(ecs).collect::<Vec<(&Entity, &Point)>>();

    enemies
        .iter(ecs)
        .for_each(|(enemy, enemy_pos)| {
            for (player, player_pos) in &player_collection {
                if **player_pos == *enemy_pos {
                    commands.push((WantsToAttack{ attacker: *enemy, victim: **player, damage: 1.0},));
                    
                    // This is definitely not the best way to do this but 48 hour game jam
                    // I guess this needs to be done whenever we force move the player
                    if let Ok(player_entry_ref) = ecs.entry_ref(**player) {
                        if let Ok(player_component) = player_entry_ref.get_component::<Player>() {
                            let mut player_direction = <(Entity, &Point)>::query().filter(component::<PlayerDirection>());
                            player_direction.for_each(ecs, |(player_dir, player_dir_pos)| {
                                let delta = player_component.last_point - **player_pos;
                                commands.add_component(**player, **player_pos + delta);
                                commands.add_component(*player_dir, *player_dir_pos + delta);
                            })
                        }
                    }
                }
            }
        });
}
