use crate::co_ordinate::CoOrdinate;

pub struct Astroid {
    pub co_ordinate: CoOrdinate,
    pub visible: bool,
}

impl Default for Astroid {
    fn default() -> Self {
        Self {
            co_ordinate: Default::default(),
            visible: false,
        }
    }
}

impl Astroid {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
