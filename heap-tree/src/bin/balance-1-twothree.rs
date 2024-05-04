use heap_tree::two_three_tree::{TTTree, TracksDepth};
use plotter::{types::{Plot, Case, RGBColor}, functions::generators::generate_random, test_sorting_algorithm};

fn main() {
  test_sorting_algorithm(
    "Balance 1", 
    vec![
      (Box::new(create_tree_and_insert), RGBColor(0, 0, 255), "Tree depth")
    ],  
    get_cases()
  );
}

fn create_tree_and_insert<T: Ord + Copy>(arr: &mut [T]) -> f64 {
  let mut tree = TTTree::new();

  for i in arr {
    tree.insert(*i);
  }
  tree.depth() as f64
}

fn get_cases() -> Vec<Case> {
  vec![
    Case::new("Two Three")
      .with_generators(vec![generate_random])
      .iterations(100)
      .plots(vec![
        Plot::new(|x| (x as f64).log2(), RGBColor(255, 0, 0), "Upper bound"),
        Plot::new(|x| (x as f64).log2() / (3.0 as f64).log2(), RGBColor(0, 255, 0), "Lower bound")
      ]),
  ]
}