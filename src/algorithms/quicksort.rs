use crate::types::{Algorithm, AlgorithmData, Expected, ExpectedData};
use crate::generate_array::{less, generate_random, generate_sorted, quick_best};

pub struct Quicksort {
  pub name: String,
}

impl Algorithm for Quicksort {
  fn new() -> Quicksort {
    Self { name: "quicksort".to_string() }
  }

  fn sort<T: Ord + Copy>(&self, arr: &mut [T]) -> usize {
    let mut comparisons: usize = 0;
    if arr.len() == 0 {
      return comparisons;
    } 
    quicksort(arr, 0, arr.len() - 1, &mut comparisons);
    comparisons
  }

  fn get_cases(&self) -> Vec<AlgorithmData> {
    vec![
      AlgorithmData {
        name: Box::new(String::from("Worst")),
        generator: generate_sorted,
        expected: ExpectedData {
          function: Expected::Quadratic,
          factor: 0.5,
        },
        iterations: 1,
        calculate_distance: false,
        plots: vec![]
      },
      AlgorithmData {
        name: Box::new(String::from("Average")),
        generator: generate_random,
        expected: ExpectedData {
          function: Expected::NLogN,
          factor: 1.39,
        },
        iterations: 100,
        calculate_distance: false,
        plots: vec![
          |x| (1.24 * x as f32 * (x as f32).log2()) as usize
        ]
      },
      AlgorithmData {
        name: Box::new(String::from("Best")),
        generator: quick_best,
        expected: ExpectedData {
          function: Expected::NLogN,
          factor: 1.0,
        },
        iterations: 1,
        calculate_distance: false,
        plots: vec![
          |x| (x as f32 * (x as f32).log2() - x as f32) as usize
        ]
      },
    ]
  }

  fn get_name(&self) -> String {
    self.name.clone()
  }
}

fn quicksort<T: Ord + Copy>(arr: &mut [T], low: usize, high: usize, comparisons: &mut usize) {
  if low < high {
    let pi = hoare_partition(arr, low, high, comparisons);
    if pi > 0 {
      quicksort(arr, low, pi - 1, comparisons);
    }
    quicksort(arr, pi + 1, high, comparisons);
  }
}

// HOARE
fn hoare_partition<T: Ord + Copy>(arr: &mut [T], low: usize, high: usize, comparisons: &mut usize) -> usize {
  let pivot = arr[low];
  let mut i = low;
  let mut j = high+1;

  loop {
    while less(arr[i+1], pivot, comparisons) {
      i += 1;
      if i == high {
        break;
      }
    }

    i += 1;

    while less(pivot, arr[j-1], comparisons) {
      j -= 1;
      if j == low {
        break;
      }
    }

    j -= 1;

    if i >= j {
      break;
    }
    arr.swap(i, j);
  }
  arr.swap(low, j);
  j
}