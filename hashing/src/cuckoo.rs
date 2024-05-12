use std::{fmt::Display, hash::{Hasher, Hash}};

const MAX_LOOP: usize = 16;

pub struct CuckooHash<K, V> {
  table_one: Vec<Option<(K,V)>>,
  table_two: Vec<Option<(K,V)>>,
  size: usize,
  probes: usize
}

impl<K: ToString + Clone + PartialEq + Hash, V: Clone> CuckooHash<K, V> {
  pub fn new(size: usize) -> CuckooHash<K, V> {
    CuckooHash {
      table_one: vec![None; size / 2],
      table_two: vec![None; size / 2],
      size: size / 2,
      probes: 0
    }  
  }

  pub fn get_amortized_cost(&self) -> f64 {
    self.probes as f64 / self.get_filled() as f64
  }

  pub fn get_filled(&self) -> usize {
    self.table_one
      .iter()
      .filter(|e| e.is_some())
      .count() 
    + self.table_two
      .iter()
      .filter(|e| e.is_some())
      .count()
  }

  pub fn get_size(&self) -> usize {
    self.size
  }

  pub fn get(&mut self, key: &K) -> Option<&V> {
    let hash_one = self.hash_one(key);
    let hash_two = self.hash_two(key);

    // T_1[h_1(k)] or T_2[h_2(k)]
    self.probes += 1;
    if let Some((k, v)) = &self.table_one[hash_one] {
      if k == key {
        return Some(v)
      }
    }

    self.probes += 1;
    if let Some((k, v)) = &self.table_two[hash_two] {
      if k == key {
        return Some(v)
      }
    }

    None
  }

  pub fn find(&self, key: &K) -> usize {
    let hash_one = self.hash_one(key);
    let hash_two = self.hash_two(key);

    let mut probes = 1;
    // T_1[h_1(k)] or T_2[h_2(k)]
    if let Some((k, _)) = &self.table_one[hash_one] {
      if k == key {
        return probes;
      }
    }

    probes += 1;
    if let Some((k, _)) = &self.table_two[hash_two] {
      if k == key {
        return probes;
      }
    }

    probes
  }

  pub fn insert(&mut self, key: &K, value: &V) {
    // If the key already exists, do nothing
    if self.get(key).is_some() {
      return;
    }

    // Initialize the key-value pair to insert
    let mut to_insert = (key.clone(), value.clone());

    // Loop for MAX_LOOP times
    for _ in 0..MAX_LOOP {
      // Insert the key-value pair into the first table
      let hash_one = self.hash_one(&to_insert.0);
      self.probes += 1;
      if self.table_one[hash_one].is_none() {
        self.table_one[hash_one] = Some(to_insert);
        // If no collision, return
        return;
      }

      // If there is a collision, swap the key-value pair
      let (k, v) = self.table_one[hash_one].take().unwrap();
      self.table_one[hash_one] = Some(to_insert);

      // Update the key-value pair to insert
      to_insert = (k.clone(), v.clone());

      // Insert the key-value pair into the second table
      let hash_two = self.hash_two(&k);
      self.probes += 1;
      if self.table_two[hash_two].is_none() {
        self.table_two[hash_two] = Some(to_insert);
        // If no collision, return
        return;
      }

      // If there is a collision, swap the key-value pair
      let (k, v) = self.table_two[hash_two].take().unwrap();
      self.table_two[hash_two] = Some(to_insert);

      // Update the key-value pair to insert
      to_insert = (k.clone(), v.clone());
    }

    // Rehashing
    // If the loop reaches MAX_LOOP, rehash the table
    // Insert the key-value pair again
    self.resize();
    self.insert(&to_insert.0, &to_insert.1);
  }

  fn rehash(&mut self) {
    self.resize(); // Resizing changes the hash and forces all elements to be rehashed
  }

  fn resize(&mut self) {
    // Create a new table with double the size
    // Rehash all the elements in the old table
    // Update the size
    let mut new_table = CuckooHash::new(self.size * 4);

    for i in 0..self.size {
      if let Some((k, v)) = &self.table_one[i] {
        new_table.insert(k, v);
      }

      if let Some((k, v)) = &self.table_two[i] {
        new_table.insert(k, v);
      }
    }

    self.table_one = new_table.table_one;
    self.table_two = new_table.table_two;
    self.size = new_table.size;
    self.probes += new_table.probes;
  }

  fn hash_one(&self, key: &K) -> usize {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish() as usize % self.get_size()

    // let k = key.to_string().parse::<usize>().unwrap();
    // k % self.size // k mod size
  }

  fn hash_two(&self, key: &K) -> usize {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish() as usize / self.get_size() % self.get_size()

    // let k = key.to_string().parse::<usize>().unwrap();
    // (k / self.size) % self.size // floor(k/size) mod size
  }
}

impl<K: Clone + ToString + PartialEq + Display + Hash, V: Clone + Display> CuckooHash<K, V> {
  pub fn print(&self) {
    self.print_table(&self.table_one, "T1");
    self.print_table(&self.table_two, "T2");
  }

  fn print_table(&self, table: &Vec<Option<(K, V)>>, table_name: &str) {
    for i in 0..self.get_size() {
      if table[i].is_some() {
        let (k, _) = table[i].clone().unwrap();
        println!("Table: {table_name}, Index: {i}, Key: {k}")
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_cuckoo_hash() {
    let mut hash = CuckooHash::new(5);
    hash.insert(&"1", &"one");
    hash.insert(&"2", &"two");
    hash.insert(&"3", &"three");
    hash.insert(&"4", &"four");
    hash.insert(&"5", &"five");

    assert_eq!(hash.get(&"1"), Some(&"one"));
    assert_eq!(hash.get(&"2"), Some(&"two"));
    assert_eq!(hash.get(&"3"), Some(&"three"));
    assert_eq!(hash.get(&"4"), Some(&"four"));
    assert_eq!(hash.get(&"5"), Some(&"five"));
  }
}