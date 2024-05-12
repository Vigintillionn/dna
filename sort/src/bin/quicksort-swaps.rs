use plotter::types::{Case, Plot, RGBColor, Algorithm};
use sort::quicksort::sort;
use plotter::functions::generators::{generate_random, generate_sorted, quick_best};

fn main() {
  // test_sorting_algorithm(
  //   "Quick Sort - Swaps",
  //   vec![
  //     (Box::new(|arr| sort(arr, false)), RGBColor(0, 0, 255), "Measured")
  //   ], 
  //   get_cases()
  // )

  Algorithm::new("Quick Sort Swaps", |arr| sort(arr, false))
    .with_cases(vec![
      Case::new("Average")
        .with_generator(generate_random)
        .iterations(50)
        .plots(vec![
          Plot::new(|x| 0.23 * x as f64 * (x as f64).log2(), RGBColor(255, 0, 0), "Expected")
        ]),

      Case::new("Worst")
        .with_generator(generate_sorted)
        .iterations(1)
        .plots(vec![
          Plot::new(|x| x as f64, RGBColor(255, 0, 0), "Expected")
        ]),

      Case::new("Best")
        .with_generator(quick_best)
        .iterations(1)
        .plots(vec![
          Plot::new(|x| 0.5 * x as f64, RGBColor(255, 0, 0), "Expected")
        ]),
    ])
    .run()
    .plot_seperate(vec!["quicksort-swaps-avg", "quicksort-swaps-worst", "quicksort-swaps-best"])
    .unwrap();

}