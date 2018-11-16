# Notes

- Bokeh is very much built on inheritance to provide values. We cannot
  do this in Rust so we have to think about other approaches.
- The other projects ([`rbokeh`](https://github.com/bokeh/rbokeh/) and
  [`bokeh-scala`](https://github.com/bokeh/bokeh-scala)) both manually
  implement the functions required and do not automatically generate
  code from the JSON specifications.
