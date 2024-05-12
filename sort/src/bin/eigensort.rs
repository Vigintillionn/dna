use plotter::{types::{Case, Plot, Combinator, BLUE, Algorithm, RED, BLACK}, functions::generators::generate_reversed};
use sort::eigensort::sort;
use plotter::functions::generators::{generate_random, generate_sorted, quick_best};

fn main() {
  Combinator::new("Eigen Sort")
    .add_algorithms(vec![
      Algorithm::new("Eigen Sort 4", |arr| sort(arr, 4))
        .with_cases(vec![
          Case::new("Average threshold 4")
            .with_generator(generate_random)
            .iterations(100)
            .set_color(BLUE)
            .range((0, 48))
            .step_size(1)
            .plots(vec![
              Plot::new(|x| if x < 4 { 0.25 * x as f64 * x as f64 } else { 1.39 * x as f64 * (x as f64).log2() }, BLUE, "Expected threshold 4")
            ]),
          Case::new("Best threshold 4")
            .with_generator(|size| if size < 4 { generate_sorted(size) } else { quick_best(size) })
            .iterations(100)
            .set_color(BLUE)
            .range((0, 48))
            .step_size(1)
            .plots(vec![
              Plot::new(|x| if x < 4 { x as f64 } else { x as f64 * (x as f64).log2() }, BLUE, "Expected threshold 4")
            ]),
          Case::new("Worst threshold 4")
            .with_generator(generate_reversed)
            .iterations(100)
            .set_color(BLUE)
            .range((0, 48))
            .step_size(1)
            .plots(vec![
              Plot::new(|x| 0.5 * x as f64 * x as f64, RED, "Expected threshold 4")
            ]),
       ]),
      Algorithm::new("Eigen Sort 8", |arr| sort(arr, 8))
        .with_cases(vec![
          Case::new("Average threshold 8")
            .with_generator(generate_random)
            .iterations(100)
            .set_color(RED)
            .range((0, 48))
            .step_size(1)
            .plots(vec![
              Plot::new(|x| if x < 8 { 0.25 * x as f64 * x as f64 } else { 1.39 * x as f64 * (x as f64).log2() }, RED, "Expected threshold 8")
            ]),
          Case::new("Best threshold 8")
            .with_generator(|size| if size < 8 { generate_sorted(size) } else { quick_best(size) })
            .iterations(100)
            .set_color(RED)
            .range((0, 48))
            .step_size(1)
            .plots(vec![
              Plot::new(|x| if x < 8 { x as f64 } else { x as f64 * (x as f64).log2() }, RED, "Expected threshold 8")
            ]),
          Case::new("Worst threshold 8")
            .with_generator(generate_reversed)
            .iterations(100)
            .set_color(RED)
            .range((0, 48))
            .step_size(1)
            .plots(vec![
              Plot::new(|x| 0.5 * x as f64 * x as f64, RED, "Expected threshold 8")
            ]),
        ]),
        Algorithm::new("Eigen Sort 16", |arr| sort(arr, 8))
        .with_cases(vec![
          Case::new("Average threshold 16")
            .with_generator(generate_random)
            .iterations(100)
            .set_color(BLACK)
            .range((0, 48))
            .step_size(1)
            .plots(vec![
              Plot::new(|x| if x < 16 { 0.25 * x as f64 * x as f64 } else { 1.39 * x as f64 * (x as f64).log2() }, BLACK, "Expected threshold 16")
            ]),
          Case::new("Best threshold 16")
            .with_generator(|size| if size < 16 { generate_sorted(size) } else { quick_best(size) })
            .iterations(100)
            .set_color(BLACK)
            .range((0, 48))
            .step_size(1)
            .plots(vec![
              Plot::new(|x| if x < 16 { x as f64 } else { x as f64 * (x as f64).log2() }, BLACK, "Expected threshold 16")
            ]),
          Case::new("Worst threshold 16")
            .with_generator(generate_reversed)
            .iterations(100)
            .set_color(BLACK)
            .range((0, 48))
            .step_size(1)
            .plots(vec![
              Plot::new(|x| 0.5 * x as f64 * x as f64, BLACK, "Expected threshold 16")
            ]),
        ]),
    ])
    .run()
    .plot_seperate(vec!["eigensort-avg", "eigensort-best", "eigensort-worst"])
    .unwrap();
}