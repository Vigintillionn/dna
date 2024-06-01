use std::collections::HashMap;

pub fn recursive(n: usize) -> f64 {
  let mut count: usize = 0;
  for i in 1..=n {
    rec_sol(i, &mut count);
  }

  count as f64
}

pub fn rec_sol(n: usize, counts: &mut usize) -> f64 {
  *counts += 1;
  if n == 1 {
    return 0.0;
  }
  if n % 2 == 0 {
    return 1.0 + rec_sol(n / 2, counts);
  }
  return 1.0 + rec_sol(3 * n + 1, counts);
}

pub fn recursive_min(n: usize) -> f64 {
  let mut count: usize = 0;
  for i in 1..=n {
    rec_min_sol(i, &mut count);
  }

  count as f64
}

pub fn rec_min_sol(n: usize, counts: &mut usize) -> f64 {
  *counts += 1;
  if n == 1 {
    return 0.0;
  }
  if n % 2 == 0 {
    return 1.0 + rec_min_sol(n / 2, counts);
  }
  return 1.0 + rec_min_sol((3 * n + 1) / 2, counts);

}

pub fn dynamic(n: usize) -> f64 {
  let mut map = HashMap::<usize, usize>::new();
  map.insert(1, 1);
  let mut count: usize = 0;

  (1..=n).into_iter().for_each(|i| {
    bottom_up(i, &mut map, &mut count);
  });

  count as f64
}

fn bottom_up(n: usize, seen: &mut HashMap<usize, usize>, counts: &mut usize) -> f64 {
  *counts += 1;

  if seen.contains_key(&n) {
    return *seen.get(&n).unwrap() as f64;
  }

  let res = if n % 2 == 0 {
    1 + bottom_up(n / 2, seen, counts) as usize
  } else {
    1 + bottom_up(3 * n + 1, seen, counts) as usize
  };

  seen.insert(n, res);
  res as f64
}