use self::template::Templates;
use legion::systems::CommandBuffer;

use crate::prelude::*;

mod template;

pub fn spawn_player(world: &mut World, pos: Point) {
    world.push((
        Player { last_point: pos },
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 50.0,
            max: 50.0,
        },
        PlayerAttackType::RightFront,
        PlayerAttackStatus {
            stage: PlayerAttackStage::Ready,
            stage_timer: 0,
        },
        PlayerParryCooldown { timestamp: 0 },
    ));
    world.push((Timer { time_stamp: 0 },));
    world.push((Score { initial: -1 },));
    world.push((PlayerDirection,));
}

pub fn spawn_entities(
    commands: &mut CommandBuffer,
    rng: &mut RandomNumberGenerator,
    spawn_points: &[Point],
) {
    let templates = Templates::load();
    templates.spawn_entities(commands, rng, spawn_points);
}

pub fn spawn_projectile(
    pt: &Point,
    commands: &mut CommandBuffer,
    lifetime: i32,
    damage: f32,
    rend: Render,
) {
    let entity = commands.push((pt.clone(), rend));
    commands.add_component(entity, Projectile { lifetime, damage });
}
