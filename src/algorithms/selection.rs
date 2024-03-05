use crate::types::Sort;
pub struct SelectionSort;

impl SelectionSort {
  pub fn new() -> SelectionSort {
    SelectionSort
  }
}

impl Sort<i32> for SelectionSort {
  fn sort(&self, arr: &mut [i32]) -> usize {
    let mut min_index: usize;
    let mut temp: i32;
    let mut comparisons: usize = 0;

    for i in 0..arr.len() {
      min_index = i;
      for j in (i + 1)..arr.len() {
        comparisons += 1;
        if arr[j] < arr[min_index] {
          min_index = j;
        }
      }
      temp = arr[i];
      arr[i] = arr[min_index];
      arr[min_index] = temp;
    }
    comparisons
  }
}