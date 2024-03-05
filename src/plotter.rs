use plotters::prelude::*;

pub fn plot_comparisons(data: Vec<(usize, usize)>, name: String) {
  let name_with_extension = format!("plots/{}.png", name);
  let root = BitMapBackend::new(&name_with_extension, (800, 600)).into_drawing_area();
  root.fill(&WHITE).unwrap();

  let (x, y): (Vec<usize>, Vec<usize>) = data.into_iter().unzip();

  let max_x = *x.iter().max().unwrap();
  let max_y = *y.iter().max().unwrap();

  let mut chart = ChartBuilder::on(&root)
    .caption("Comparisons vs Array Length", ("sans-serif", 30))
    .margin(5)
    .x_label_area_size(40)
    .y_label_area_size(40)
    .build_cartesian_2d(0..max_x, 0..max_y)
    .unwrap();

  chart
    .configure_mesh()
    .x_labels(10)
    .y_labels(10)
    .draw()
    .unwrap();

  chart
    .draw_series(PointSeries::of_element(x.into_iter().zip(y.into_iter()), 3, &BLUE, &|c, s, st| {
        return EmptyElement::at(c) + Circle::new((0,0), s, st.filled());
    }))
    .unwrap();
}