use hashing::hashtable::HashTable;
use plotter::types::{Algorithm, Case, Plot, RED, Combinator};
use rand::seq::SliceRandom;

fn main() {
    Algorithm::new("Hashing Misses", |arr| calculate(arr, 0.6))
      .with_case(
        Case::new("Misses")
          .with_generator(|size| (0..size as i32).collect())
          .iterations(50)
          .step_size(50)
          .range((0, 5000))
          .plots(vec![
            Plot::new(|_| expected_hit(0.6), RED, "Expected")
          ])
      )
      .run()
      .plot("hashing-misses-0.6")
      .unwrap();
}

fn expected_hit(alpha: f64) -> f64 {
  0.5 * (1.0 + 1.0 / (1.0 - alpha).powi(2))
}

fn calculate(arr: &mut [i32], alpha: f64) -> f64 {
  if arr.len() == 0 {
    return 0.0;
  }

  let (table, _) = create_table_with_load(alpha, arr.len());
  // let mut rng = rand::thread_rng();
  // let to_test = inserted.choose(&mut rng).unwrap();
  let to_test = (20 * arr.len() + 1) as i32;
  let res = table.find(&to_test);

  res as f64
}

fn create_table_with_load(expected_load: f64, size: usize) -> (HashTable<i32, i32>, Vec<i32>) {
  let mut table = HashTable::<i32, i32>::new(size);
  let mut rng = rand::thread_rng();
  let mut permutation = (0..20*size as i32).collect::<Vec<i32>>();
  permutation.shuffle(&mut rng);

  // let mut to_test: i32 = permutation[0];
  let mut inserted: Vec<i32> = vec![];

  let mut i = 0;
  while table.get_load_factor() < expected_load {
    let int: i32 = permutation[i];//rng.gen::<i32>(); 
    table.insert(int, int);
    inserted.push(int);
    i += 1;
  }

  (table, inserted)
}

/*
    // .with_hash_fn(|key| {
    //   let mut hash = 0;
    //   for byte in key.as_bytes() {
    //     hash += *byte as i32;
    //   }
    //   hash
    // });

*/