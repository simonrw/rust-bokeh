//! Rust implementation of the client-side portion of Bokeh.

#![deny(missing_docs)]

use failure::format_err;
use serde_json::{json, to_string, Value};
use std::collections::HashMap;

type Result<T> = std::result::Result<T, failure::Error>;

/// Trait encoding the ability to transform the type into their Bokeh representation
pub trait ToBokeh {
    /// Compulsory method for converting Bokeh model into serializable JSON
    ///
    /// This must be implemented by any struct that is to be converted to Bokeh type, and sent to
    /// BokehJS in the browser
    fn as_bokeh_value(&self) -> Value;

    /// Convert a bokeh struct to string
    ///
    /// Automatically implemented for objects based on their `ToBokeh::as_bokeh_value`
    /// implementation.
    fn as_string(&self) -> serde_json::Result<String> {
        to_string(&ToBokeh::as_bokeh_value(self))
    }
}

// Marker traits

/// Marker trait representing glyphs
pub trait Glyph {}

// ColumnDataSource

/// Column data source for handling columar data
pub struct ColumnDataSource {
    columns: HashMap<String, Vec<f64>>,
}

impl ColumnDataSource {
    /// Create a new default column data source
    pub fn new() -> Self {
        ColumnDataSource {
            columns: HashMap::new(),
        }
    }

    /// Add a column to the data source
    pub fn add<S>(&mut self, key: S, values: &[f64])
    where
        S: Into<String>,
    {
        self.columns.insert(key.into(), values.to_vec());
    }
}

// Plot

/// Position for layout
#[derive(Eq, PartialEq, Hash)]
pub enum Position {
    #[doc(hidden)]
    Below,
    #[doc(hidden)]
    Left,
    #[doc(hidden)]
    Right,
    #[doc(hidden)]
    Above,
}

/// A plot object
pub struct Plot<'s> {
    /// Minimum border width
    pub min_border: Option<u32>,
    source: Option<&'s ColumnDataSource>,
    glyphs: Vec<Box<Glyph>>,
    layouts: HashMap<Position, Layout>,
    tools: Vec<Tool>,
}

impl<'s> Plot<'s> {
    /// Create a new empty plot
    pub fn new() -> Self {
        Plot {
            min_border: None,
            source: None,
            glyphs: Vec::new(),
            layouts: HashMap::new(),
            tools: Vec::new(),
        }
    }

    /// Add a glyph to the plot
    pub fn add_glyph<G>(&mut self, source: &'s ColumnDataSource, glyph: G)
    where
        G: Glyph + 'static,
    {
        self.source = Some(source);
        self.glyphs.push(Box::new(glyph));
    }

    /// Add a layout to the plot
    pub fn add_layout(&mut self, position: Position, layout: Layout) {
        self.layouts.insert(position, layout);
    }

    /// Add a tool to the plot
    pub fn add_tool(&mut self, tool: Tool) {
        self.tools.push(tool);
    }
}

// Circle

/// Circle marker
#[derive(Default)]
pub struct Circle {
    /// X key to extract from ColumnDataSource
    pub x: Option<String>,
    /// Y key to extract from ColumnDataSource
    pub y: Option<String>,
    /// fill color key to extract from ColumnDataSource
    pub fill_color: Option<String>,
    /// size key to extract from ColumnDataSource
    pub size: Option<u32>,
    /// line color key to extract from ColumnDataSource
    pub line_color: Option<String>,
}

impl Circle {
    /// Create a new circle marker representation
    pub fn new() -> Self {
        Circle::default()
    }
}

impl Glyph for Circle {}

// Layout

/// All of the enumerated layout options
pub enum Layout {
    /// Linear range
    LinearAxis,
}

// Tools

/// Tools for the plot
pub enum Tool {
    /// Allow the plot to pan
    PanTool,
    /// Zoom in and out with the mouse wheel
    WheelZoomTool,
}

// BasicTicker

/// Struct representing ticks
pub struct BasicTicker;

impl BasicTicker {
    /// Create a new BasicTicker
    pub fn new() -> BasicTicker {
        BasicTicker {}
    }
}

impl ToBokeh for BasicTicker {
    fn as_bokeh_value(&self) -> Value {
        json!({
            "attributes": {},
            "type": "BasicTicker",
        })
    }
}

// Basic tick formatter

/// Struct dealing with basic tick formatting.
pub struct BasicTickFormatter;

impl BasicTickFormatter {
    /// Create a new BasicTickFormatter
    pub fn new() -> BasicTickFormatter {
        BasicTickFormatter {}
    }
}

impl ToBokeh for BasicTickFormatter {
    fn as_bokeh_value(&self) -> Value {
        json!({
            "attributes": {},
            "type": "BasicTickFormatter",
        })
    }
}

// Document

/// Main document object for the plot
pub struct Document<'s> {
    plot: Option<Plot<'s>>,
}

impl<'s> Document<'s> {
    /// Create a new document
    pub fn new() -> Self {
        Document { plot: None }
    }

    /// Add the root plot to the document
    pub fn add_root(&mut self, plot: Plot<'s>) {
        self.plot = Some(plot);
    }

    /// Check the document is sane
    pub fn validate(self) -> Result<ValidatedDocument<'s>> {
        let plot = self.plot.ok_or(format_err!("document requires a plot"))?;

        Ok(ValidatedDocument { plot })
    }
}

/// Represents a valid document
pub struct ValidatedDocument<'s> {
    plot: Plot<'s>,
}

impl<'s> ValidatedDocument<'s> {
    /// Get the references of all sub-objects to put into the JSON graph
    pub fn references(&self) -> Vec<Box<ToBokeh>> {
        let mut out = Vec::new();
        out
    }
}

/// Write a document to a file at path `path`
pub fn file_html<S>(_doc: &Document, _title: S) -> Result<String>
where
    S: Into<String>,
{
    unimplemented!()
}

/// Return the JSON representation as a serde_json::Value
pub fn to_bokeh_json<S>(doc: &ValidatedDocument, title: S) -> Result<Value>
where
    S: Into<String>,
{
    let references: Vec<Value> = doc
        .references()
        .into_iter()
        .map(|r| r.as_bokeh_value())
        .collect();

    let out = json!({
        "roots": {
            "references": references,
        },
        "title": title.into(),
        "version": "1.0.3",
    });
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_without_id_equal {
        ($left:expr, $right:expr) => {
            assert!($left.is_object(), "Left value {} is not an object", $left);
            assert!(
                $right.is_object(),
                "Right value {} is not an object",
                $right
            );

            let lo = $left.as_object().unwrap();
            let ro = $right.as_object().unwrap().clone();

            for key in ro.keys() {
                assert!(lo.contains_key(key), "Key `{}` missing from output", key);

                let lvalue = &lo[key];
                let rvalue = &ro[key];

                assert_eq!(lvalue, rvalue);
            }
        };
    }

    // TODO: test ids somehow

    /*
    #[test]
    fn test_basic_tick_formatter() {
        let tf = BasicTickFormatter::new();
        let json_value: Value = session.serialize(&tf).unwrap();

        assert_without_id_equal!(
            json_value,
            json!({
                "attributes": {},
                "type": "BasicTickFormatter",
            })
        );
    }

    #[test]
    fn test_basic_ticker() {
        let tf = BasicTicker::new();
        let json_value: Value = tf.as_bokeh_value();
        assert_without_id_equal!(
            json_value,
            json!({
                "attributes": {},
                "type": "BasicTicker",
            })
        );
    }
    */
}
