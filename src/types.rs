pub trait Sort<T> {
  fn sort(&self, arr: &mut [T]) -> usize;
}

pub struct AlgorithmData {
  pub name: String,
  pub generator: fn(usize) -> Vec<i32>,
}