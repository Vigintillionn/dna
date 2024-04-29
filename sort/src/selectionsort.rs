use plotter::functions::{less, swap};

pub fn sort<T: Ord + Copy>(arr: &mut [T]) -> f64 {
  let mut min_index: usize;
  let mut swaps: f64 = 0.0;
  let mut compares: f64 = 0.0;

  // One by one move boundary of unsorted subarray
  for i in 0..arr.len() {
    min_index = i;
    // Find the minimum element in unsorted array
    for j in (i + 1)..arr.len() {
      // If arr[j] is smaller, then it is the new minimum
      if less(arr[j], arr[min_index], &mut compares) {
        min_index = j;
      }
    }
    // Swap the found minimum element with the first element
    swap(arr, i, min_index, &mut swaps);
  }

  compares
}

