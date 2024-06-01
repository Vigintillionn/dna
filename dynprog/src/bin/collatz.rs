use dynprog::collatz;
use plotter::types::{Algorithm, Case};

fn main() {
  Algorithm::new("Collatz", collatz_recursive)
    .with_case(Case::new("Recursive")
      .iterations(1)
      .step_size(1)
      .range((1, 5000))
      .with_generator(|size| (0..size as i32).into_iter().collect())
    )
    .run()
    .plot("collatz-recursive")
    .unwrap();

  Algorithm::new("Collatz-Zoomed", collatz_recursive)
    .with_case(Case::new("Recursive")
      .iterations(1)
      .step_size(1)
      .range((1, 30))
      .with_generator(|size| (0..size as i32).into_iter().collect())
    )
    .run()
    .plot("collatz-recursive-zoomed")
    .unwrap();

  Algorithm::new("Collatz-Min", collatz_min)
    .with_case(Case::new("Recursive-Min")
      .iterations(1)
      .step_size(1)
      .range((1, 5000))
      .with_generator(|size| (0..size as i32).into_iter().collect())
    )
    .run()
    .plot("collatz-min")
    .unwrap();

  Algorithm::new("Collatz-Min-Zoomed", collatz_min_nums)
    .with_case(Case::new("Recursive-Min")
      .iterations(1)
      .step_size(1)
      .range((1, 30))
      .with_generator(|size| (0..size as i32).into_iter().collect())
    )
    .run()
    .plot("collatz-min-zoomed")
    .unwrap();

  Algorithm::new("Collatz-Nums", collatz_nums)
    .with_case(Case::new("Recursive")
      .iterations(1)
      .step_size(1)
      .range((1, 30))
      .with_generator(|size| (0..size as i32).into_iter().collect())
    )
    .run()
    .plot("collatz")
    .unwrap();

  Algorithm::new("Collatz", collatz_dynamic)
    .with_case(Case::new("Dynamic")
      .iterations(1)
      .step_size(1)
      .range((1, 10000))
      .with_generator(|size| (0..size as i32).into_iter().collect())
    )
    .run()
    .plot("collatz-dynamic")
    .unwrap();
}

fn collatz_min(arr: &mut [i32]) -> f64 {
  collatz::recursive_min(arr.len() as usize)
}

fn collatz_min_nums(arr: &mut [i32]) -> f64 {
  let n = arr.len() as usize;
  let mut counts = 0;
  collatz::rec_min_sol(n, &mut counts)
}

fn collatz_recursive(arr: &mut [i32]) -> f64 {
  collatz::recursive(arr.len() as usize)
}

fn collatz_dynamic(arr: &mut [i32]) -> f64 {
  collatz::dynamic(arr.len() as usize)
}

fn collatz_nums(arr: &mut [i32]) -> f64 {
  let n = arr.len() as usize;
  let mut counts = 0;
  collatz::rec_sol(n, &mut counts)
}