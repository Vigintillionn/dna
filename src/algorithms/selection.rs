use crate::types::{Algorithm, AlgorithmData, Expected};
use crate::generate_array::{generate_random, generate_sorted, generate_reversed};

pub struct SelectionSort {
  pub name: String,
}

impl Algorithm for SelectionSort {
  fn new() -> SelectionSort {
    Self { name: "selection".to_string() }
  }

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

  fn get_cases(&self) -> Vec<AlgorithmData> {
    vec![
      AlgorithmData {
        name: "average".to_string(),
        generator: generate_random,
        expected: Expected::Quadratic,
        factor: 0.5,
      }, 
      AlgorithmData {
        name: "best".to_string(),
        generator: generate_sorted,
        expected: Expected::Quadratic,
        factor: 0.5,
      }, 
      AlgorithmData {
        name: "worst".to_string(),
        generator: generate_reversed,
        expected: Expected::Quadratic,
        factor: 0.5,
      }
    ]
  }

  fn get_name(&self) -> String {
    self.name.clone()
  }
}