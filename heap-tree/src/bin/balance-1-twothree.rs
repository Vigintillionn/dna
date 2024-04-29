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
    Case {
      name: "Two Three".to_string(),
      generator: Box::new(generate_random),
      iterations: 100,
      plots: vec![
        Plot {
          color: RGBColor(255, 0, 0),
          func: Box::new(|x| (x as f64).log2()),
          name: "Upper bound".to_string()
        },
        Plot {
          color: RGBColor(0, 255, 0),
          func: Box::new(|x| (x as f64).log2() / (3.0 as f64).log2()),
          name: "Lower bound".to_string()
        }
      ],
    },
  ]
}