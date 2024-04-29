use plotter::functions::less;

pub fn sort<T: Ord + Copy>(arr: &mut [T]) -> f64 {
  let mut comparisons: f64 = 0.0;
  let mut j: i32;
  let mut key: T;
  // loop through the array
  for i in 1..arr.len() {
    key = arr[i];
    j = i as i32 - 1;
    comparisons += 1.0;
    // move elements of arr[0..i-1] that are greater than key to one position ahead of their current position
    while j >= 0 && less(key, arr[j as usize], &mut comparisons) {
      arr[(j + 1) as usize] = arr[j as usize];
      j -= 1;
    }
    // insert the key element at its correct position in the sorted array
    arr[(j + 1) as usize] = key;
  }
  comparisons
}