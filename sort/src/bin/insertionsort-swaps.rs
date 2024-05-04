use plotter::types::{Case, Plot, RGBColor, Algorithm, RED};
use sort::selectionsort::sort;
use plotter::functions::generators::{generate_random, generate_reversed, generate_sorted};

fn main() {
  Algorithm::new("Insertion Sort Swaps", |arr| sort(arr, false))
    .with_cases(vec![
      Case::new("Average")
        .with_generator(generate_random)
        .iterations(100)
        .plots(vec![
          Plot::new(|x| x as f64, RED, "Expected")
        ]),
      Case::new("Worst")
        .with_generator(generate_reversed)
        .iterations(1)
        .plots(vec![
          Plot::new(|x| x as f64, RED, "Expected")
        ]),
      Case::new("Best")
        .with_generator(generate_sorted)
        .iterations(1)
        .plots(vec![
          Plot::new(|x| x as f64, RED, "Expected")
        ]),
    ])
    .run()
    .plot_seperate(vec!["insertionsort-swaps-average", "insertionsort-swaps-worst", "insertionsort-swaps-best"])
    .unwrap();
}