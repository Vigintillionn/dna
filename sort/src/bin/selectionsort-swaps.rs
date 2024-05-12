use plotter::types::{Case, Plot, RGBColor, Algorithm};
use sort::selectionsort::sort;
use plotter::functions::generators::{generate_random, generate_reversed, generate_sorted};

fn main() {
  // test_sorting_algorithm(
  //   "Selection Sort - Swaps",
  //   vec![
  //     (Box::new(|arr| sort(arr, false)), RGBColor(0, 0, 255), "Measured")
  //   ], 
  //   get_cases()
  // )

  Algorithm::new("Selection Sort Swaps", |arr| sort(arr, false))
    .with_cases(vec![
      Case::new("Average")
        .with_generator(generate_random)
        .iterations(1)
        .plots(vec![
          Plot::new(|x| x as f64, RGBColor(255, 0, 0), "Expected")
        ]),
      Case::new("Worst")
        .with_generator(generate_reversed)
        .iterations(1)
        .plots(vec![
          Plot::new(|x| x as f64, RGBColor(255, 0, 0), "Expected")
        ]),
      Case::new("Best")
        .with_generator(generate_sorted)
        .iterations(1)
        .plots(vec![
          Plot::new(|x| x as f64, RGBColor(255, 0, 0), "Expected")
        ]),
    ])
    .run()
    .plot_seperate(vec!["selectionsort-swaps-avg", "selectionsort-swaps-worst", "selectionsort-swaps-best"])
    .unwrap();
}