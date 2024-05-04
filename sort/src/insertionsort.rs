use plotter::functions::{less, swap};

pub fn sort<T: Ord + Copy>(arr: &mut [T], ret_comparisons: bool) -> f64 {
  let mut comparisons: f64 = 0.0;
  let mut swaps: f64 = 0.0;

  for i in 1..arr.len() {
    let mut j = i;
    while j > 0 && less(arr[j], arr[j - 1], &mut comparisons) {
      swap(arr, j, j-1, &mut swaps);
      j -= 1;
      swaps += 1.0;
    }
  }

  if ret_comparisons {
    comparisons
  } else {
    swaps
  }
}