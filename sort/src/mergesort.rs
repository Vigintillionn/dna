use plotter::functions::less;

pub fn sort<T: Ord + Copy>(arr: &mut [T]) -> f64 {
  let mut aux = arr.to_vec();
  if arr.len() == 0 {
    return 0.0;
  }
  // Outsource the actual sorting to a helper function
  let mut comparisons: f64 = 0.0;
  merge_sort(arr, &mut aux, 0, arr.len() - 1, &mut comparisons);
  // merge_sort(arr, &mut comparisons);
  comparisons
}

fn merge_sort<T: Ord + Copy>(arr: &mut [T], aux: &mut Vec<T>, low: usize, high: usize, comparisons: &mut f64) -> f64 {
  // let mut comparisons: usize = 0;
  // Base case: if low is greater than or equal to high, the array is sorted
  if low < high {
    let mid = low + (high - low) / 2;
    // Recursively sort the left and right halves
    merge_sort(arr, aux, low, mid, comparisons);
    merge_sort(arr, aux, mid + 1, high, comparisons);
    merge(arr, aux, low, mid, high, comparisons);
  }
  *comparisons
}

fn merge<T: Ord + Copy>(arr: &mut [T], aux: &mut Vec<T>, low: usize, mid: usize, high: usize, comparisons: &mut f64)  {
  // Copy the array into the auxiliary array
  for i in low..=high {
    aux[i] = arr[i];
  }

  let mut i = low;
  let mut j = mid + 1;

  // Compare the elements of the two halves and merge them into the original array
  // The smaller element is always copied into the original array first
  for k in low..=high {


    if i > mid {
      arr[k] = aux[j];
      j += 1;
    } else if j > high {
      arr[k] = aux[i];
      i += 1;
    } else if less(&aux[j], &aux[i], comparisons) {
      arr[k] = aux[j];
      j += 1;
    } else {
      arr[k] = aux[i];
      i += 1;
    }
  }
}