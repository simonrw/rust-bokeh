#!/usr/bin/env python3

from bokeh.plotting import figure, show, output_file
from bokeh.layouts import column

output_file("examples/python/multiple_output.html")

p1 = figure()
p1.circle([1, 2, 3], [4, 5, 6])

show(p1)

p2 = figure()
p2.line([1, 2, 3], [4, 5, 6])

show(p2)
