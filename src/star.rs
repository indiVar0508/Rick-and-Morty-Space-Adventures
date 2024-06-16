use crate::co_ordinate::CoOrdinate;

pub struct Star {
    pub co_ordinate: CoOrdinate,
}

impl Default for Star {
    fn default() -> Self {
        Self {
            co_ordinate: CoOrdinate::default(),
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
