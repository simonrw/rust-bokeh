extern crate rust_bokeh;

fn main() {
    let mut p = rust_bokeh::FigureBuilder::new()
        .build()
        .expect("Building figure");

    let xdata = vec![1, 2, 3];
    let ydata = vec![4, 5, 6];

    p.circle(xdata, ydata);

    rust_bokeh::to_file(p, "/tmp/out.html").expect("Rendering HTML");
}
