use std::process::exit;

use bevy::render::render_resource::encase::rts_array::Length;
use rand::Rng;
use rick_n_morty_space_travel::{
    astroid::Astroid, star::Star, TOTAL_ASTROIDS, TOTAL_STARS, WINDOW_HEIGHT, WINDOW_WITDH,
};
use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    jet_speed: f32,
    stars: Vec<Star>,
    astroids: Vec<Astroid>,
}

fn main() {
    let mut game = Game::new();

    game.window_settings(Window {
        resolution: WindowResolution::new(WINDOW_WITDH, WINDOW_HEIGHT),
        title: "Rick and morty Space Adventures".to_owned(), // Set the background color
        transparent: true,
        ..Default::default()
    });

    let mut stars: Vec<Star> = vec![];
    for i in 0..TOTAL_STARS {
        let star: Star = Star::new();
        let star_sprite = game.add_sprite(format!("star_{}", i), "sprite/rick_n_morty/star.png");
        star_sprite.scale = 0.009;
        // 0.005 -> 0.01
        star_sprite.translation.x = star.co_ordinate.x;
        star_sprite.translation.y = star.co_ordinate.y;
        stars.push(star);
    }

    let mut astroids: Vec<Astroid> = vec![];
    let img_options: Vec<&str> = vec![
        "sprite/rick_n_morty/astroid_zero.png",
        "sprite/rick_n_morty/astroid_one.png",
        "sprite/rick_n_morty/astroid_zero_rotated.png",
        "sprite/rick_n_morty/astroid_one_rotated.png",
    ];
    for i in 0..TOTAL_ASTROIDS {
        let img_path = img_options[i % img_options.length()];
        let astroid_sprite = game.add_sprite(format!("astroid_{}", i), img_path);
        astroid_sprite.layer = 1.0;
        astroid_sprite.scale = 0.1;
        astroid_sprite.collision = true;
        // At time of creation everything is hidden
        astroid_sprite.translation.x = WINDOW_HEIGHT * 5.0;
        astroids.push(Astroid::new());
    }

    let space_ship_sprite = game.add_sprite("space_ship", "sprite/rick_n_morty/spaceship.png");
    // scale down the the space_ship size
    // FIXME: tranparent image breaks pixel.
    space_ship_sprite.scale = 0.3;
    // Don't let stars run over spaceship
    space_ship_sprite.layer = 2.0;
    space_ship_sprite.collision = true;
    game.add_logic(space_ship_logic);
    game.add_logic(star_logic);
    game.add_logic(astroid_logic);
    game.add_logic(collision_logic);
    game.run(GameState {
        jet_speed: 340.0,
        stars,
        astroids,
    });
}

fn star_logic(engine: &mut Engine, game_state: &mut GameState) {
    let mut rng = rand::thread_rng();
    // Move the stars
    // FIXME: Ideally it would be better to have space_ship POV for star origin,
    //        but for some reason not able to resolve immutable thing, plus need some thinking
    // Alternatively, instead of moving ship maybe translate the entire screen which might give
    // better feel and spaceship is always in center!?
    for i in 0..TOTAL_STARS {
        let star_stripe = engine.sprites.get_mut(&format!("star_{}", i)).unwrap();
        star_stripe.translation.x = (game_state.stars[i].co_ordinate.x
            / game_state.stars[i].co_ordinate.z)
            * (WINDOW_WITDH / 2.0);
        star_stripe.translation.y = (game_state.stars[i].co_ordinate.y
            / game_state.stars[i].co_ordinate.z)
            * (WINDOW_HEIGHT / 2.0);
        game_state.stars[i].co_ordinate.z -= rng.gen_range(0.0..10.0);
        if game_state.stars[i].co_ordinate.z < 1.0 {
            game_state.stars[i] = Star::new();
        }
    }
}

fn astroid_logic(engine: &mut Engine, game_state: &mut GameState) {
    let mut rng = rand::thread_rng();
    let chance: f32 = rng.gen_range(0.0..1.0);
    if chance < 0.01 {
        // 1% chance of seeing a astroid
        for (i, astroid) in game_state.astroids.iter_mut().enumerate() {
            if astroid.visible == false {
                astroid.visible = true;
                let astroid_sprite = engine.sprites.get_mut(&format!("astroid_{}", i)).unwrap();
                astroid_sprite.translation.x = astroid.co_ordinate.x;
                astroid_sprite.translation.y = astroid.co_ordinate.y;
                break;
            }
        }
    }
    for i in 0..game_state.astroids.length() {
        if game_state.astroids[i].visible == true {
            let astroid_sprite = engine.sprites.get_mut(&format!("astroid_{}", i)).unwrap();
            astroid_sprite.scale += 0.1 * engine.delta_f32;
            if astroid_sprite.scale > 1.0 {
                // it's past space_ship to make it disappear
                astroid_sprite.translation.x = WINDOW_WITDH * 5.0;
                astroid_sprite.scale = 0.1;
                game_state.astroids[i] = Astroid::new();
            }
        }
    }
    // handle if two astroids collide
    for event in engine.collision_events.drain(..) {
        match event.state {
            CollisionState::Begin => {
                // Move either of them away
                if event.pair.0.starts_with("astroid") && event.pair.1.starts_with("astroid") {
                    let astroid_sprite = engine.sprites.get_mut(&event.pair.0).unwrap();
                    astroid_sprite.translation.x = WINDOW_WITDH * 5.0;
                    astroid_sprite.scale = 0.1;
                }
            }
            CollisionState::End => {}
        }
    }
}

