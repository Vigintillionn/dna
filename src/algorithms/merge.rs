use crate::types::Sort;

pub struct MergeSort;

impl MergeSort {
  pub fn new() -> MergeSort {
    MergeSort
  }
}

impl Sort<i32> for MergeSort {
  fn sort(&self, arr: &mut [i32]) -> usize {
    let mut aux = arr.to_vec();
    let comparisons = merge_sort(arr, &mut aux, 0, arr.len() - 1);
    comparisons
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

