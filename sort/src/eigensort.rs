use plotter::functions::less;

pub fn sort<T: Ord + Copy>(arr: &mut [T], threshold: usize) -> f64 {
  let mut comparisons: f64 = 0.0;
  if arr.len() == 0 {
    return 0.0;
  }

  if arr.len() <= threshold {
    insertion_sort(arr, &mut comparisons);
  } else {
    quicksort(arr, 0, arr.len() - 1, &mut comparisons);
  }

  comparisons
}

fn insertion_sort<T: Ord + Copy>(arr: &mut [T], comparisons: &mut f64) {
  let mut j: i32;
  let mut key: T;
  // loop through the array
  for i in 1..arr.len() {
    key = arr[i];
    j = i as i32 - 1;
    // move elements of arr[0..i-1] that are greater than key to one position ahead of their current position
    while j >= 0 && less(key, arr[j as usize], comparisons) {
      arr[(j + 1) as usize] = arr[j as usize];
      j -= 1;
    }
    // insert the key element at its correct position in the sorted array
    arr[(j + 1) as usize] = key;
  }
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
    // swap(arr, i, j, swaps);
  }
  arr.swap(low, j);
  // swap(arr, low, j, swaps);
  j
}