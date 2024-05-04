use heap_tree::rbtree::Llrb;
use plotter::{test_sorting_algorithm, types::{RGBColor, Case}, functions::generators::generate_random};

fn main() {
  test_sorting_algorithm(
    "Balance 2", 
    vec![
      (Box::new(ratio_lrot), RGBColor(255, 0, 0), "Left Rotations"),
      (Box::new(ratio_rrot), RGBColor(0, 0, 0), "Right Rotations"),
      (Box::new(ratio_flips), RGBColor(0, 0, 255), "Color Flips"),
      (Box::new(ratio_total), RGBColor(0, 255, 0), "Total Actions")
    ], get_cases()
  )
}

fn ratio_lrot(array: &mut [i32]) -> f64 {
  let mut tree = Llrb::new("tree");

  for &value in array.iter() {
    tree.set(value, value);
  }

  (tree.lrotate_count as f64 / array.len() as f64) * 1000.0
}

fn ratio_rrot(array: &mut [i32]) -> f64 {
  let mut tree = Llrb::new("tree");

  for &value in array.iter() {
    tree.set(value, value);
  }

  (tree.rrotate_count as f64 / array.len() as f64) * 1000.0
}

fn ratio_flips(array: &mut [i32]) -> f64 {
  let mut tree = Llrb::new("tree");

  for &value in array.iter() {
    tree.set(value, value);
  }

  (tree.flip_count as f64 / array.len() as f64) * 1000.0
}

fn ratio_total(array: &mut [i32]) -> f64 {
  let mut tree = Llrb::new("tree");

  for &value in array.iter() {
    tree.set(value, value);
  }

  ((tree.flip_count + tree.rrotate_count + tree.lrotate_count) as f64 / array.len() as f64) * 1000.0
}

fn get_cases() -> Vec<Case> {
  vec![
    Case::new("Actions")
      .iterations(100)
      .with_generators(vec![generate_random])
  ]
}