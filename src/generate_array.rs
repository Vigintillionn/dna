use rand::prelude::SliceRandom;

pub fn less<T: Ord>(a: T, b: T, comparisons: &mut usize) -> bool {
  *comparisons += 1;
  a < b
}

pub fn generate_random(length: usize) -> Vec<i32> {
  let mut arr: Vec<i32> = Vec::new();
  for _ in 0..length {
      arr.push(rand::random::<i32>());
  }
  arr
}

pub fn generate_sorted(length: usize) -> Vec<i32> {
  let mut arr: Vec<i32> = Vec::new();
  for i in 0..length {
      arr.push(i.try_into().unwrap());
  }
  arr
}

pub fn generate_reversed(length: usize) -> Vec<i32> {
  let mut arr: Vec<i32> = Vec::new();
  for i in (0..length).rev() {
      arr.push(i.try_into().unwrap());
  }
  arr
}

pub fn generate_interleaved(length: usize) -> Vec<i32> {
  if length <= 0 {
    return vec![];
  }
  let mut arr: Vec<i32> = (0..length).map(|x| x as i32).collect();
  let n = arr.len() - 1;
  make_interleaved(&mut arr, 0, n);
  arr
}

pub fn generate_with_dupes(length: usize) -> Vec<i32> {
  let mut arr: Vec<i32> = Vec::new();
  let dupes = length / 10;
  for i in 0..dupes {
    for _ in 0..10 {
      arr.push(i.try_into().unwrap());
    }
  }
  shuffle_array(&mut arr);
  arr
}

pub fn generate_nearly_sorted(length: usize, chunk_size: usize) -> Vec<i32> {
  let mut arr: Vec<i32> = (0..length).map(|x| x as i32).collect();

  for chunk in arr.chunks_mut(chunk_size) {
    shuffle_slice(chunk);
  }

  arr
}

pub fn shuffle_array(arr: &mut Vec<i32>) {
  let mut rng = rand::thread_rng();
  arr.shuffle(&mut rng);
}

pub fn shuffle_slice(arr: &mut [i32]) {
  let mut rng = rand::thread_rng();
  arr.shuffle(&mut rng);
}

fn make_interleaved(arr: &mut [i32], l: usize, r: usize) {
  if l < r {
    let m = l + (r - l) / 2;

    let mut left = vec![0; m - l + 1];
    let mut right = vec![0; r - m];

    for i in 0..=(m - l) {
      left[i] = arr[i * 2];
    }

    for i in 0..(r - m) {
      right[i] = arr[i * 2 + 1];
    }

    make_interleaved(&mut left, l, m);
    make_interleaved(&mut right, m + 1, r);

    let mut i = 0;
    for &item in &left {
      arr[i] = item;
      i += 1;
    }

    for j in 0..(r - m) {
      arr[i + j] = right[j];
    }
  }
}

// https://stackoverflow.com/questions/32556521/java-quicksort-best-case-array-generation
pub fn quick_best(length: usize) -> Vec<i32> {
  let mut sorted = (1..=length).map(|x| x as i32).collect();
  quick_best_rec(&mut sorted, 0, length);
  sorted
}

fn quick_best_rec(array: &mut Vec<i32>, start: usize, end: usize) {
  let count = end - start;
  if count < 3 {
      return;
  }

  let middle = start + (count - 1) / 2;

  quick_best_rec(array, start, middle);
  array.swap(start as usize, middle as usize);
  quick_best_rec(array, middle + 1, end);
}