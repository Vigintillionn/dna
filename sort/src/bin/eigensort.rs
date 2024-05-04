use plotter::types::{Case, RGBColor, Plot, Combinator, BLUE, Algorithm};
use sort::eigensort::sort;
use plotter::functions::generators::{generate_random, generate_sorted, quick_best};

fn main() {
  Combinator::new("Eigen Sort")
    .add_algorithms(vec![
      Algorithm::new("Eigen Sort 512", |arr| sort(arr, 512))
        .with_cases(vec![
          Case::new("Average")
            .with_generator(generate_random)
            .iterations(10)
            .set_color(BLUE),
          Case::new("Best")
            .with_generator(generate_sorted)
            .iterations(10)
            .set_color(BLUE)
       ]),
      Algorithm::new("Eigen Sort 1024", |arr| sort(arr, 1024))
        .with_cases(vec![
          Case::new("Average")
            .with_generator(generate_random)
            .iterations(10)
            .set_color(BLUE),
          Case::new("Best")
            .with_generator(generate_sorted)
            .iterations(10)
            .set_color(BLUE)
        ]),
      
    ])
    .run()
    .plot_seperate(vec!["eigensort-avg", "eigensort-best"])
    .unwrap();
}