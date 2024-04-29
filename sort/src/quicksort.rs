use plotter::functions::less;

pub fn sort<T: Ord + Copy>(arr: &mut [T]) -> f64 {
  let mut comparisons: f64 = 0.0;
  if arr.len() == 0 {
    return comparisons;
  } 
  quicksort(arr, 0, arr.len() - 1, &mut comparisons);
  comparisons
}

fn quicksort<T: Ord + Copy>(arr: &mut [T], low: usize, high: usize, comparisons: &mut f64) {
  if low < high {
    let pi = hoare_partition(arr, low, high, comparisons);
    if pi > 0 {
      quicksort(arr, low, pi - 1, comparisons);
    }
    quicksort(arr, pi + 1, high, comparisons);
  }
}

fn hoare_partition<T: Ord + Copy>(arr: &mut [T], low: usize, high: usize, comparisons: &mut f64) -> usize {
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