use plotter::types::{Case, Plot, RGBColor, Combinator, Algorithm};
use sort::{insertionsort, mergesort, quicksort};
use plotter::test_sorting_algorithm;
use plotter::functions::generators::{generate_random, generate_sorted, generate_reversed, generate_interleaved, quick_best};

fn main() {
  Combinator::new("IS - MS - QS")
    .add_algorithms(
      vec![
        Algorithm::new("Insertion Sort", |arr| insertionsort::sort(arr, true))
          .set_color(RGBColor(0, 0, 255))
          .with_cases(vec![
            Case::new("Insertion Sort")
            .with_generator(generate_random)
            .iterations(20)
            .plots(vec![
              Plot::new(|x| 0.25 * x as f64 * x as f64, RGBColor(0, 0, 255), "Expected IS")
            ]),
            Case::new("Insertion Sort")
            .with_generator(generate_sorted)
            .iterations(10)
            .plots(vec![
              Plot::new(|x| x as f64, RGBColor(0, 0, 255), "Expected IS")
            ])
        ]),
        Algorithm::new("Merge Sort", mergesort::sort)
          .set_color(RGBColor(255, 0, 0))
          .with_case(
            Case::new("Merge Sort")
            .with_generator(generate_random)
            .iterations(1)
            .plots(vec![
              Plot::new(|x| 0.85 * x as f64 * (x as f64).log2(), RGBColor(255, 0, 0), "Expected MS")
            ])
          ),
        Algorithm::new("Quick Sort", |arr| quicksort::sort(arr, true))
          .set_color(RGBColor(0, 255, 0))          
          .with_case(
            Case::new("Quick Sort")
            .with_generator(generate_random)
            .iterations(1)
            .plots(vec![
              Plot::new(|x| 1.39 * x as f64 * (x as f64).log2(), RGBColor(0, 255, 0), "Expected QS")
            ])
          )
      ]
    )
    .run()
    .plot("is-ms-qs")
    .unwrap();
}