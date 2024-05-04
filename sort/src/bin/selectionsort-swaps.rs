use plotter::types::{Case, Plot, RGBColor};
use sort::selectionsort::sort;
use plotter::test_sorting_algorithm;
use plotter::functions::generators::{generate_random, generate_reversed, generate_sorted};

fn main() {
  test_sorting_algorithm(
    "Selection Sort - Swaps",
    vec![
      (Box::new(|arr| sort(arr, false)), RGBColor(0, 0, 255), "Measured")
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
        Plot::new(|x| x as f64, RGBColor(255, 0, 0), "Expected")
      ]),

    Case::new("Worst")
      .with_generators(vec![generate_reversed])
      .iterations(1)
      .plots(vec![
        Plot::new(|x| x as f64, RGBColor(255, 0, 0), "Expected")
      ]),

    Case::new("Best")
      .with_generators(vec![generate_sorted])
      .iterations(1)
      .plots(vec![
        Plot::new(|x| x as f64, RGBColor(255, 0, 0), "Expected")
      ]),
  ]
}