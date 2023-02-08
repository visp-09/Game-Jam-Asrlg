use crate::prelude::*;
use bracket_geometry::prelude::*;
use legion::systems::CommandBuffer;

#[system(for_each)]
pub fn ripple_attack(
    _line_of_sight: &HasLineOfSight,
    ripple_ai: &mut RippleAI,
    pos: &Point,
    commands: &mut CommandBuffer,
    #[resource] tick: &u64,
) {
    if ripple_ai.timestamp <= *tick {
        ripple_ai.current_radius += 1;
        if ripple_ai.current_radius > ripple_ai.max_radius {
            ripple_ai.current_radius = 1;
        }

        for point in BresenhamCircle::new(*pos, ripple_ai.current_radius).into_iter() {
            spawn_projectile(
                &point,
                commands,
                ripple_ai.interval as i32,
                2.0,
                Render {
                    color: ColorPair::new(ORANGE1, BLACK),
                    glyph: to_cp437('+'),
                },
            );
        }

        ripple_ai.timestamp += ripple_ai.interval as u64;
    }
}
