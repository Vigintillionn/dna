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