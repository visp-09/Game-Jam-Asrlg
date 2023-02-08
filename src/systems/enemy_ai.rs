use crate::prelude::*;
use legion::systems::CommandBuffer;

#[system]
#[write_component(Enemy)]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(Render)]
pub fn enemy_ai(
	ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &Map,
    ) {
	let mut enemies = <(Entity, &Point, &Render)>::query().filter(component::<Enemy>()  & !component::<HasLineOfSight>());
	let mut players = <&Point>::query().filter(component::<Player>());

	let mut player_pos = Point::zero();

	for posn in players.iter(ecs) {
		player_pos = *posn;
	}

	for (enemy, enemy_pos, render) in enemies.iter(ecs) {
		if player_pos == *enemy_pos {
			continue;
		}
		// line of sight
		let line = VectorLine::new(player_pos, *enemy_pos);
		let mut line_of_sight = true;
		line.for_each(|p| {
			if map.tiles[map_idx(p.x, p.y)] == TileType::Wall {
				line_of_sight = false;
			}
		});

		if !line_of_sight {
			continue;
		}
		// distance check
		let dist_square = (player_pos.x - enemy_pos.x) * (player_pos.x - enemy_pos.x) + (player_pos.y - enemy_pos.y) * (player_pos.y - enemy_pos.y);

		if dist_square > ENEMY_DETECTION_RADIUS * ENEMY_DETECTION_RADIUS {
			continue;
		}

		if line_of_sight {
			commands.add_component(*enemy, HasLineOfSight{});
			commands.add_component(*enemy, Render {
				color: ColorPair::new(RED, BLACK),
				glyph: render.glyph
			});
		}
	}
}