use std::collections::HashSet;

use crate::prelude::*;
use bracket_geometry::prelude::*;
use legion::systems::CommandBuffer;
use rand::{distributions::Uniform, prelude::*};

#[system(for_each)]
pub fn detonator_attack(
    _line_of_sight: &HasLineOfSight,
    detonator_ai: &mut DetonatorAI,
    pos: &Point,
    commands: &mut CommandBuffer,
    #[resource] tick: &u64,
    #[resource] used_attack_points: &mut HashSet<Point>,
) {
    if *tick % detonator_ai.interval as u64 == 0 {
        let top_left = *pos
            - Point::new(
                (detonator_ai.dimensions.x * detonator_ai.tile_length) / 2,
                (detonator_ai.dimensions.y * detonator_ai.tile_length) / 2,
            );
        let mut rng = rand::thread_rng();
        let max_attack_range = Uniform::from(
            (detonator_ai.dimensions.x * detonator_ai.dimensions.y) / 4
                ..detonator_ai.dimensions.x * detonator_ai.dimensions.y,
        );
        let max_x = Uniform::from(0..detonator_ai.dimensions.x);
        let max_y = Uniform::from(0..detonator_ai.dimensions.y);
        let max_attack_squares = max_attack_range.sample(&mut rng);
        let mut attack_points: Vec<Point> = Vec::new();
        for _square in 0..max_attack_squares {
            let x = max_x.sample(&mut rng);
            let y = max_y.sample(&mut rng);
            let current_attack_point = Point::new(x, y);
            if !used_attack_points.contains(&current_attack_point) {
                attack_points.push(current_attack_point);
            }
        }
        used_attack_points.clear();
        for new_attack_point in attack_points {
            used_attack_points.insert(new_attack_point);
            println!("{:?}", new_attack_point);
        }
        println!("");
        for attack_point in used_attack_points.iter() {
            let start = top_left + *attack_point * detonator_ai.tile_length;
            for i in 0..detonator_ai.tile_length {
                for j in 0..detonator_ai.tile_length {
                    let curr_point = start + Point::new(i, j);
                    if curr_point != *pos {
                        spawn_projectile(
                            &curr_point,
                            commands,
                            detonator_ai.interval as i32,
                            2.0,
                            Render {
                                color: ColorPair::new(ORANGE1, BLACK),
                                glyph: to_cp437('+'),
                            },
                        );
                    }
                }
            }
        }
    }
}
