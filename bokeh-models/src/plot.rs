use super::glyphs::Glyph;
use super::idgen::create_id;
use super::layout::Layout;
use super::tools::Tool;
use super::ColumnDataSource;
use std::collections::HashMap;

pub trait Root {
    fn id(&self) -> i32;
}

#[derive(Default)]
pub struct Plot {
    pub min_border: Option<i32>,
    glyphs: Vec<Box<dyn Glyph>>,
    layouts: HashMap<String, Box<dyn Layout>>,
    tools: Vec<Box<dyn Tool>>,
    id: i32,
}

impl Plot {
    pub fn new() -> Plot {
        let id = create_id();
        Plot::with_id(id)
    }

    pub(crate) fn with_id(id: i32) -> Self {
        let mut plot = Plot::default();
        plot.id = id;
        plot
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

impl Root for Plot {
    fn id(&self) -> i32 {
        self.id
    }
}
