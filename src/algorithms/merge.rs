use crate::types::{Algorithm, AlgorithmData, Expected, ExpectedData};
use crate::generate_array::{generate_random, generate_sorted, generate_reversed};

/*
  Merge sort is a divide and conquer algorithm that divides the input array into two halves, 
  calls itself for the two halves, and then merges the two sorted halves. The merge() function is used for merging two halves. 
  The merge(arr, l, m, r) is a key process that assumes that arr[l..m] and arr[m+1..r] are sorted and merges the 
  two sorted sub-arrays into one.

  Stepwise implementation:
  1. Divide the unsorted array into n sub-arrays, each containing 1 element (a sub-array of 1 element is considered sorted).
  2. Repeatedly merge sub-arrays to produce new sorted sub-arrays until there is only 1 sub-array remaining.

  Case distinction:
  - Best case: list is already sorted ~1/2 * n * log(n)
  - Average case: ~c * n * log(n)
  - Worst case: maximum amount of merges ~n * log(n)  

  More info: https://en.wikipedia.org/wiki/Merge_sort
*/
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
    // Outsource the actual sorting to a helper function
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
  // Base case: if low is greater than or equal to high, the array is sorted
  if low < high {
    let mid = (low + high) / 2;
    // Recursively sort the left and right halves
    comparisons += merge_sort(arr, aux, low, mid);
    comparisons += merge_sort(arr, aux, mid + 1, high);
    comparisons += merge(arr, aux, low, mid, high);
  }
  comparisons
}

fn merge(arr: &mut [i32], aux: &mut Vec<i32>, low: usize, mid: usize, high: usize) -> usize  {
  // Copy the array into the auxiliary array
  for i in low..=high {
    aux[i] = arr[i];
  }

  let mut i = low;
  let mut j = mid + 1;

  // Compare the elements of the two halves and merge them into the original array
  // The smaller element is always copied into the original array first
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