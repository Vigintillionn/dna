use crate::types::Sort;

pub struct InsertionSort;

impl InsertionSort {
  pub fn new() -> InsertionSort {
    InsertionSort
  }
}

impl Sort<i32> for InsertionSort {
  fn sort(&self, arr: &mut [i32]) -> usize {
    let mut comparisons: usize = 0;
    let mut j: i32;
    let mut key: i32;
    for i in 1..arr.len() {
      key = arr[i];
      j = i as i32 - 1;
      comparisons += 1;
      while j >= 0 && arr[j as usize] > key {
        comparisons += 1;
        arr[(j + 1) as usize] = arr[j as usize];
        j -= 1;
      }
      arr[(j + 1) as usize] = key;
    }
    comparisons
  }
}