fn space_ship_logic(engine: &mut Engine, game_state: &mut GameState) {
    // engine.show_colliders = true;
    let space_ship = engine.sprites.get_mut("space_ship").unwrap();

    // Handle horizontal traversal
    if engine.keyboard_state.pressed(KeyCode::Right) {
        space_ship.translation.x += game_state.jet_speed * engine.delta_f32;
        // tilt the spaceship to right
        space_ship.rotation = SOUTH_EAST;
    } else if engine.keyboard_state.pressed(KeyCode::Left) {
        space_ship.translation.x -= game_state.jet_speed * engine.delta_f32;
        // tilt the spaceship to left
        space_ship.rotation = NORTH_EAST;
    } else {
        // if not moving anyways make space_ship horizontal again
        space_ship.rotation = 0.0;
    }

    // veritical traversal
    if engine.keyboard_state.pressed(KeyCode::Up) {
        space_ship.translation.y += game_state.jet_speed * engine.delta_f32;
    } else if engine.keyboard_state.pressed(KeyCode::Down) {
        space_ship.translation.y -= game_state.jet_speed * engine.delta_f32;
    }

    // Bound the space_ship
    // Fixme: how to get size of sprite?
    const HORIZONTAL_PLAYER_OFFSET: f32 = 50.0;
    const WIDTH_OFFSET: f32 = WINDOW_WITDH / 2.0;
    if space_ship.translation.x > (WIDTH_OFFSET - HORIZONTAL_PLAYER_OFFSET) {
        space_ship.translation.x = WIDTH_OFFSET - HORIZONTAL_PLAYER_OFFSET;
    } else if space_ship.translation.x < -(WIDTH_OFFSET - HORIZONTAL_PLAYER_OFFSET) {
        space_ship.translation.x = -WIDTH_OFFSET + HORIZONTAL_PLAYER_OFFSET;
    }

    // Fixme: how to get size of sprite?
    const VERTICAL_PLAYER_OFFSET: f32 = 25.0;
    const HEIGHT_OFFSET: f32 = WINDOW_HEIGHT / 2.0;
    if space_ship.translation.y > (HEIGHT_OFFSET - VERTICAL_PLAYER_OFFSET) {
        space_ship.translation.y = HEIGHT_OFFSET - VERTICAL_PLAYER_OFFSET;
    } else if space_ship.translation.y < -(HEIGHT_OFFSET - VERTICAL_PLAYER_OFFSET - 30.0) {
        space_ship.translation.y = -HEIGHT_OFFSET + VERTICAL_PLAYER_OFFSET + 30.0;
    }
}

fn collision_logic(engine: &mut Engine, game_state: &mut GameState) {
    let mut game_over = false;
    for i in 0..game_state.astroids.length() {
        let astroid_sprite = engine.sprites.get_mut(&format!("astroid_{}", i)).unwrap();
        if game_state.astroids[i].visible && astroid_sprite.scale > 0.85 {
            for event in engine.collision_events.drain(..) {
                match event.state {
                    CollisionState::Begin => {
                        if event.pair.0 == "space_ship" || event.pair.1 == "space_ship" {
                            // handle spaceship collision
                            let astroid_name: &str;
                            if event.pair.0 == "space_ship" {
                                astroid_name = &event.pair.1;
                            } else {
                                astroid_name = &event.pair.0;
                            }
                            // let space_ship_sprite = engine.sprites.get_mut("space_ship").unwrap();
                            let astroid_sprite = engine.sprites.get_mut(astroid_name).unwrap();
                            astroid_sprite.collider.points();
                            // println!("{} {}", astroid_name, astroid_sprite.scale);
                            if astroid_sprite.scale > 0.8 {
                                // to keep 3 d affect check for scale of astroid to be greater than 0.95
                                // if scale is more than 0.9 then this is a collision.
                                // This is a collision
                                // FIXME: need to figure out a way for consistent collision
                                // ref: https://github.com/CleanCut/rusty_engine/issues/71
                                // println!("stopppp!!!");
                                game_over = true;
                            }
                        }
                    }
                    CollisionState::End => {}
                }
            }
        }
    }
    if game_over == true {
        // println!("stopping");
        let game_over_text = engine.add_text("game_over", "You Lost!");
        game_over_text.font_size = 128.0;
        engine.audio_manager.play_sfx(SfxPreset::Jingle3, 0.5);
        exit(0);
    }
}
