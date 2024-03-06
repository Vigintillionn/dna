use crate::types::{Algorithm, AlgorithmData, Expected, ExpectedData};
use crate::generate_array::{generate_random, generate_sorted, generate_reversed};

pub struct InsertionSort {
  pub name: String,
}

impl Algorithm for InsertionSort {
  fn new() -> InsertionSort {
    Self { name: "insertion".to_string() }
  }

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

  fn get_cases(&self) -> Vec<AlgorithmData> {
    vec![
      AlgorithmData {
        name: "average".to_string(),
        generator: generate_random,
        expected: ExpectedData {
          function: Expected::Quadratic,
          factor: 0.25,
        }
      }, 
      AlgorithmData {
        name: "best".to_string(),
        generator: generate_sorted,
        expected: ExpectedData {
          function: Expected::Linear,
          factor: 1.0,
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