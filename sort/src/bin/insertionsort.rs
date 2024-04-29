use plotter::types::{Case, Plot, RGBColor};
use sort::insertionsort::sort;
use plotter::test_sorting_algorithm;
use plotter::functions::generators::{generate_random, generate_sorted, generate_reversed, generate_with_dupes, generate_nearly_sorted};

fn main() {
  test_sorting_algorithm(
    "Insertion Sort",
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
      iterations: 100,
      plots: vec![Plot {
        color: RGBColor(255, 0, 0),
        func: Box::new(|x| 0.25 * x as f64 * x as f64),
        name: "Expected".to_string(),
      }],
    },
    Case {
      name: "Best".to_string(),
      generator: Box::new(generate_sorted),
      iterations: 1,
      plots: vec![Plot {
        color: RGBColor(255, 0, 0),
        func: Box::new(|x| x as f64),
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
      name: "Dupes".to_string(),
      generator: Box::new(generate_with_dupes),
      iterations: 25,
      plots: vec![Plot {
        color: RGBColor(255, 0, 0),
        func: Box::new(|x| 0.25 * x as f64 * x as f64),
        name: "Expected".to_string(),
      }],
    },
    Case {
      name: "Nearly-sorted".to_string(),
      generator: Box::new(|i| generate_nearly_sorted(i, 100)),
      iterations: 25,
      plots: vec![Plot {
        color: RGBColor(255, 0, 0),
        func: Box::new(|x| 25.74 * x as f64),
        name: "Expected".to_string(),
      }],
    },
  ]
}