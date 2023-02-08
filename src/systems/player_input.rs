use std::collections::HashSet;

use legion::{systems::CommandBuffer, world::SubWorld};

use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
#[read_component(Score)]
#[write_component(Projectile)]
#[read_component(PlayerParryCooldown)]
#[read_component(PlayerAttackType)]
#[read_component(PlayerAttackStatus)]

pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] keys: &HashSet<VirtualKeyCode>,
    #[resource] mouse_tile: &Point,
    #[resource] mouse_keys: &HashSet<usize>,
    #[resource] tick: &u64,
    #[resource] camera: &Camera,
    #[resource] map: &Map,
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());

    let mut direction_delta = Point::new(0, 0);
    // Direction
    let mut player_directions = <Entity>::query().filter(component::<PlayerDirection>());
    for (_, player_pos) in players.iter(ecs) {
        let camera_offset = Point::new(camera.left_x, camera.top_y);
        let (direction_pos, direction_char) =
            direction(&(*mouse_tile - (*player_pos - camera_offset)));
        for player_direction in player_directions.iter(ecs) {
            commands.add_component(*player_direction, direction_pos + *player_pos);
            commands.add_component(
                *player_direction,
                Render {
                    color: ColorPair::new(WHITE, BLACK),
                    glyph: direction_char,
                },
            );
        }
        direction_delta = direction_pos;
    }

    // Attack
    let mut next_attack_status = PlayerAttackStatus {
        stage: PlayerAttackStage::Ready,
        stage_timer: 0,
    };

    let mut player_attack_statuses =
        <(Entity, &PlayerAttackStatus, &PlayerAttackType, &Point)>::query()
            .filter(component::<Player>());
    for (player, attack_status, attack_type, player_pos) in player_attack_statuses.iter(ecs) {
        if attack_status.stage_timer > 0 {
            next_attack_status = PlayerAttackStatus {
                stage: attack_status.stage,
                stage_timer: attack_status.stage_timer - 1,
            };
        } else {
            match attack_status.stage {
                PlayerAttackStage::Ready => {
                    if mouse_keys.contains(&0) {
                        next_attack_status = PlayerAttackStatus {
                            stage: PlayerAttackStage::Stage1,
                            stage_timer: 2,
                        };
                    }
                }
                PlayerAttackStage::Stage1 => {
                    next_attack_status = PlayerAttackStatus {
                        stage: PlayerAttackStage::Stage2,
                        stage_timer: 3,
                    };
                }
                PlayerAttackStage::Stage2 => {
                    next_attack_status = PlayerAttackStatus {
                        stage: PlayerAttackStage::Stage3,
                        stage_timer: 4,
                    };
                }
                PlayerAttackStage::Stage3 => {
                    next_attack_status = PlayerAttackStatus {
                        stage: PlayerAttackStage::Stage4,
                        stage_timer: 5,
                    };
                }
                PlayerAttackStage::Stage4 => {
                    next_attack_status = PlayerAttackStatus {
                        stage: PlayerAttackStage::Ready,
                        stage_timer: 50,
                    };
                    match attack_type {
                        PlayerAttackType::LeftFront => {
                            commands.add_component(*player, PlayerAttackType::RightFront)
                        }
                        PlayerAttackType::RightFront => {
                            commands.add_component(*player, PlayerAttackType::LeftFront)
                        }
                    }
                }
            }
            if next_attack_status.stage != PlayerAttackStage::Ready {
                let attack_pos = *player_pos
                    + attack_delta(&direction_delta, *attack_type, next_attack_status.stage);
                spawn_projectile(
                    &attack_pos,
                    commands,
                    2,
                    0.75,
                    Render {
                        color: ColorPair::new(TURQUOISE1, BLACK),
                        glyph: to_cp437('*'),
                    },
                );
            } else if mouse_keys.contains(&1) {
                if let Ok(parry_cooldown) = ecs
                    .entry_ref(*player)
                    .unwrap()
                    .get_component::<PlayerParryCooldown>()
                {
                    if parry_cooldown.timestamp <= *tick {
                        commands.add_component(
                            *player,
                            PlayerParryCooldown {
                                timestamp: *tick + 90,
                            },
                        );

                        let mut projectiles =
                            <(Entity, &Point)>::query().filter(component::<Projectile>());
                        projectiles
                            .iter(ecs)
                            .filter(|(_, pos)| **pos == *player_pos + direction_delta)
                            .for_each(|(entity, _)| {
                                commands.remove_component::<Projectile>(*entity);
                                commands.remove(*entity);
                                println!("DELETED PROJECTILE");
                                if let Ok(health) =
                                    ecs.entry_ref(*player).unwrap().get_component::<Health>()
                                {
                                    commands.add_component(
                                        *player,
                                        Health {
                                            max: health.max,
                                            current: health.current + 2.0,
                                        },
                                    );
                                }
                            });

                        let mut player_direction =
                            <Entity>::query().filter(component::<PlayerDirection>());
                        player_direction.for_each(ecs, |player_dir| {
                            commands.add_component(*player_dir, Parrying { timer: 10 });
                        });
                        if let Ok(health) =
                            ecs.entry_ref(*player).unwrap().get_component::<Health>()
                        {
                            commands.add_component(
                                *player,
                                Health {
                                    max: health.max,
                                    current: health.current + 2.0,
                                },
                            );
                        }
                    }
                }
            }
        }
        commands.add_component(*player, next_attack_status);
    }

    // Movement
    if !keys.is_empty() && tick % 4 == 0 && next_attack_status.stage == PlayerAttackStage::Ready {
        let delta = move_delta(keys);
        if delta.x != 0 || delta.y != 0 {
            players.iter(ecs).for_each(|(entity, pos)| {
                let destination = *pos + delta;
                if map.can_enter_tile(destination) {
                    commands.add_component(*entity, destination);
                    for player_direction in player_directions.iter(ecs) {
                        commands.add_component(*player_direction, destination + direction_delta);
                    }
                }
            })
        }
    }
}

