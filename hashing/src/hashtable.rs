use std::{hash::{Hash, Hasher}, fmt::Display};

pub struct HashTable<K, V> {
  table: Vec<Option<(K, V)>>,
  size: usize,
  // load_factor: f64,
  resize: bool,
  probes: usize
}

impl<K: Clone + Hash + PartialEq + ToString, V: Clone> HashTable<K, V> {
  pub fn new(size: usize) -> Self {
    HashTable {
      table: vec![None; size],
      size,
      // load_factor: 0.5,
      resize: true,
      probes: 0
    }
  }

  pub fn get_size(&self) -> usize {
    self.size
  }

  pub fn get_load_factor(&self) -> f64 {
    self.get_filled() as f64 / self.get_size() as f64
  }

  pub fn get_amortized_cost(&self) -> f64 {
    self.probes as f64 / self.get_filled() as f64
  }

  fn get_filled(&self) -> usize {
    self.table.iter().filter(|x| x.is_some()).count()
  }

  pub fn no_resize(mut self) -> HashTable<K, V> {
    self.resize = false;
    self
  }

  pub fn insert(&mut self, key: K, value: V) {
    // Get index from hash
    let index = self.hash(&key);

    // Insert key-value pair if index is empty
    self.probes += 1;
    if self.table[index].is_none() {
      self.table[index] = Some((key, value));
    } else {
      // Linear probing
      let mut i = index;
      while i != index {
        if i >= self.get_size() {
          i = 0;
        }

        self.probes += 1;
        if self.table[i].is_none() {
          self.table[i] = Some((key, value));
          break;
        }

        i += 1;
      }

      self.probes += 1; 
    }

    // Resize table if load factor is greater than 0.75
    if self.get_load_factor() > 0.5 && self.resize {
      self.resize();
    }
  }

  pub fn get(&self, key: &K) -> Option<&V> {
    // Get index from hash
    let index = self.hash(&key);

    // Check if key exists at index
    if self.table[index].is_some() {
      let (k, v) = self.table[index].as_ref().unwrap();
      if k == key {
        return Some(v);
      }
    } else {
      return None;
    }

    // Linear probing
    let mut i = index + 1;
    while i != index {
      if i >= self.get_size() {
        i = 0;
      }

      if self.table[i].is_some() {
        let (k, v) = self.table[i].as_ref().unwrap();
        if k == key {
          return Some(v);
        }
      } else {
        return None;
      }

      i += 1;
    }

    None
  }

  pub fn find(&self, key: &K) -> usize {
    let mut probes = 1;
    // Get index from hash
    let index = self.hash(&key);

    // Check if key exists at index
    if self.table[index].is_some() {
      let (k, _) = self.table[index].as_ref().unwrap();
      if k == key {
        println!("Hit");
        return 1;
      }
    } else {
      return 1;
    }

    // Linear probing
    let mut i = index + 1;
    while i != index {
      if i >= self.get_size() {
        i = 0;
      }

      probes += 1;

      if self.table[i].is_some() {
        let (k, _) = self.table[i].as_ref().unwrap();
        if k == key {
          println!("Hit");
          return probes;
        }
      } else {
        return probes + 1;
      }

      i += 1;
    }

    probes
  }

  fn resize(&mut self) {
    // Create new table with double the size
    let mut new_table = HashTable::<K,V>::new(self.get_size() * 2);

    // Insert key-value pairs from old table to new table
    for i in 0..self.get_size() {
      if self.table[i].is_some() {
        let (key, value) = self.table[i].clone().unwrap();
        new_table.insert(key, value);
      }
    }

    // Update table and size
    self.table = new_table.table;
    self.size = self.get_size() * 2;
    self.probes += new_table.probes;
  }

  fn hash(&self, key: &K) -> usize {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish() as usize % self.get_size()


    // let mut hash = 2166136261;
    // for byte in key.to_string().as_bytes() {
    //   hash ^= *byte as usize;
    //   hash = hash.wrapping_mul(16777619);
    // }

    // hash % self.get_size()

  }
}

impl<K: Clone + Hash + Display + PartialEq + ToString, V: Clone + Display> HashTable<K, V> {
  pub fn print(&self) {
    for i in 0..self.get_size() {
      if self.table[i].is_some() {
        let (key, value) = self.table[i].clone().unwrap();
        println!("Index: {}, Key: {}, Value: {}", i, key, value);
      }
    }
  }
}