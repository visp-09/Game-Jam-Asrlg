use legion::Schedule;

//use self::score_board::score;

mod collisions;
mod combat;
mod death;
mod enemy_ai;
mod entity_render;
mod hud;
mod map_render;
mod movement;
mod new_room;
mod player_camera;
mod player_input;
mod projectile_lifespan;
mod ripple_attack;
mod detonator_attack;
mod score_board;
mod parry_animation;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(movement::movement_system())
        .add_system(enemy_ai::enemy_ai_system())
        .add_system(ripple_attack::ripple_attack_system())
        .add_system(detonator_attack::detonator_attack_system())
        .add_system(collisions::collisions_system())
        .add_system(combat::combat_system())
        .add_system(parry_animation::parry_animation_system())
        .add_system(death::enemy_death_system())
        .add_system(death::player_death_system())
        .add_system(projectile_lifespan::projectile_lifespan_system())
        .add_system(death::projectile_death_system())
        .add_system(player_camera::player_camera_system())
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(new_room::new_room_system())
        .add_system(score_board::score_board_system())
        .add_system(hud::hud_system())
        .build()
}
