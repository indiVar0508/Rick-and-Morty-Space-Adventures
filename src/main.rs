use rick_n_morty_space_travel::{WINDOW_HEIGHT, WINDOW_WITDH};
use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    jet_speed: f32
}
fn main() {
    let mut game = Game::new();

    game.window_settings(Window {
        resolution: WindowResolution::new(WINDOW_WITDH, WINDOW_HEIGHT),
        title: "Rick and morty Space Adventures".to_owned(),    // Set the background color
        transparent: true, 
        ..Default::default()
    });

    let sprite = game.add_sprite("space_ship", "sprite/rick_n_morty/spaceship.png");
    // scale down the the space_ship size
    // FIXME: tranparent image breaks pixel.
    sprite.scale = 0.3;
    game.add_logic(space_ship_logic);
    game.run(GameState { jet_speed: 340.0});
}

fn space_ship_logic(engine: &mut Engine, game_state: &mut GameState) {
    engine.show_colliders = true;
    let space_ship = engine.sprites.get_mut("space_ship").unwrap();

    // Handle horizontal traversal
    if engine.keyboard_state.pressed(KeyCode::Right) {
        space_ship.translation.x += game_state.jet_speed * engine.delta_f32;
        // tilt the spaceship to right
        space_ship.rotation = SOUTH_EAST;
    }
    else if engine.keyboard_state.pressed(KeyCode::Left) {
        space_ship.translation.x -= game_state.jet_speed * engine.delta_f32;
        // tilt the spaceship to left
        space_ship.rotation = NORTH_EAST;
    }
    else {
        // if not moving anyways make space_ship horizontal again
        space_ship.rotation = 0.0;
    }

    // veritical traversal
    if engine.keyboard_state.pressed(KeyCode::Up) {
        space_ship.translation.y += game_state.jet_speed * engine.delta_f32;
    }
    else if engine.keyboard_state.pressed(KeyCode::Down){
        space_ship.translation.y -= game_state.jet_speed * engine.delta_f32;
    }

    // Bound the space_ship
    // Fixme: how to get size of sprite?
    const HORIZONTAL_PLAYER_OFFSET: f32 = 50.0;
    const WIDTH_OFFSET: f32 = WINDOW_WITDH / 2.0;
    if space_ship.translation.x > (WIDTH_OFFSET - HORIZONTAL_PLAYER_OFFSET) {
        space_ship.translation.x = WIDTH_OFFSET  - HORIZONTAL_PLAYER_OFFSET;
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
