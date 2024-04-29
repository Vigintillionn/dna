pub use plotters::style::RGBColor;

pub type PlotFunction = Box<dyn Fn(usize) -> f64 + Send + Sync>;
pub type SortFunction<T> = Box<dyn Fn(&mut [T]) -> f64 + Send + Sync>;
pub type Algorithm = Box<dyn Fn() -> f64 + Send + Sync>;

pub struct Plot {
  pub func: PlotFunction,
  pub color: RGBColor,
  pub name: String
}

pub struct Case {
  pub name: String,
  pub generator: Box<dyn Fn(usize) -> Vec<i32> + Send + Sync>,
  pub iterations: usize,
  pub plots: Vec<Plot>
}