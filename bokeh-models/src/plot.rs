use super::glyphs::Glyph;
use super::layout::Layout;
use super::tools::Tool;
use super::ColumnDataSource;

pub trait Root {}

#[derive(Default, Clone)]
pub struct Plot {
    pub min_border: Option<i32>,
}

impl Plot {
    pub fn new() -> Plot {
        Plot::default()
    }

    pub fn add_glyph<G>(&mut self, _source: &ColumnDataSource, _glyph: &G)
    where
        G: Glyph + Clone,
    {
    }

    pub fn add_layout<L>(&mut self, _layout: L, _position: &str)
    where
        L: Layout + Clone,
    {
    }

    pub fn add_tool<T>(&mut self, _tool: T)
    where
        T: Tool + Clone,
    {
    }
}

impl Root for Plot {}
