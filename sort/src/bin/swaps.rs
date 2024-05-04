use plotter::types::{Case, Plot, RGBColor};
use sort::{selectionsort, quicksort};
use plotter::test_sorting_algorithm;
use plotter::functions::generators::{generate_random, generate_sorted, generate_reversed, generate_interleaved, quick_best};

fn main() {
  test_sorting_algorithm(
    "SS - QS - Swaps",
    vec![
      (Box::new(|arr| selectionsort::sort(arr, false)), RGBColor(255, 0, 0), "Insertion Sort"),
      (Box::new(|arr| quicksort::sort(arr, false)), RGBColor(0, 0, 255), "Quick Sort")
    ], 
    get_cases()
  )
}

fn get_cases() -> Vec<Case> {
  vec![
    Case::new("Average")
      .with_generators(vec![generate_random])
      .iterations(50),

    Case::new("Best")
      .with_generators(vec![generate_sorted, quick_best])
      .iterations(50),

    Case::new("Worst")
      .with_generators(vec![generate_reversed, generate_sorted])
      .iterations(50),
  ]
}