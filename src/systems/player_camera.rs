use crate::prelude::*;

#[system(for_each)]
pub fn player_camera(player: &mut Player, pos: &Point, #[resource] camera: &mut Camera) {
    if *pos != player.last_point {
        camera.on_player_move(*pos);
        player.last_point = *pos;
    }
}
