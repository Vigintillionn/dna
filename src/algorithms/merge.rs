use crate::types::{Algorithm, AlgorithmData, Expected, ExpectedData};
use crate::generate_array::{generate_random, generate_sorted, generate_reversed};

pub struct MergeSort {
  pub name: String
}

impl Algorithm for MergeSort {
  fn new() -> MergeSort {
    Self { name: "merge".to_string() }
  }

  fn sort(&self, arr: &mut [i32]) -> usize {
    let mut aux = arr.to_vec();
    if arr.len() == 0 {
      return 0;
    }
    let comparisons = merge_sort(arr, &mut aux, 0, arr.len() - 1);
    comparisons
  }

  fn get_cases(&self) -> Vec<AlgorithmData> {
    vec![
      AlgorithmData {
        name: "average".to_string(),
        generator: generate_random,
        expected: ExpectedData {
          function: Expected::NLogN,
          factor: 0.5,
        }
      }, 
      AlgorithmData {
        name: "best".to_string(),
        generator: generate_sorted,
        expected: ExpectedData {
          function: Expected::Quadratic,
          factor: 0.5,
        }
      }, 
      AlgorithmData {
        name: "worst".to_string(),
        generator: generate_reversed,
        expected: ExpectedData {
          function: Expected::Quadratic,
          factor: 1.0,
        }
      }
    ]
  }

  fn get_name(&self) -> String {
    self.name.clone()
  }
}

fn merge_sort(arr: &mut [i32], aux: &mut Vec<i32>, low: usize, high: usize) -> usize {
  let mut comparisons: usize = 0;
  if low < high {
    let mid = (low + high) / 2;
    comparisons += merge_sort(arr, aux, low, mid);
    comparisons += merge_sort(arr, aux, mid + 1, high);
    comparisons += merge(arr, aux, low, mid, high);
  }
  comparisons
}

fn merge(arr: &mut [i32], aux: &mut Vec<i32>, low: usize, mid: usize, high: usize) -> usize  {
  for i in low..=high {
    aux[i] = arr[i];
  }

  let mut i = low;
  let mut j = mid + 1;

  let mut comparisons: usize = 0;
  for k in low..=high {
    if i > mid {
      arr[k] = aux[j];
      j += 1;
    } else if j > high {
      arr[k] = aux[i];
      i += 1;
    } else if aux[j] < aux[i] {
      arr[k] = aux[j];
      j += 1;
      comparisons += 1;
    } else {
      arr[k] = aux[i];
      i += 1;
      comparisons += 1;
    }
  }

  comparisons
}