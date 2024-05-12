use hashing::hashtable::HashTable;
use plotter::types::{Algorithm, Plot, Case, RED};
use rand::seq::SliceRandom;

const SIZE: usize = 1024;

fn main() {
  Algorithm::new("Hashing Hits", calculate)
    .with_case(
      Case::new("Hits")
        .with_generator(|size| (0..size as i32).collect())
        .iterations(50)
        .range((1, 70))
        .step_size(1)
        .plots(vec![
          Plot::new(|alpha| 0.5 * (1.0 + 1.0 / (1.0 - (alpha as f64 / 100.0))), RED, "Expected")
        ])
    )
    .run()
    .plot("hashing-hits")
    .unwrap();
}

fn calculate(arr: &mut [i32]) -> f64 {
  let load_factor = arr.len() as f64 / 100.0;
  let (table, inserted) = create_table_with_load(load_factor);
  let mut rng = rand::thread_rng();
  let to_test = inserted.choose(&mut rng).unwrap();
  let res = table.find(&to_test);
  res as f64
}

fn create_table_with_load(expected_load: f64) -> (HashTable<i32, i32>, Vec<i32>) {
  let mut table = HashTable::<i32, i32>::new(SIZE);
  let mut rng = rand::thread_rng();
  let mut permutation = (0..20*SIZE as i32).collect::<Vec<i32>>();
  permutation.shuffle(&mut rng);

  // let mut to_test: i32 = permutation[0];
  let mut inserted: Vec<i32> = vec![];

  let mut i = 0;
  while table.get_load_factor() < expected_load {
    let int: i32 = permutation[i]; //rng.gen::<i32>(); 
    table.insert(int, int);
    inserted.push(int);
    i += 1;
  }

  (table, inserted)
}