use plotters::prelude::Color;

#[derive(Clone)]
pub enum Expected {
  Linear,
  Quadratic,
  NLogN,
}

#[derive(Clone)]
pub struct ExpectedData {
  pub function: Expected,
  pub factor: f64,
}

// #[derive(Clone)]
pub struct AlgorithmData {
  pub name: Box<String>,
  pub generator: fn(usize) -> Vec<i32>,
  pub expected: ExpectedData,
  pub iterations: usize,
  pub calculate_distance: bool,
  pub plots: Vec<fn(usize) -> usize>
}

pub trait Algorithm {
  fn new() -> Self;
  fn sort<T: Ord + Copy>(&self, arr: &mut [T]) -> usize;
  fn get_cases(&self) -> Vec<AlgorithmData>;
  fn get_name(&self) -> String;
}