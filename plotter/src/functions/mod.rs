pub mod generators;

pub fn swap<T: Ord + Copy>(arr: &mut [T], i: usize, j: usize, count: &mut f64) {
  let temp = arr[i];
  arr[i] = arr[j];
  arr[j] = temp;
  *count += 1.0;
}

pub fn less<T: Ord + Copy>(a: T, b: T, count: &mut f64) -> bool {
  *count += 1.0;
  a < b
}