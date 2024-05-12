use plotter::types::{Case, Plot, RGBColor, Algorithm};
use sort::mergesort::sort;
use plotter::functions::generators::{generate_random, generate_sorted, generate_interleaved};

fn main() {
  Algorithm::new("Merge Sort", |arr| sort(arr, true))
    .with_cases(vec![
      Case::new("Average")
        .with_generator(generate_random)
        .iterations(100)
        .plots(vec![
          Plot::new(|x| 0.85 * x as f64 * (x as f64).log2(), RGBColor(255, 0, 0), "Expected")
        ]),
      Case::new("Best")
        .with_generator(generate_sorted)
        .iterations(1)
        .plots(vec![
          Plot::new(|x| 0.5 * x as f64 * (x as f64).log2(), RGBColor(255, 0, 0), "Expected")
        ]),
      Case::new("Worst")
        .with_generator(generate_interleaved)
        .iterations(1)
        .plots(vec![
          Plot::new(|x| x as f64 * (x as f64).log2(), RGBColor(255, 0, 0), "Expected")
        ]),
    ])
    .run()
    .plot_seperate(vec!["mergesort-avg", "mergesort-best", "mergesort-worst"])
    .unwrap();

}