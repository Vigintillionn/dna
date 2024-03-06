use plotters::prelude::*;

use crate::types::Expected;

pub fn plot_comparisons(
  data: Vec<(usize, usize)>, 
  name: String, 
  case: String, 
  filename: String,
  expected: Expected,
  factor: f64
) {
  let name_with_extension = format!("plots/{}.png", filename);
  let root = BitMapBackend::new(&name_with_extension, (800, 600)).into_drawing_area();
  root.fill(&WHITE).unwrap();

  let (x, y): (Vec<usize>, Vec<usize>) = data.into_iter().unzip();

  let max_x = *x.iter().max().unwrap();
  let max_y = *y.iter().max().unwrap();

  let title = format!("{} - {}", name, case);

  let mut chart = ChartBuilder::on(&root)
    .caption(title, ("sans-serif", 30))
    .margin(45)
    .x_label_area_size(40)
    .y_label_area_size(40)
    .build_cartesian_2d(0..max_x, 0..max_y)
    .unwrap();

  chart
    .configure_mesh()
    .x_labels(10)
    .y_labels(10)
    .x_desc("Array Length")
    .axis_desc_style(("sans-serif", 15))
    .draw()
    .unwrap();

  chart
    .draw_series(PointSeries::of_element(x.into_iter().zip(y.into_iter()), 3, &BLUE, &|c, s, st| {
        return EmptyElement::at(c) + Circle::new((0,0), s, st.filled());
    }))
    .unwrap();

  chart
    .draw_series(LineSeries::new(
      (0..=max_x).map(|x| (x, apply_expected(expected.clone(), factor, x as f64))),
      &RED
    ))
    .unwrap();
}

fn apply_expected(expected: Expected, factor: f64, x: f64) -> usize {
  let res = match expected {
    Expected::NLogN => x * x.log2(),
    Expected::Quadratic => x * x,
    Expected::Linear => x,
  };

  (res * factor) as usize
}