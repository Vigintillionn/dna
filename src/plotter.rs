use plotters::prelude::*;

use crate::types::{Expected, ExpectedData};

pub fn plot_comparisons(
  data: Vec<(usize, Vec<usize>)>, 
  name: String, 
  case: String, 
  filename: String,
  expected: ExpectedData,
) {
  let name_with_extension = format!("plots/{}.png", filename);
  let root = BitMapBackend::new(&name_with_extension, (1200, 1000)).into_drawing_area();
  root.fill(&WHITE).unwrap();

  let (x, y): (Vec<usize>, Vec<Vec<usize>>) = data.clone().into_iter().unzip();

  let max_x = *x.iter().max().unwrap();
  // let max_y = *y.iter().max().unwrap();
  let max_y = *y.iter().map(|v| v.iter().max().unwrap()).max().unwrap();

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
    .y_desc("Comparisons")
    .axis_desc_style(("sans-serif", 15))
    .draw()
    .unwrap();

  // chart
  //   .draw_series(PointSeries::of_element(x.into_iter().zip(y.into_iter()), 3, &BLUE, &|c, s, st| {
  //       let avg_y = (c.1.iter().sum::<usize>() as f64 / c.1.len() as f64) as usize;
  //       return EmptyElement::at((c.0, avg_y)) + Circle::new((0,0), s, st.filled());
  //   }))
  //   .unwrap();

  // chart.draw_series(LineSeries::new(
  //     data.iter().map(|(x, y)| (*x, y.iter().sum::<usize>() / y.len())),
  //     &BLUE,
  // ));


  chart
    .draw_series(
      data.into_iter().map(|(x, y)| {
        let yl = y.iter().min().unwrap();
        let ym = &y[y.len() / 2];
        let yh = y.iter().max().unwrap();

        return ErrorBar::new_vertical(x, *yl, *ym, *yh, BLUE.filled(), 4)
      }),
    ).unwrap().label("Measured").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

  chart
    .draw_series(LineSeries::new(
      (0..=max_x).map(|x| (x, apply_expected(expected.clone().function, expected.clone().factor, x as f64))),
      &RED
    ))
    .unwrap().label("Expected").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));
}

fn apply_expected(expected: Expected, factor: f64, x: f64) -> usize {
  let res = match expected {
    Expected::NLogN => x * x.log2(),
    Expected::Quadratic => x * x,
    Expected::Linear => x,
  };

  (res * factor) as usize
}