use plotters::prelude::*;
use crate::types::{Plot, AlgorithmResult};

#[allow(dead_code)]
pub fn plot_case(
  results: Vec<AlgorithmResult>, //Vec<(Vec<(usize, Vec<f64>)>, RGBColor, &str)>,
  title: &str,
  filename: &str,
  iterations: usize,
  plots: &Vec<&Plot>
) -> Result<(), Box<dyn std::error::Error>> {
  // Name the file with the filename and the extension
  let name_with_extension = format!("out/{}.png", filename);
  // Create a new bitmap backend with the name of the file
  let root = BitMapBackend::new(&name_with_extension, (1200, 1000)).into_drawing_area();
  root.fill(&WHITE)?;

  let root = root.titled(title, ("sans-serif", 60))?;

  // let data: Vec<(Vec<(usize, Vec<f64>)>, RGBColor, &str)> = results
  //   .into_iter()
  //   .map(|r| (r.data, r.color, &r.name[..]))
  //   .collect();

  let data: Vec<Vec<(usize, Vec<f64>)>> = results
    .clone()
    .into_iter()
    .map(|r| r.data)
    .collect();

  // Enumerate the data to get the x and y values
  let (x, y): (Vec<usize>, Vec<Vec<f64>>) = data
    .clone()
    .into_iter()
    .flatten()
    .unzip();

  // Find the maximum x and y values
  let max_x = *x.iter().max().unwrap();
  let max_y = y.iter()
    .flat_map(|v| v.iter())
    .copied()
    .reduce(|a, b| a.max(b))
    .unwrap_or(0.0);

  let mut max_candidates = vec![max_y];
  for plot in plots {
    let max = (0..=max_x).map(|x| (plot.func)(x)).fold(f64::NEG_INFINITY, |max, x| x.max(max));
    max_candidates.push(max);
  }
  let max_y = max_candidates.iter().copied().fold(f64::NEG_INFINITY, |max, x| x.max(max));

  // let caption = &format!("Min, max and average of {} measurements", iterations)[..];

  // Create a chart with the title and the maximum x and y values
  let mut chart = ChartBuilder::on(&root)
    // .caption(caption, ("sans-serif", 40).into_font())
    .margin(25)
    .x_label_area_size(40)
    .y_label_area_size(40)
    .build_cartesian_2d(0..max_x, 2.0..max_y)?;

  // Book keeping
  chart
    .configure_mesh()
    .x_labels(20)
    .y_labels(20)
    .x_desc("Array Length")
    .y_desc("Measurements")
    .axis_desc_style(("sans-serif", 20))
    .draw()?;

  // Draw the plots
  for plot in plots {
    chart
      .draw_series(LineSeries::new(
        (0..=max_x).map(|x| (x, if x > 0 { (plot.func)(x) } else { 0.0 })),
        plot.color.stroke_width(1),
      ))?.label(&plot.name).legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], plot.color));
  }

  // Draw the main chart
  for r in results {
    let color: RGBColor = r.color;
    chart
    .draw_series(
      r.data.into_iter().map(|(x, y)| {
        let yl = y.iter().copied().reduce(|a, b| a.min(b)).unwrap_or(0.0);
        let ym: f64 = y.iter().sum::<f64>() / y.len() as f64;//&y[y.len() / 2];
        let yh = y.iter().copied().reduce(|a, b| a.max(b)).unwrap_or(0.0);

        return ErrorBar::new_vertical(x, yl, ym, yh, color.filled(), 4)
      }),
    )?.label(r.name).legend(move |(x, y)| ErrorBar::new_vertical(x + 10, y - 5, y, y + 5, color.filled(), 4));
  }

  chart.configure_series_labels().position(SeriesLabelPosition::UpperLeft).background_style(WHITE).label_font(("sans-serif", 20)).border_style(BLACK).draw()?;

  Ok(())
}