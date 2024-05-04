use plotter::{types::{Combinator, Algorithm}, functions::generators::generate_random};

fn main() {
  // Combinator::new("Test")
  //   .add_algorithm(
  //     Algorithm::new("Test", test_func)
  //   )
  //   .run();

  Algorithm::new("Test", test_func, generate_random);
}

fn test_func(arr: &mut[i32]) -> f64 {
  let mut test = 0.0;
  for i in 0..arr.len() {
    test += arr[i] as f64;
  }
  test
}