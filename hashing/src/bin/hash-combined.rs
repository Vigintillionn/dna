use hashing::{cuckoo::CuckooHash, hashtable::HashTable};
use plotter::{types::{Combinator, Algorithm, Case, RED, BLUE}, functions::generators::generate_random};

fn main() {
  Combinator::new("Hash Combined")
    .add_algorithms(vec![
      Algorithm::new("Cuckoo", get_amortized_cost_cuckoo)
        .with_case(
          Case::new("Cuckoo")
            .with_generator(generate_random)
            .iterations(50)
            .step_size(100)
            .range((0, 20000))
        )
        .set_color(BLUE),
      Algorithm::new("Linear Probing", get_amortized_cost_lp)
        .with_case(
          Case::new("Linear Probing")
            .with_generator(generate_random)
            .iterations(50)
            .step_size(100)
            .range((0, 20000))
        )
        .set_color(RED)
    ])
    .run()
    .plot("hash-combined")
    .unwrap();
}

fn get_amortized_cost_cuckoo(arr: &mut [i32]) -> f64 {
  let mut table = CuckooHash::<i32, i32>::new(4);
  arr.iter().for_each(|v| table.insert(&v, &v));
  table.get_amortized_cost()
}

fn get_amortized_cost_lp(arr: &mut [i32]) -> f64 {
  let mut table = HashTable::<i32, i32>::new(4);
  arr.iter().for_each(|v| table.insert(v.clone(), v.clone()));
  table.get_amortized_cost()
}