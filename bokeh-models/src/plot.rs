use super::glyphs::Glyph;
use super::layout::Layout;
use super::tools::Tool;
use super::ColumnDataSource;
use std::collections::HashMap;

pub trait Root {}

#[derive(Default)]
pub struct Plot {
    pub min_border: Option<i32>,
    glyphs: Vec<Box<dyn Glyph>>,
    layouts: HashMap<String, Box<dyn Layout>>,
    tools: Vec<Box<dyn Tool>>,
}

impl Plot {
    pub fn new() -> Plot {
        Plot::default()
    }

    pub fn add_glyph<G>(&mut self, _source: &ColumnDataSource, glyph: G)
    where
        G: Glyph + 'static,
    {
        self.glyphs.push(Box::new(glyph));
    }

    pub fn add_layout<L>(&mut self, layout: L, position: &str)
    where
        L: Layout + 'static,
    {
        self.layouts.insert(position.to_string(), Box::new(layout));
    }

    pub fn add_tool<T>(&mut self, tool: T)
    where
        T: Tool + 'static,
    {
        self.tools.push(Box::new(tool));
    }
}

impl Root for Plot {}
