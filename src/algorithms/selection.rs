use crate::types::{Algorithm, AlgorithmData, Expected, ExpectedData};
use crate::generate_array::{generate_random, generate_sorted, generate_reversed};

/*
  Selection sort is a simple sorting algorithm that works by repeatedly finding the minimum element 
  from the unsorted part of the array and putting it at the beginning. The algorithm maintains two subarrays 
  in a given array.

  Stepwise implementation:
  1. The subarray which is already sorted.
  2. Remaining subarray which is unsorted.
  3. In every iteration of selection sort, the minimum element from the unsorted subarray is picked and moved to the sorted subarray.

  Case distinction:
  - Best case: ~1/2 * n^2
  - Average case: ~1/2 * n^2
  - Worst case: ~1/2 * n^2

  More info: https://en.wikipedia.org/wiki/Selection_sort
*/
pub struct SelectionSort {
  pub name: String,
}

impl Algorithm for SelectionSort {
  fn new() -> SelectionSort {
    Self { name: "selection".to_string() }
  }

  fn sort<T: Ord + Copy>(&self, arr: &mut [T]) -> usize {
    let mut min_index: usize;
    let mut temp: T;
    let mut comparisons: usize = 0;

    // One by one move boundary of unsorted subarray
    for i in 0..arr.len() {
      min_index = i;
      // Find the minimum element in unsorted array
      for j in (i + 1)..arr.len() {
        comparisons += 1;
        // If arr[j] is smaller, then it is the new minimum
        if arr[j] < arr[min_index] {
          min_index = j;
        }
      }
      // Swap the found minimum element with the first element
      temp = arr[i];
      arr[i] = arr[min_index];
      arr[min_index] = temp;
    }
    comparisons
  }

  fn get_cases(&self) -> Vec<AlgorithmData> {
    vec![
      AlgorithmData {
        name: "average".to_string(),
        generator: generate_random,
        expected: ExpectedData {
          function: Expected::Quadratic,
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
          factor: 0.5,
        }
      }
    ]
  }

  fn get_name(&self) -> String {
    self.name.clone()
  }
}