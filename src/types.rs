#[derive(Clone)]
pub enum Expected {
  Linear,
  Quadratic,
  NLogN,
}

#[derive(Clone)]
pub struct AlgorithmData {
  pub name: String,
  pub generator: fn(usize) -> Vec<i32>,
  pub expected: Expected,
  pub factor: f64,
}

pub trait Algorithm {
  fn new() -> Self;
  fn sort(&self, arr: &mut [i32]) -> usize;
  fn get_cases(&self) -> Vec<AlgorithmData>;
  fn get_name(&self) -> String;
}