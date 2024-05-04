use plotter::types::{Case, Plot, RGBColor};
use sort::quicksort::sort;
use plotter::test_sorting_algorithm;
use plotter::functions::generators::{generate_random, generate_sorted, quick_best};

fn main() {
  test_sorting_algorithm(
    "Quick Sort",
    vec![
      (Box::new(|arr| sort(arr, true)), RGBColor(0, 0, 255), "Measured")
    ], 
    get_cases()
  )
}

fn get_cases() -> Vec<Case> {
  vec![
    Case::new("Average")
      .with_generators(vec![generate_random])
      .iterations(100)
      .plots(vec![
        Plot::new(|x| 1.39 * x as f64 * (x as f64).log2(), RGBColor(255, 0, 0), "Expected")
      ]),

    Case::new("Worst")
      .with_generators(vec![generate_sorted])
      .iterations(1)
      .plots(vec![
        Plot::new(|x| 0.5 * x as f64 * x as f64, RGBColor(255, 0, 0), "Expected")
      ]),

    Case::new("Best")
      .with_generators(vec![quick_best])
      .iterations(1)
      .plots(vec![
        Plot::new(|x| x as f64 * (x as f64).log2(), RGBColor(255, 0, 0), "Expected")
      ]),
  ]
}