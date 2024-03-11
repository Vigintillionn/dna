use crate::types::{Algorithm, AlgorithmData, Expected, ExpectedData};
use crate::generate_array::{generate_random, generate_sorted, generate_reversed, generate_with_dupes, generate_nearly_sorted};

/*
  Insertion sort is a simple sorting algorithm that builds the final sorted array (or list) one item at a time.
  It is much less efficient on large lists than more advanced algorithms such as quicksort, heapsort, or merge sort.
  However, insertion sort provides several advantages:
  - Simple implementation
  - Efficient for (quite) small data sets

  Stepwise implementation:
  1. Iterate from arr[1] to arr[n] over the array.
  2. Compare the current element (key) to its predecessor.
  3. If the key element is smaller than its predecessor, compare it to the elements before.
  Move the greater elements one position up to make space for the swapped element.

  Case distinction:
  - Best case: list is already sorted ~n
  - Average case: ~1/4 * n^2
  - Worst case: ~1/2 * n^2

  More info: https://en.wikipedia.org/wiki/Insertion_sort
*/

pub struct InsertionSort {
  pub name: String,
}

impl Algorithm for InsertionSort {
  fn new() -> InsertionSort {
    Self { name: "insertion".to_string() }
  }

  fn sort<T: Ord + Copy>(&self, arr: &mut [T]) -> usize {
    let mut comparisons: usize = 0;
    let mut j: i32;
    let mut key: T;
    // loop through the array
    for i in 1..arr.len() {
      key = arr[i];
      j = i as i32 - 1;
      comparisons += 1;
      // move elements of arr[0..i-1] that are greater than key to one position ahead of their current position
      while j >= 0 && arr[j as usize] > key {
        comparisons += 1;
        arr[(j + 1) as usize] = arr[j as usize];
        j -= 1;
      }
      // insert the key element at its correct position in the sorted array
      arr[(j + 1) as usize] = key;
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
          factor: 0.25,
        },
        iterations: 10
      }, 
      AlgorithmData {
        name: "best".to_string(),
        generator: generate_sorted,
        expected: ExpectedData {
          function: Expected::Linear,
          factor: 1.0,
        },
        iterations: 1
      }, 
      AlgorithmData {
        name: "worst".to_string(),
        generator: generate_reversed,
        expected: ExpectedData {
          function: Expected::Quadratic,
          factor: 0.5,
        },
        iterations: 1
      },
      AlgorithmData {
        name: "dupes".to_string(),
        generator: generate_with_dupes,
        expected: ExpectedData {
          function: Expected::Quadratic,
          factor: 0.25,
        },
        iterations: 10
      },
      AlgorithmData {
        name: "nearly-sorted".to_string(),
        generator: |i| generate_nearly_sorted(i, 10),
        expected: ExpectedData {
          function: Expected::Linear,
          factor: 3.25,
        },
        iterations: 10
      }
    ]
  }

  fn get_name(&self) -> String {
    self.name.clone()
  }
}