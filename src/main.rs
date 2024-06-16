use rand::Rng;
use rick_n_morty_space_travel::{star::Star, TOTAL_STARS, WINDOW_HEIGHT, WINDOW_WITDH};
use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    jet_speed: f32,
    stars: Vec<Star>,
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

    let space_ship_sprite = game.add_sprite("space_ship", "sprite/rick_n_morty/spaceship.png");
    // scale down the the space_ship size
    // FIXME: tranparent image breaks pixel.
    space_ship_sprite.scale = 0.3;
    // Don't let stars run over spaceship
    space_ship_sprite.layer = 2.0;
    game.add_logic(space_ship_logic);
    game.add_logic(star_logic);
    game.run(GameState {
        jet_speed: 340.0,
        stars,
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
