// use heap_tree::rbtree::RBTree;
use heap_tree::rbtree::Llrb;
use plotter::{types::{Case, RGBColor, Plot}, functions::generators::generate_random, test_sorting_algorithm};

fn main() {
  test_sorting_algorithm(
    "Balance 2", 
    vec![
      (Box::new(create_tree_and_insert), RGBColor(0, 0, 255), "Red/Black ratio * 1000"),
      (Box::new(calculate_blacks), RGBColor(0, 0, 0), "# Black links"),
      (Box::new(calculate_reds), RGBColor(255, 0, 0), "# Red links")
    ],  
    get_cases()
  );
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

fn get_cases() -> Vec<Case> {
  vec![
    Case::new("Black Red")
      .with_generators(vec![generate_random])
      .iterations(100)
      .plots(vec![
        Plot::new(|x| 0.75 * x as f64, RGBColor(0, 0, 0), "0.75n"),
        Plot::new(|x| 0.25 * x as f64, RGBColor(255, 0, 0), "0.25n"),
        Plot::new(|_| 333.0 as f64, RGBColor(0, 0, 255), "333"),
      ])
  ]
}