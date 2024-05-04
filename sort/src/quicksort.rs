use plotter::functions::{less, swap};

pub fn sort<T: Ord + Copy>(arr: &mut [T], ret_comparisons: bool) -> f64 {
  let mut comparisons: f64 = 0.0;
  let mut swaps: f64 = 0.0;
  if arr.len() == 0 {
    if ret_comparisons {
      return comparisons;
    } else {
      return swaps;
    }

  } 
  quicksort(arr, 0, arr.len() - 1, &mut comparisons, &mut swaps);
  if ret_comparisons {
    comparisons
  } else {
    swaps
  }
}

fn quicksort<T: Ord + Copy>(arr: &mut [T], low: usize, high: usize, comparisons: &mut f64, swaps: &mut f64) {
  if low < high {
    let pi = hoare_partition(arr, low, high, comparisons, swaps);
    if pi > 0 {
      quicksort(arr, low, pi - 1, comparisons, swaps);
    }
    quicksort(arr, pi + 1, high, comparisons, swaps);
  }
}

fn hoare_partition<T: Ord + Copy>(arr: &mut [T], low: usize, high: usize, comparisons: &mut f64, swaps: &mut f64) -> usize {
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
    // arr.swap(i, j);
    swap(arr, i, j, swaps);
  }
  // arr.swap(low, j);
  swap(arr, low, j, swaps);
  j
}