fn single_move_delta(key: VirtualKeyCode) -> Point {
    match key {
        VirtualKeyCode::A => Point::new(-1, 0),
        VirtualKeyCode::D => Point::new(1, 0),
        VirtualKeyCode::W => Point::new(0, -1),
        VirtualKeyCode::S => Point::new(0, 1),
        _ => Point::new(0, 0),
    }
}

fn move_delta(keys: &HashSet<VirtualKeyCode>) -> Point {
    let mut delta = Point::new(0, 0);
    for key in keys {
        delta += single_move_delta(*key);
    }
    delta
}

fn direction(mouse_vector: &Point) -> (Point, FontCharType) {
    let mouse_vec2 = mouse_vector.to_vec2().normalized();

    if mouse_vec2.x >= 0.707106781187 {
        (Point { x: 1, y: 0 }, to_cp437('>'))
    } else if mouse_vec2.y <= -0.707106781187 {
        (Point { x: 0, y: -1 }, to_cp437('^'))
    } else if mouse_vec2.y >= 0.707106781187 {
        (Point { x: 0, y: 1 }, to_cp437('v'))
    } else {
        (Point { x: -1, y: 0 }, to_cp437('<'))
    }
}

fn attack_delta(
    direction_delta: &Point,
    attack_type: PlayerAttackType,
    attack_status: PlayerAttackStage,
) -> Point {
    match attack_type {
        PlayerAttackType::RightFront => {
            if direction_delta.x == 1 && direction_delta.y == 0 {
                match attack_status {
                    PlayerAttackStage::Stage1 => Point::new(0, 1),
                    PlayerAttackStage::Stage2 => Point::new(1, 1),
                    PlayerAttackStage::Stage3 => Point::new(1, 0),
                    PlayerAttackStage::Stage4 => Point::new(1, -1),
                    PlayerAttackStage::Ready => unimplemented!(),
                }
            } else if direction_delta.x == 0 && direction_delta.y == -1 {
                match attack_status {
                    PlayerAttackStage::Stage1 => Point::new(1, 0),
                    PlayerAttackStage::Stage2 => Point::new(1, -1),
                    PlayerAttackStage::Stage3 => Point::new(0, -1),
                    PlayerAttackStage::Stage4 => Point::new(-1, -1),
                    PlayerAttackStage::Ready => unimplemented!(),
                }
            } else if direction_delta.x == 0 && direction_delta.y == 1 {
                match attack_status {
                    PlayerAttackStage::Stage1 => Point::new(-1, 0),
                    PlayerAttackStage::Stage2 => Point::new(-1, 1),
                    PlayerAttackStage::Stage3 => Point::new(0, 1),
                    PlayerAttackStage::Stage4 => Point::new(1, 1),
                    PlayerAttackStage::Ready => unimplemented!(),
                }
            } else {
                match attack_status {
                    PlayerAttackStage::Stage1 => Point::new(0, -1),
                    PlayerAttackStage::Stage2 => Point::new(-1, -1),
                    PlayerAttackStage::Stage3 => Point::new(-1, 0),
                    PlayerAttackStage::Stage4 => Point::new(-1, 1),
                    PlayerAttackStage::Ready => unimplemented!(),
                }
            }
        }
        PlayerAttackType::LeftFront => {
            if direction_delta.x == 1 && direction_delta.y == 0 {
                match attack_status {
                    PlayerAttackStage::Stage1 => Point::new(0, -1),
                    PlayerAttackStage::Stage2 => Point::new(1, -1),
                    PlayerAttackStage::Stage3 => Point::new(1, 0),
                    PlayerAttackStage::Stage4 => Point::new(1, 1),
                    PlayerAttackStage::Ready => unimplemented!(),
                }
            } else if direction_delta.x == 0 && direction_delta.y == -1 {
                match attack_status {
                    PlayerAttackStage::Stage1 => Point::new(-1, 0),
                    PlayerAttackStage::Stage2 => Point::new(-1, -1),
                    PlayerAttackStage::Stage3 => Point::new(0, -1),
                    PlayerAttackStage::Stage4 => Point::new(1, -1),
                    PlayerAttackStage::Ready => unimplemented!(),
                }
            } else if direction_delta.x == 0 && direction_delta.y == 1 {
                match attack_status {
                    PlayerAttackStage::Stage1 => Point::new(1, 0),
                    PlayerAttackStage::Stage2 => Point::new(1, 1),
                    PlayerAttackStage::Stage3 => Point::new(0, 1),
                    PlayerAttackStage::Stage4 => Point::new(-1, 1),
                    PlayerAttackStage::Ready => unimplemented!(),
                }
            } else {
                match attack_status {
                    PlayerAttackStage::Stage1 => Point::new(0, 1),
                    PlayerAttackStage::Stage2 => Point::new(-1, 1),
                    PlayerAttackStage::Stage3 => Point::new(-1, 0),
                    PlayerAttackStage::Stage4 => Point::new(-1, -1),
                    PlayerAttackStage::Ready => unimplemented!(),
                }
            }
        }
    }
}
