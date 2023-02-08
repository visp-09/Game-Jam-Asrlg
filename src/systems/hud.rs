use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Score)]
#[read_component(Enemy)]
#[write_component(Timer)]
#[read_component(Timer)]

pub fn hud(ecs: &mut SubWorld, #[resource] tick: &u64, #[resource] map: &mut Map) {
    let mut final_score = 0;
    let mut health_query = <&Health>::query().filter(component::<Player>());
    for player_health in health_query.iter(ecs) {
        let mut draw_batch = DrawBatch::new();

        draw_batch.bar_horizontal(
            Point::zero(),
            SCREEN_WIDTH,
            player_health.current as i32,
            player_health.max as i32,
            ColorPair::new(RED, BLACK),
        );

        draw_batch.print_color_centered(
            0,
            format!(
                " Health: {} / {} ",
                player_health.current, player_health.max
            ),
            ColorPair::new(WHITE, RED),
        );
        draw_batch.submit(10000).expect("Batch error");
    }

    let mut score_query = <&Score>::query();
    for score_item in score_query.iter(ecs) {
        let query = <&Player>::query().iter(ecs).count();
        if query == 1 {
            let mut draw_batch = DrawBatch::new();

            draw_batch.print_color_centered(
                3,
                format!(" Your score is: {}", score_item.initial),
                ColorPair::new(WHITE, RED),
            );
            draw_batch.target(2);
            draw_batch.print_centered(
                1,
                "Explore the ROOTS and fight your way out. WASD keys to move.",
            );
        }
        final_score = score_item.initial;
    }

    let mut draw_batch = DrawBatch::new();

    let query = <&Enemy>::query().iter(ecs).count();
    let time_query = <&mut Timer>::query().iter_mut(ecs).nth(0).unwrap();
    if map.rooms.len() > 2 && query == 0 {
        time_query.time_stamp = tick + 180;
    }

    if time_query.time_stamp > *tick {
        draw_batch.print_color_centered(
            10,
            format!(" ROOT NODE CLEARED!"),
            ColorPair::new(WHITE, RED),
        );
    }

    let query = <&Player>::query().iter(ecs).count();

    if query == 0 {
        let mut draw_batch = DrawBatch::new();
        draw_batch.target(3);
        if final_score <= 3 {
            draw_batch.print_color_centered(
                4,
                format!("Skill Issue :D"),
                ColorPair::new(WHITE, RED),
            );
            draw_batch.print_color_centered(
                5,
                format!("Your score was {}", final_score),
                ColorPair::new(WHITE, RED),
            );
        }
        if final_score > 3 && final_score <= 5 {
            draw_batch.print_color_centered(4, format!("meh :/"), ColorPair::new(WHITE, RED));
            draw_batch.print_color_centered(
                5,
                format!("Your score was {}", final_score),
                ColorPair::new(WHITE, RED),
            );
        }
        if final_score > 5 && final_score <= 8 {
            draw_batch.print_color_centered(4, format!("Not bad! :o"), ColorPair::new(WHITE, RED));
            draw_batch.print_color_centered(
                5,
                format!("Your score was {}", final_score),
                ColorPair::new(WHITE, RED),
            );
        }
        if final_score > 8 && final_score <= 20 {
            draw_batch.print_color_centered(4, format!("Crazy! :O"), ColorPair::new(WHITE, RED));
            draw_batch.print_color_centered(
                5,
                format!("Your score was {}", final_score),
                ColorPair::new(WHITE, RED),
            );
        }
        if final_score > 20 {
            draw_batch.print_color_centered(
                4,
                format!("No social life."),
                ColorPair::new(WHITE, RED),
            );
            draw_batch.print_color_centered(
                5,
                format!("Your score was {}", final_score),
                ColorPair::new(WHITE, RED),
            );
        }
        draw_batch.submit(3).expect("Batch error");
    }

    draw_batch.submit(2).expect("Batch error");
}
