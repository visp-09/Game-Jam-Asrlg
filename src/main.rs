mod camera;
mod components;
mod map;
mod spawner;
mod systems;

mod prelude {
    pub use crate::world::*;
    pub use bracket_random::prelude::*;
    pub use bracket_terminal::prelude::*;
    pub use legion::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;

    pub const MAP_WIDTH: i32 = 800;
    pub const MAP_HEIGHT: i32 = 500;

    pub const ENEMY_DETECTION_RADIUS: i32 = 20;

    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;

    pub use kira::{
        manager::{backend::cpal::CpalBackend, AudioManager, AudioManagerSettings},
        sound::static_sound::{StaticSoundData, StaticSoundSettings},
    };
}
use std::collections::HashSet;

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    systems: Schedule,
    tick: u64,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let rng = RandomNumberGenerator::new();
        let map = Map::new(rng);
        let used_attack_points: HashSet<Point> = HashSet::new();
        let mut manager =
            AudioManager::<CpalBackend>::new(AudioManagerSettings::default()).unwrap();
        let sound_data =
            StaticSoundData::from_file("data/music.mp3", StaticSoundSettings::default()).unwrap();
        manager.play(sound_data.clone()).unwrap();

        resources.insert(manager);
        resources.insert(Camera::new(map.player_start));
        spawn_player(&mut ecs, map.player_start);
        resources.insert(map);
        resources.insert(used_attack_points);
        Self {
            ecs,
            resources,
            systems: build_scheduler(),
            tick: 0,
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();
        ctx.set_active_console(3);
        ctx.cls();
        let input = INPUT.lock();
        self.resources.insert(input.key_pressed_set().clone());
        self.resources.insert(input.mouse_tile(1));
        self.resources
            .insert(input.mouse_button_pressed_set().clone());
        self.resources.insert(ctx.key);
        self.resources.insert(self.tick);
        self.systems.execute(&mut self.ecs, &mut self.resources);
        self.tick += 1;
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    std::env::set_var("RUST_BACKTRACE", "1");

    let mut context = BTermBuilder::new()
        .with_title("asrlg")
        .with_fps_cap(60.0)
        .with_dimensions(SCREEN_WIDTH, SCREEN_HEIGHT)
        .with_tile_dimensions(8, 8)
        .with_resource_path("resources")
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, "terminal8x8.png")
        .with_simple_console_no_bg(SCREEN_WIDTH, SCREEN_HEIGHT, "terminal8x8.png")
        .with_simple_console_no_bg(SCREEN_WIDTH, SCREEN_HEIGHT, "terminal8x8.png")
        .with_simple_console_no_bg(SCREEN_WIDTH / 4, SCREEN_HEIGHT / 4, "terminal8x8.png")
        //.with_fullscreen(true)
        .build()?;
    context.with_post_scanlines(false);

    let gs: State = State::new();
    main_loop(context, gs)
}
