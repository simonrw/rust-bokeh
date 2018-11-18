# README

In order to implement a new model, the following steps must be
implemented:

- Implement `ToBokehJs` for the object, which returns a valid JSON
  structure for serialization,
- Add a test for the glyph to make sure it matches the expected output.

Models in the Bokeh API each have an ID. In order to be given a
predictable ID, the model should include a `with_id` custom constructor
which takes an ID type. Then in the tests, pass the known predictable id
to the `with_id` constructor.
