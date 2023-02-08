use legion::systems::CommandBuffer;

use crate::prelude::*;

#[system(for_each)]
#[read_component(Parrying)]
pub fn parry_animation(entity: &Entity, parr: &Parrying, commands: &mut CommandBuffer) {
	if parr.timer > 0 {
		commands.add_component(*entity, Render {
			color: ColorPair::new(YELLOW, BLACK),
			glyph: to_cp437('o')
		});
		commands.add_component(*entity, Parrying{timer: parr.timer - 1});
	} else {
		commands.remove_component::<Parrying>(*entity);
	}
}