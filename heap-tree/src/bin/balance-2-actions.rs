use heap_tree::rbtree::Llrb;
use plotter::{types::{Case, Combinator, Algorithm, RED, GREEN, BLUE, BLACK}, functions::generators::generate_random};

fn main() {
  Combinator::new("Balance 2")
    .add_algorithms(vec![
      Algorithm::new("Left Rotations", ratio_lrot)
        .with_case(Case::new("Actions").with_generator(generate_random).iterations(100).set_color(RED)),
      Algorithm::new("Right Rotations", ratio_rrot)
        .with_case(Case::new("Actions").with_generator(generate_random).iterations(100).set_color(BLACK)),
      Algorithm::new("Color Flips", ratio_flips)
        .with_case(Case::new("Actions").with_generator(generate_random).iterations(100).set_color(BLUE)),
      Algorithm::new("Total Actions", ratio_total)
        .with_case(Case::new("Actions").with_generator(generate_random).iterations(100).set_color(GREEN))
    ])
    .run()
    .plot("balance-2-actions")
    .unwrap();
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