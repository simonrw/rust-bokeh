# rust-bokeh

[![Build Status](https://travis-ci.org/mindriot101/rust-bokeh.svg?branch=dev)](https://travis-ci.org/mindriot101/rust-bokeh)
[![Join the chat at https://gitter.im/rust-bokeh/Lobby](https://badges.gitter.im/rust-bokeh/Lobby.svg)](https://gitter.im/rust-bokeh/Lobby?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

Bokeh-compatible Rust plotting library

## Architecture design

The architecture of this project tries to map to the original `bokeh`
structure as possible. To that end, the top level directory consists of
a rust *workspace*. The main implementation is split into a low level
interface `bokeh-models` and a high level interface `bokeh-plotting`.

## Setup instructions

The script `bin/setup` sets up a python virtual environment and installs
bokeh. This helps with working out what JSON schema we need.

## Building

`cargo build [--release]`

## Testing

`cargo test`

## Contributing

### Models

In order to implement a new model, the following steps must be
implemented:

- Implement `ToBokehJs` for the object, which returns a valid JSON
  structure for serialization,
- Add a test for the glyph to make sure it matches the expected output.

Models in the Bokeh API each have an ID. In order to be given a
predictable ID, the model should include a `with_id` custom constructor
which takes an ID type. Then in the tests, pass the known predictable id
to the `with_id` constructor.
