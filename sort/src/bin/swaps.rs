use plotter::types::{Case, Plot, Combinator, Algorithm, RED, BLUE, BLACK};
use sort::{selectionsort, quicksort};
use plotter::functions::generators::{generate_random, generate_sorted, generate_reversed, quick_best};

fn main() {
  Combinator::new("SS - IS - QS - Swaps")
    .add_algorithms(vec![
      Algorithm::new("Selection Sort", |arr| selectionsort::sort(arr, false))
        .with_cases(vec![
          Case::new("Average SS")
            .with_generator(generate_random)
            .iterations(1)
            .plots(vec![
              Plot::new(|x| x as f64, RED, "n")
            ])
            .set_color(RED),
          Case::new("Worst SS")
            .with_generator(generate_reversed)
            .iterations(1)
            .plots(vec![
              Plot::new(|x| x as f64, RED, "n")
            ])
            .set_color(RED),
          Case::new("Best SS")
            .with_generator(generate_sorted)
            .iterations(1)
            .plots(vec![
              Plot::new(|x| x as f64, RED, "n")
            ])
            .set_color(RED),
        ]),
      Algorithm::new("Insertion Sort", |arr| selectionsort::sort(arr, false))
        .with_cases(vec![
          Case::new("Average IS")
            .with_generator(generate_random)
            .iterations(50)
            .plots(vec![
              Plot::new(|x| x as f64, BLUE, "n")
            ])
            .set_color(BLUE),
          Case::new("Worst IS")
            .with_generator(generate_reversed)
            .iterations(1)
            .plots(vec![
              Plot::new(|x| x as f64, BLUE, "n")
            ])
            .set_color(BLUE),
          Case::new("Best IS")
            .with_generator(generate_sorted)
            .iterations(1)
            .plots(vec![
              Plot::new(|x| x as f64, BLUE, "n")
            ])
            .set_color(BLUE),
        ]),
      Algorithm::new("Quick Sort", |arr| quicksort::sort(arr, false))
        .with_cases(vec![
          Case::new("Average QS")
            .with_generator(generate_random)
            .iterations(50)
            .plots(vec![
              Plot::new(|x| 2.5 * x as f64, BLACK, "5/2n")
            ])
            .set_color(BLACK),
          Case::new("Worst QS")
            .with_generator(generate_sorted)
            .iterations(1)
            .plots(vec![
              Plot::new(|x| x as f64, BLACK, "n")
            ])
            .set_color(BLACK),
          Case::new("Best QS")
            .with_generator(quick_best)
            .iterations(1)
            .plots(vec![
              Plot::new(|x| 0.5 * x as f64, BLACK, "1/2n")
            ])
            .set_color(BLACK),
        ]),
    ])
    .run()
    .plot_seperate(vec!["swaps-avg", "swaps-worst", "swaps-best"])
    .unwrap();
}