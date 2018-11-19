# rust-bokeh

[![CircleCI](https://circleci.com/gh/mindriot101/rust-bokeh/tree/dev.svg?style=svg)](https://circleci.com/gh/mindriot101/rust-bokeh/tree/dev)
[![Join the chat at https://gitter.im/rust-bokeh/Lobby](https://badges.gitter.im/rust-bokeh/Lobby.svg)](https://gitter.im/rust-bokeh/Lobby?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

Bokeh-compatible Rust plotting library

- [Models documentation](bokeh-models/README.md)

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
