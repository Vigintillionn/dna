// use heap_tree::rbtree::RBTree;
use heap_tree::rbtree::Llrb;
use plotter::{types::{Case, Plot, Combinator, Algorithm, RED, BLACK, BLUE}, functions::generators::generate_random};

fn main() {
  Combinator::new("Balance 2")
    .add_algorithms(vec![
      Algorithm::new("Red Black Ratio", create_tree_and_insert)
      .with_case(
        Case::new("Average")
        .with_generator(generate_random)
        .iterations(100)
        .plots(vec![
          Plot::new(|_| 333.0 as f64, BLUE, "0.75n")
        ])
        .set_color(BLUE),
      ),
      Algorithm::new("# Black Links", calculate_blacks)
        .with_case(
          Case::new("Average")
          .with_generator(generate_random)
          .iterations(100)
          .plots(vec![
            Plot::new(|x| 0.75 * x as f64, BLACK, "0.75n")
          ])
          .set_color(BLACK),
        ),
      Algorithm::new("# Red Links", calculate_reds)
      .with_case(
        Case::new("Average")
        .with_generator(generate_random)
        .iterations(100)
        .plots(vec![
          Plot::new(|x| 0.25 * x as f64, RED, "0.75n")
        ])
        .set_color(RED),
      ),
    ])
    .run()
    .plot("balance-2")
    .unwrap();
}

fn create_tree_and_insert<T: Ord + Copy>(arr: &mut [T]) -> f64 {
  let mut tree: Llrb<T, T> = Llrb::new("test");

  for i in arr {
    tree.set(*i, *i);
  }

  if tree.iter().count() == 0 {
    return 0.0;
  }

  let blacks = tree.n_blacks();
  let reds = tree.iter().count() - blacks;
  reds as f64 / blacks as f64 * 1000.0
}

fn calculate_blacks<T: Ord + Copy>(arr: &mut [T]) -> f64 {
  let mut tree: Llrb<T, T> = Llrb::new("test");

  for i in arr {
    tree.set(*i, *i);
  }

  if tree.iter().count() == 0 {
    return 0.0;
  }

  tree.n_blacks() as f64
}

fn calculate_reds<T: Ord + Copy>(arr: &mut [T]) -> f64 {
  let mut tree: Llrb<T, T> = Llrb::new("test");

  for i in arr {
    tree.set(*i, *i);
  }

  if tree.iter().count() == 0 {
    return 0.0;
  }

  tree.iter().count() as f64 - tree.n_blacks() as f64
}