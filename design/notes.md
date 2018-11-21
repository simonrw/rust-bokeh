# Notes

- Bokeh is very much built on inheritance to provide values. We cannot
  do this in Rust so we have to think about other approaches.
- The other projects ([`rbokeh`](https://github.com/bokeh/rbokeh/) and
  [`bokeh-scala`](https://github.com/bokeh/bokeh-scala)) both manually
  implement the functions required and do not automatically generate
  code from the JSON specifications.
- The blob of JSON in the output template is the JSON-serialized
  representation of a
  [`Document`](https://bokeh.pydata.org/en/latest/docs/reference/document/document.html#bokeh-document-document)
  object. This is the top level object that holds all of the other
  objects.

## Objects and packages

The following list is the names of the files contained under Python's
`bokeh.models`:

- `annotations`
- `arrow_heads`
- `axes`
- `callbacks`
- `expressions`
- `filters`
- `formatters`
- `glyphs`
- `graphs`
- `grids`
- `layouts`
- `map_plots`
- `mappers`
- `markers`
- `plots`
- `ranges`
- `renderers`
- `scales`
- `selections`
- `sources`
- `tickers`
- `tiles`
- `tools`
- `transforms`
