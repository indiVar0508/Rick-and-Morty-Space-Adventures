use rand::Rng;

use crate::{WINDOW_HEIGHT, WINDOW_WITDH};

pub struct Star {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for Star {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(-(WINDOW_WITDH / 2.0)..(WINDOW_WITDH / 2.0)),
            y: rng.gen_range(-(WINDOW_HEIGHT / 2.0)..(WINDOW_HEIGHT / 2.0)),
            z: rng.gen_range((WINDOW_WITDH / 5.0)..(WINDOW_WITDH / 2.0)),
        }
    }
}

impl Star {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
