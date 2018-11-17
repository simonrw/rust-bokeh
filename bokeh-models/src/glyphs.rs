pub trait Glyph {}

#[derive(Default, Clone)]
pub struct Circle {
    pub x: Option<String>,
    pub y: Option<String>,
    pub fill_color: Option<String>,
    pub size: Option<u32>,
    pub line_color: Option<String>,
}

impl Circle {
    pub fn new() -> Circle {
        Circle::default()
    }
}

impl Glyph for Circle {}
