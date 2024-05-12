use plotter::types::{Case, Plot, RGBColor, Algorithm};
use sort::selectionsort::sort;
use plotter::functions::generators::{generate_random, generate_reversed, generate_sorted};

fn main() {
  Algorithm::new("Selection Sort", |arr| sort(arr, false))
    .with_cases(vec![
      Case::new("Average")
        .with_generator(generate_random)
        .iterations(100)
        .plots(vec![
          Plot::new(|x| 0.5 * x as f64 * x as f64, RGBColor(255, 0, 0), "Expected")
        ]),
      Case::new("Worst")
        .with_generator(generate_reversed)
        .iterations(1)
        .plots(vec![
          Plot::new(|x| 0.5 * x as f64 * x as f64, RGBColor(255, 0, 0), "Expected")
        ]),
      Case::new("Best")
        .with_generator(generate_sorted)
        .iterations(1)
        .plots(vec![
          Plot::new(|x| 0.5 * x as f64 * x as f64, RGBColor(255, 0, 0), "Expected")
        ]),
    ])
    .run()
    .plot_seperate(vec!["selectionsort-avg", "selectionsort-worst", "selectionsort-best"])
    .unwrap();
}