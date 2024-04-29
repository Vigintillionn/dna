use plotter::types::{Case, Plot, RGBColor};
use sort::selectionsort::sort;
use plotter::test_sorting_algorithm;
use plotter::functions::generators::{generate_random, generate_reversed, generate_sorted};

fn main() {
  test_sorting_algorithm(
    "Selection Sort",
    vec![
      (Box::new(sort), RGBColor(0, 0, 255), "Measured")
    ], 
    get_cases())
}

fn get_cases() -> Vec<Case> {
  vec![
    Case {
      name: "Average".to_string(),
      generator: Box::new(generate_random),
      iterations: 1,
      plots: vec![Plot {
        color: RGBColor(255, 0, 0),
        func: Box::new(|x| 0.5 * x as f64 * x as f64),
        name: "Expected".to_string(),
      }],
    },
    Case {
      name: "Worst".to_string(),
      generator: Box::new(generate_reversed),
      iterations: 1,
      plots: vec![Plot {
        color: RGBColor(255, 0, 0),
        func: Box::new(|x| 0.5 * x as f64 * x as f64),
        name: "Expected".to_string(),
      }],
    },
    Case {
      name: "Best".to_string(),
      generator: Box::new(generate_sorted),
      iterations: 1,
      plots: vec![Plot {
        color: RGBColor(255, 0, 0),
        func: Box::new(|x| 0.5 * x as f64 * x as f64),
        name: "Expected".to_string(),
      }],
    },
  ]
}