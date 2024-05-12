// use heap_tree::rbtree::RBTree;
use heap_tree::rbtree::Llrb;
use plotter::{types::{Plot, Case, Algorithm, RED, GREEN}, functions::generators::generate_random};

fn main() {
  Algorithm::new("Balance 1", create_tree_and_insert)
    .with_case(
      Case::new("Left Leaning Red Black")
        .with_generator(generate_random)
        .iterations(100)
        .plots(vec![
          Plot::new(|x| (x as f64).log2(), RED, "Upper bound"),
          Plot::new(|x| (x as f64).log2() / (3.0 as f64).log2(), GREEN, "Lower bound")
        ])
    )
    .run()
    .plot_seperate(vec!["balance-1-llrb"])
    .unwrap();
}

fn create_tree_and_insert<T: Ord + Copy>(arr: &mut [T]) -> f64 {
  let mut tree: Llrb<T, T> = Llrb::new("test");

  for i in arr {
    tree.set(*i, *i);
  }
  tree.black_depth() as f64
}