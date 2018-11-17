use super::layout::Layout;

#[derive(Clone)]
pub struct LinearAxis;

impl LinearAxis {
    pub fn new() -> LinearAxis {
        LinearAxis {}
    }
}

impl Layout for LinearAxis {}
