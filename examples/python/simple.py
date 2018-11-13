#!/usr/bin/env python3

from bokeh.plotting import figure, show, output_file

output_file("/tmp/out.html")

p = figure()
p.circle([1, 2, 3], [4, 5, 6])
show(p)
