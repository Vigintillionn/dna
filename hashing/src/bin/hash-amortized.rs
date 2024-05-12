use hashing::{cuckoo::CuckooHash, hashtable::HashTable};
use plotter::{types::{Algorithm, Case}, functions::generators::generate_random};

fn main() {
  // let mut table = CuckooHash::<i32, i32>::new(4);
  // table.insert(&11, &11);
  // table.insert(&7, &7);
  // table.insert(&15, &15);
  // table.insert(&19, &19);
  // table.insert(&23, &23);
  // table.insert(&27, &27);

  // table.print();

  Algorithm::new("Linear Probing Amortized", get_amortized_cost)
    .with_case(
      Case::new("Amortized")
        .with_generator(generate_random)
        .iterations(50)
        .step_size(100)
        .range((0, 20000))
    )
  .run()
  .plot("hashing-amortized")
  .unwrap();
}

fn get_amortized_cost(arr: &mut [i32]) -> f64 {
  let mut table = HashTable::<i32, i32>::new(4);
  arr.iter().for_each(|v| table.insert(v.clone(), v.clone()));
  table.get_amortized_cost()
}