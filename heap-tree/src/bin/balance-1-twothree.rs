use heap_tree::two_three_tree::{TTTree, TracksDepth};
use plotter::{types::{Plot, Case, Algorithm, RED, GREEN}, functions::generators::generate_random};

fn main() {
  Algorithm::new("Balance 1", create_tree_and_insert)
    .with_case(
      Case::new("Two Three")
        .with_generator(generate_random)
        .iterations(100)
        .plots(vec![
          Plot::new(|x| (x as f64).log2(), RED, "Upper bound"),
          Plot::new(|x| (x as f64).log2() / (3.0 as f64).log2(), GREEN, "Lower bound")
        ])
    )
    .run()
    .plot_seperate(vec!["balance-1-twothree"])
    .unwrap();
}

fn create_tree_and_insert<T: Ord + Copy>(arr: &mut [T]) -> f64 {
  let mut tree = TTTree::new();

  for i in arr {
    tree.insert(*i);
  }
  tree.depth() as f64
}