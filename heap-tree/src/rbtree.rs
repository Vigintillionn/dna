use std::{
  borrow::Borrow,
  cmp::{Ord, Ordering},
  marker, mem,
  ops::{Bound, Deref, DerefMut, RangeBounds},
};

use rand::Rng;

/// Depth calculates minimum, maximum, average and percentile of leaf-node
/// depths in the [`Llrb`] tree.
#[derive(Clone)]
pub struct Depth {
    samples: usize,
    min: usize,
    max: usize,
    total: usize,
    depths: [u64; 256],
}

impl Depth {
    pub(crate) fn new() -> Depth {
        Default::default()
    }

    pub(crate) fn sample(&mut self, depth: usize) {
        self.samples += 1;
        self.total += depth;
        if self.min == 0 || depth < self.min {
            self.min = depth
        }
        if self.max == 0 || depth > self.max {
            self.max = depth
        }
        self.depths[depth] += 1;
    }

    /// Return number of leaf-nodes sampled in [`Llrb`] instance.
    pub fn samples(&self) -> usize {
        self.samples
    }

    /// Return minimum depth of leaf-node in [`Llrb`] instance.
    pub fn min(&self) -> usize {
        self.min
    }

    /// Return maximum depth of leaf-node in [`Llrb`] instance.
    pub fn max(&self) -> usize {
        self.max
    }

    /// Return the average depth of leaf-nodes in [`Llrb`] instance.
    pub fn mean(&self) -> usize {
        self.total / self.samples
    }

    /// Return depth as tuple of percentiles, each tuple provides
    /// (percentile, depth). Returned percentiles from 90, 91 .. 99
    pub fn percentiles(&self) -> Vec<(u8, usize)> {
        let mut percentiles: Vec<(u8, usize)> = vec![];
        let (mut acc, mut prev_perc) = (0_u64, 90_u8);
        let iter = self.depths.iter().enumerate().filter(|(_, &item)| item > 0);
        for (depth, samples) in iter {
            acc += *samples;
            let perc = ((acc as f64 / self.samples as f64) * 100_f64) as u8;
            if perc >= prev_perc {
                percentiles.push((perc, depth));
                prev_perc = perc;
            }
        }
        percentiles
    }

    /// Pretty print depth statistics in human readable format, useful in logs.
    pub fn pretty_print(&self, prefix: &str) {
        let mean = self.mean();
        println!(
            "{}depth (min, max, avg): {:?}",
            prefix,
            (self.min, mean, self.max)
        );
        for (depth, n) in self.percentiles().into_iter() {
            if n > 0 {
                println!("{}  {} percentile = {}", prefix, depth, n);
            }
        }
    }

    /// Convert depth statistics to JSON format, useful for plotting.
    pub fn json(&self) -> String {
        let ps: Vec<String> = self
            .percentiles()
            .into_iter()
            .map(|(d, n)| format!("{}: {}", d, n))
            .collect();
        let strs = [
            format!("min: {}", self.min),
            format!("mean: {}", self.mean()),
            format!("max: {}", self.max),
            format!("percentiles: {}", ps.join(", ")),
        ];
        ("{ ".to_string() + strs.join(", ").as_str() + " }").to_string()
    }
}

impl Default for Depth {
    fn default() -> Self {
        Depth {
            samples: 0,
            min: 0,
            max: 0,
            total: 0,
            depths: [0; 256],
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum Error<K>
where
    K: Clone + Ord,
{
    /// Fatal case, breaking one of the two LLRB rules.
    ConsecutiveReds,
    /// Fatal case, breaking one of the two LLRB rules. The String
    /// component of this variant can be used for debugging.
    UnbalancedBlacks(String),
    /// Fatal case, index entries are not in sort-order.
    SortError(K, K),
    /// Returned by create() API when key is already present.
    OverwriteKey,
}

// TODO: replace id() with to_name().

/// Llrb manage a single instance of in-memory index using
/// [left-leaning-red-black][llrb] tree.
///
/// [llrb]: https://en.wikipedia.org/wiki/Left-leaning_red-black_tree
#[derive(Clone)]
pub struct Llrb<K, V>
where
  K: Clone + Ord,
  V: Clone,
{
  name: String,
  root: Option<Box<Node<K, V>>>,
  n_count: usize, // number of entries in the tree.
}

impl<K, V> Extend<(K, V)> for Llrb<K, V>
where
  K: Clone + Ord,
  V: Clone,
{
  fn extend<I>(&mut self, iter: I)
  where
      I: IntoIterator<Item = (K, V)>,
  {
      iter.into_iter().for_each(|(key, value)| {
          self.set(key, value);
      });
  }
}

/// Different ways to construct a new Llrb instance.
impl<K, V> Llrb<K, V>
where
  K: Clone + Ord,
  V: Clone,
{
  /// Create an empty instance of Llrb, identified by `name`.
  /// Applications can choose unique names.
  pub fn new<S>(name: S) -> Llrb<K, V>
  where
      S: AsRef<str>,
  {
      Llrb {
          name: name.as_ref().to_string(),
          root: Default::default(),
          n_count: Default::default(),
      }
  }
}

/// Maintenance API.
impl<K, V> Llrb<K, V>
where
  K: Clone + Ord,
  V: Clone,
{
  /// Identify this instance. Applications can choose unique names while
  /// creating Llrb instances.
  #[inline]
  pub fn id(&self) -> String {
      self.name.clone()
  }

  /// Return number of entries in this instance.
  #[inline]
  pub fn len(&self) -> usize {
      self.n_count
  }

  /// Check whether this index is empty.
  #[inline]
  pub fn is_empty(&self) -> bool {
      self.n_count == 0
  }

  /// Return quickly with basic statisics, only entries() method is valid
  /// with this statisics.
  pub fn stats(&self) -> Stats {
      Stats::new(self.n_count, mem::size_of::<Node<K, V>>())
  }
}

type Insert<K, V> = (Box<Node<K, V>>, Option<Error<K>>);

type Upsert<K, V> = (Box<Node<K, V>>, Option<V>);

type Delete<K, V> = (Option<Box<Node<K, V>>>, Option<V>);

type Delmin<K, V> = (Option<Box<Node<K, V>>>, Option<Node<K, V>>);

/// Write operations on Llrb instance.
impl<K, V> Llrb<K, V>
where
  K: Clone + Ord,
  V: Clone,
{
  /// Create a new {key, value} entry in the index. If key is already
  /// present return error.
  pub fn create(&mut self, key: K, value: V) -> Result<(), Error<K>> {
      let (mut root, error) = Llrb::insert(self.root.take(), key, value);
      root.set_black();
      self.root = Some(root);
      match error {
          Some(err) => Err(err),
          None => {
              self.n_count += 1;
              Ok(())
          }
      }
  }

  /// Set value for key. If there is an existing entry for key,
  /// overwrite the old value with new value and return the old value.
  pub fn set(&mut self, key: K, value: V) -> Option<V> {
      let (mut root, old_value) = Llrb::upsert(self.root.take(), key, value);
      root.set_black();
      self.root = Some(root);
      match old_value {
          old_value @ Some(_) => old_value,
          None => {
              self.n_count += 1;
              None
          }
      }
  }

  /// Delete key from this instance and return its value. If key is
  /// not present, then delete is effectively a no-op.
  pub fn delete<Q>(&mut self, key: &Q) -> Option<V>
  where
      K: Borrow<Q>,
      Q: Ord + ?Sized,
  {
      let (root, old_value) = match Llrb::do_delete(self.root.take(), key) {
          (None, old_value) => (None, old_value),
          (Some(mut root), old_value) => {
              root.set_black();
              (Some(root), old_value)
          }
      };
      self.root = root;
      if old_value.is_some() {
          self.n_count -= 1;
      }
      old_value
  }

  /// Validate LLRB tree with following rules:
  ///
  /// * From root to any leaf, no consecutive reds allowed in its path.
  /// * Number of blacks should be same under left child and right child.
  /// * Make sure keys are in sorted order.
  ///
  /// Additionally return full statistics on the tree. Refer to [`Stats`]
  /// for more information.
  pub fn validate(&self) -> Result<Stats, Error<K>> {
      let root = self.root.as_ref().map(Deref::deref);
      let (red, nb, d) = (is_red(root), 0, 0);
      let mut stats = Stats::new(self.n_count, mem::size_of::<Node<K, V>>());
      stats.set_depths(Depth::new());
      let blacks = Llrb::validate_tree(root, red, nb, d, &mut stats)?;
      stats.set_blacks(blacks);
      Ok(stats)
  }

  pub fn black_depth(&self) -> usize {
    // Find the depth of the tree
    // The depth of a tree is the number of edges with a black color on the longest path between the root and a leaf.

    let mut depth = 0;
    let mut node = self.root.as_ref().map(Deref::deref);

    while !node.is_none() {
      if is_black(node) {
        depth += 1
      }

      node = node.unwrap().right.as_deref()
    }

    depth
  }

  pub fn n_blacks(&self) -> usize {
    self.root.as_ref().unwrap().n_blacks()
  }
}

/// Read operations on Llrb instance.
impl<K, V> Llrb<K, V>
where
  K: Clone + Ord,
  V: Clone,
{
  /// Get the value for key.
  pub fn get<Q>(&self, key: &Q) -> Option<V>
  where
      K: Borrow<Q>,
      Q: Ord + ?Sized,
  {
      let mut node = self.root.as_ref().map(Deref::deref);
      while let Some(nref) = node {
          node = match nref.key.borrow().cmp(key) {
              Ordering::Less => nref.right_deref(),
              Ordering::Greater => nref.left_deref(),
              Ordering::Equal => return Some(nref.value.clone()),
          };
      }
      None
  }

  /// Return a random entry from this index.
  pub fn random<R: Rng>(&self, rng: &mut R) -> Option<(K, V)> {
      let mut nref = self.root.as_ref().map(Deref::deref)?;

      let mut at_depth = rng.gen::<u8>() % 40;
      loop {
          let next = match rng.gen::<u8>() % 2 {
              0 => nref.left_deref(),
              1 => nref.right_deref(),
              _ => unreachable!(),
          };
          if at_depth == 0 || next.is_none() {
              break Some((nref.key.clone(), nref.value.clone()));
          }
          at_depth -= 1;
          nref = next.unwrap();
      }
  }

  /// Return an iterator over all entries in this instance.
  pub fn iter(&self) -> Iter<K, V> {
      let node = self.root.as_ref().map(Deref::deref);
      Iter {
          paths: Some(build_iter(IFlag::Left, node, vec![])),
      }
  }

  /// Range over all entries from low to high.
  pub fn range<Q, R>(&self, range: R) -> Range<K, V, R, Q>
  where
      K: Borrow<Q>,
      R: RangeBounds<Q>,
      Q: Ord + ?Sized,
  {
      let root = self.root.as_ref().map(Deref::deref);
      let paths = match range.start_bound() {
          Bound::Unbounded => Some(build_iter(IFlag::Left, root, vec![])),
          Bound::Included(low) => Some(find_start(root, low, true, vec![])),
          Bound::Excluded(low) => Some(find_start(root, low, false, vec![])),
      };
      let high = marker::PhantomData;
      Range { range, paths, high }
  }

  /// Reverse range over all entries from high to low.
  pub fn reverse<R, Q>(&self, range: R) -> Reverse<K, V, R, Q>
  where
      K: Borrow<Q>,
      R: RangeBounds<Q>,
      Q: Ord + ?Sized,
  {
      let root = self.root.as_ref().map(Deref::deref);
      let paths = match range.end_bound() {
          Bound::Unbounded => Some(build_iter(IFlag::Right, root, vec![])),
          Bound::Included(high) => Some(find_end(root, high, true, vec![])),
          Bound::Excluded(high) => Some(find_end(root, high, false, vec![])),
      };
      let low = marker::PhantomData;
      Reverse { range, paths, low }
  }
}

impl<K, V> Llrb<K, V>
where
  K: Clone + Ord,
  V: Clone,
{
  fn insert(node: Option<Box<Node<K, V>>>, key: K, value: V) -> Insert<K, V> {
      if node.is_none() {
          return (Node::new(key, value, false /*black*/), None);
      }

      let mut node = Llrb::walkdown_rot23(node.unwrap());

      match node.key.cmp(&key) {
          Ordering::Greater => {
              let (left, e) = Llrb::insert(node.left.take(), key, value);
              node.left = Some(left);
              (Llrb::walkuprot_23(node), e)
          }
          Ordering::Less => {
              let (right, e) = Llrb::insert(node.right.take(), key, value);
              node.right = Some(right);
              (Llrb::walkuprot_23(node), e)
          }
          Ordering::Equal => {
              let err = Some(Error::OverwriteKey);
              (Llrb::walkuprot_23(node), err)
          }
      }
  }

  fn upsert(node: Option<Box<Node<K, V>>>, key: K, value: V) -> Upsert<K, V> {
      if node.is_none() {
          return (Node::new(key, value, false /*black*/), None);
      }

      let mut node = Llrb::walkdown_rot23(node.unwrap());

      match node.key.cmp(&key) {
          Ordering::Greater => {
              let (left, o) = Llrb::upsert(node.left.take(), key, value);
              node.left = Some(left);
              (Llrb::walkuprot_23(node), o)
          }
          Ordering::Less => {
              let (right, o) = Llrb::upsert(node.right.take(), key, value);
              node.right = Some(right);
              (Llrb::walkuprot_23(node), o)
          }
          Ordering::Equal => {
              let old_value = node.value.clone();
              node.set_value(value);
              (Llrb::walkuprot_23(node), Some(old_value))
          }
      }
  }

  fn do_delete<Q>(node: Option<Box<Node<K, V>>>, key: &Q) -> Delete<K, V>
  where
      K: Borrow<Q>,
      Q: Ord + ?Sized,
  {
      let mut node = match node {
          None => return (None, None),
          Some(node) => node,
      };

      if node.key.borrow().gt(key) {
          if node.left.is_none() {
              (Some(node), None)
          } else {
              let ok = !is_red(node.left_deref());
              if ok && !is_red(node.left.as_ref().unwrap().left_deref()) {
                  node = Llrb::move_red_left(node);
              }
              let (left, old_value) = Llrb::do_delete(node.left.take(), key);
              node.left = left;
              (Some(Llrb::fixup(node)), old_value)
          }
      } else {
          if is_red(node.left_deref()) {
              node = Llrb::rotate_right(node);
          }

          if !node.key.borrow().lt(key) && node.right.is_none() {
              return (None, Some(node.value.clone()));
          }

          let ok = node.right.is_some() && !is_red(node.right_deref());
          if ok && !is_red(node.right.as_ref().unwrap().left_deref()) {
              node = Llrb::move_red_right(node);
          }

          if !node.key.borrow().lt(key) {
              // node == key
              let (right, mut res_node) = Llrb::delete_min(node.right.take());
              node.right = right;
              if res_node.is_none() {
                  panic!("do_delete(): fatal logic, call the programmer");
              }
              let subdel = res_node.take().unwrap();
              let mut newnode = Box::new(subdel.clone_detach());
              newnode.left = node.left.take();
              newnode.right = node.right.take();
              newnode.black = node.black;
              (Some(Llrb::fixup(newnode)), Some(node.value.clone()))
          } else {
              let (right, old_value) = Llrb::do_delete(node.right.take(), key);
              node.right = right;
              (Some(Llrb::fixup(node)), old_value)
          }
      }
  }

  fn delete_min(node: Option<Box<Node<K, V>>>) -> Delmin<K, V> {
      if node.is_none() {
          return (None, None);
      }
      let mut node = node.unwrap();
      if node.left.is_none() {
          return (None, Some(*node));
      }
      let left = node.left_deref();
      if !is_red(left) && !is_red(left.unwrap().left_deref()) {
          node = Llrb::move_red_left(node);
      }
      let (left, old_node) = Llrb::delete_min(node.left.take());
      node.left = left;
      (Some(Llrb::fixup(node)), old_node)
  }

  fn validate_tree(
      node: Option<&Node<K, V>>,
      fromred: bool,
      mut nb: usize,
      depth: usize,
      stats: &mut Stats,
  ) -> Result<usize, Error<K>> {
      if node.is_none() {
          stats.depths.as_mut().unwrap().sample(depth);
          return Ok(nb);
      }

      let red = is_red(node.as_ref().map(Deref::deref));
      if fromred && red {
          return Err(Error::ConsecutiveReds);
      }
      if !red {
          nb += 1;
      }
      let node = &node.as_ref().unwrap();
      let (left, right) = (node.left_deref(), node.right_deref());
      let lblacks = Llrb::validate_tree(left, red, nb, depth + 1, stats)?;
      let rblacks = Llrb::validate_tree(right, red, nb, depth + 1, stats)?;
      if lblacks != rblacks {
          let err = format!("left: {} right: {}", lblacks, rblacks);
          return Err(Error::UnbalancedBlacks(err));
      }
      if node.left.is_some() {
          let left = node.left.as_ref().unwrap();
          if left.key.ge(&node.key) {
              let (lkey, parent) = (left.key.clone(), node.key.clone());
              return Err(Error::SortError(lkey, parent));
          }
      }
      if node.right.is_some() {
          let right = node.right.as_ref().unwrap();
          if right.key.le(&node.key) {
              let (rkey, parent) = (right.key.clone(), node.key.clone());
              return Err(Error::SortError(rkey, parent));
          }
      }
      Ok(lblacks)
  }

  //--------- rotation routines for 2-3 algorithm ----------------

  fn walkdown_rot23(node: Box<Node<K, V>>) -> Box<Node<K, V>> {
      node
  }

  fn walkuprot_23(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
      if is_red(node.right_deref()) && !is_red(node.left_deref()) {
          node = Llrb::rotate_left(node);
      }
      let left = node.left_deref();
      if is_red(left) && is_red(left.unwrap().left_deref()) {
          node = Llrb::rotate_right(node);
      }
      if is_red(node.left_deref()) && is_red(node.right_deref()) {
          Llrb::flip(node.deref_mut())
      }
      node
  }

  //              (i)                       (i)
  //               |                         |
  //              node                       x
  //              /  \                      / \
  //             /    (r)                 (r)  \
  //            /       \                 /     \
  //          left       x             node      xr
  //                    / \            /  \
  //                  xl   xr       left   xl
  //
  fn rotate_left(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
      if is_black(node.right_deref()) {
          panic!("rotateleft(): rotating a black link ? Call the programmer");
      }
      let mut x = node.right.take().unwrap();
      node.right = x.left.take();
      x.black = node.black;
      node.set_red();
      x.left = Some(node);
      x
  }

  //              (i)                       (i)
  //               |                         |
  //              node                       x
  //              /  \                      / \
  //            (r)   \                   (r)  \
  //           /       \                 /      \
  //          x       right             xl      node
  //         / \                                / \
  //       xl   xr                             xr  right
  //
  fn rotate_right(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
      if is_black(node.left_deref()) {
          panic!("rotateright(): rotating a black link ? Call the programmer")
      }
      let mut x = node.left.take().unwrap();
      node.left = x.right.take();
      x.black = node.black;
      node.set_red();
      x.right = Some(node);
      x
  }

  //        (x)                   (!x)
  //         |                     |
  //        node                  node
  //        / \                   / \
  //      (y) (z)              (!y) (!z)
  //     /      \              /      \
  //   left    right         left    right
  //
  fn flip(node: &mut Node<K, V>) {
      node.left.as_mut().unwrap().toggle_link();
      node.right.as_mut().unwrap().toggle_link();
      node.toggle_link();
  }

  fn fixup(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
      node = if is_red(node.right_deref()) {
          Llrb::rotate_left(node)
      } else {
          node
      };
      node = {
          let left = node.left_deref();
          if is_red(left) && is_red(left.unwrap().left_deref()) {
              Llrb::rotate_right(node)
          } else {
              node
          }
      };
      if is_red(node.left_deref()) && is_red(node.right_deref()) {
          Llrb::flip(node.deref_mut());
      }
      node
  }

  fn move_red_left(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
      Llrb::flip(node.deref_mut());
      if is_red(node.right.as_ref().unwrap().left_deref()) {
          node.right = Some(Llrb::rotate_right(node.right.take().unwrap()));
          node = Llrb::rotate_left(node);
          Llrb::flip(node.deref_mut());
      }
      node
  }

  fn move_red_right(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
      Llrb::flip(node.deref_mut());
      if is_red(node.left.as_ref().unwrap().left_deref()) {
          node = Llrb::rotate_right(node);
          Llrb::flip(node.deref_mut());
      }
      node
  }
}

fn is_red<K, V>(node: Option<&Node<K, V>>) -> bool
where
  K: Clone + Ord,
  V: Clone,
{
  node.map_or(false, |node| !node.is_black())
}

fn is_black<K, V>(node: Option<&Node<K, V>>) -> bool
where
  K: Clone + Ord,
  V: Clone,
{
  node.map_or(true, |node| node.is_black())
}

pub struct Iter<'a, K, V>
where
  K: Clone + Ord,
  V: Clone,
{
  paths: Option<Vec<Fragment<'a, K, V>>>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V>
where
  K: Clone + Ord,
  V: Clone,
{
  type Item = (K, V);

  fn next(&mut self) -> Option<Self::Item> {
      let mut paths = match self.paths.take() {
          Some(paths) => paths,
          None => return None,
      };
      match paths.pop() {
          None => None,
          Some(mut path) => match (path.flag, path.nref) {
              (IFlag::Left, nref) => {
                  path.flag = IFlag::Center;
                  paths.push(path);
                  self.paths = Some(paths);
                  Some((nref.key.clone(), nref.value.clone()))
              }
              (IFlag::Center, nref) => {
                  path.flag = IFlag::Right;
                  paths.push(path);
                  let rnref = nref.right_deref();
                  self.paths = Some(build_iter(IFlag::Left, rnref, paths));
                  self.next()
              }
              (_, _) => {
                  self.paths = Some(paths);
                  self.next()
              }
          },
      }
  }
}

pub struct Range<'a, K, V, R, Q>
where
  K: Clone + Ord + Borrow<Q>,
  V: Clone,
  R: RangeBounds<Q>,
  Q: Ord + ?Sized,
{
  range: R,
  paths: Option<Vec<Fragment<'a, K, V>>>,
  high: marker::PhantomData<Q>,
}

impl<'a, K, V, R, Q> Iterator for Range<'a, K, V, R, Q>
where
  K: Clone + Ord + Borrow<Q>,
  V: Clone,
  R: RangeBounds<Q>,
  Q: Ord + ?Sized,
{
  type Item = (K, V);

  fn next(&mut self) -> Option<Self::Item> {
      let mut paths = match self.paths.take() {
          Some(paths) => paths,
          None => return None,
      };

      let item = match paths.pop() {
          None => None,
          Some(mut path) => match (path.flag, path.nref) {
              (IFlag::Left, nref) => {
                  path.flag = IFlag::Center;
                  paths.push(path);
                  self.paths = Some(paths);
                  Some((nref.key.clone(), nref.value.clone()))
              }
              (IFlag::Center, nref) => {
                  path.flag = IFlag::Right;
                  paths.push(path);
                  let rnref = nref.right_deref();
                  self.paths = Some(build_iter(IFlag::Left, rnref, paths));
                  self.next()
              }
              (_, _) => {
                  self.paths = Some(paths);
                  self.next()
              }
          },
      };
      match item {
          None => None,
          Some((k, v)) => match self.range.end_bound() {
              Bound::Included(high) if k.borrow().le(high) => Some((k, v)),
              Bound::Excluded(high) if k.borrow().lt(high) => Some((k, v)),
              Bound::Unbounded => Some((k, v)),
              Bound::Included(_) | Bound::Excluded(_) => {
                  self.paths.take();
                  None
              }
          },
      }
  }
}

pub struct Reverse<'a, K, V, R, Q>
where
  K: Clone + Ord + Borrow<Q>,
  V: Clone,
  R: RangeBounds<Q>,
  Q: Ord + ?Sized,
{
  range: R,
  paths: Option<Vec<Fragment<'a, K, V>>>,
  low: marker::PhantomData<Q>,
}

impl<'a, K, V, R, Q> Iterator for Reverse<'a, K, V, R, Q>
where
  K: Clone + Ord + Borrow<Q>,
  V: Clone,
  R: RangeBounds<Q>,
  Q: Ord + ?Sized,
{
  type Item = (K, V);

  fn next(&mut self) -> Option<Self::Item> {
      let mut paths = match self.paths.take() {
          Some(paths) => paths,
          None => return None,
      };

      let item = match paths.pop() {
          None => None,
          Some(mut path) => match (path.flag, path.nref) {
              (IFlag::Right, nref) => {
                  path.flag = IFlag::Center;
                  paths.push(path);
                  self.paths = Some(paths);
                  Some((nref.key.clone(), nref.value.clone()))
              }
              (IFlag::Center, nref) => {
                  path.flag = IFlag::Left;
                  paths.push(path);
                  let rnref = nref.left_deref();
                  self.paths = Some(build_iter(IFlag::Right, rnref, paths));
                  self.next()
              }
              (_, _) => {
                  self.paths = Some(paths);
                  self.next()
              }
          },
      };
      match item {
          None => None,
          Some((k, v)) => match self.range.start_bound() {
              Bound::Included(low) if k.borrow().ge(low) => Some((k, v)),
              Bound::Excluded(low) if k.borrow().gt(low) => Some((k, v)),
              Bound::Unbounded => Some((k, v)),
              Bound::Included(_) | Bound::Excluded(_) => {
                  self.paths.take();
                  None
              }
          },
      }
  }
}

/// Node corresponds to a single entry in Llrb instance.
#[derive(Clone)]
pub struct Node<K, V>
where
  K: Clone + Ord,
  V: Clone,
{
  key: K,
  value: V,
  black: bool,                    // store: black or red
  left: Option<Box<Node<K, V>>>,  // store: left child
  right: Option<Box<Node<K, V>>>, // store: right child
}

// Primary operations on a single node.
impl<K, V> Node<K, V>
where
  K: Clone + Ord,
  V: Clone,
{
  // CREATE operation
  fn new(key: K, value: V, black: bool) -> Box<Node<K, V>> {
      Box::new(Node {
          key,
          value,
          black,
          left: None,
          right: None,
      })
  }

  fn n_blacks(&self) -> usize {
    let black_children = self.left.as_ref().map_or(0, |l| l.n_blacks())
        + self.right.as_ref().map_or(0, |r| r.n_blacks());
    if self.is_black() {
        black_children + 1
    } else {
        black_children
    }
}

  // clone and detach this node from the tree.
  fn clone_detach(&self) -> Node<K, V> {
      Node {
          key: self.key.clone(),
          value: self.value.clone(),
          black: self.black,
          left: None,
          right: None,
      }
  }

  #[inline]
  fn left_deref(&self) -> Option<&Node<K, V>> {
      self.left.as_ref().map(Deref::deref)
  }

  #[inline]
  fn right_deref(&self) -> Option<&Node<K, V>> {
      self.right.as_ref().map(Deref::deref)
  }

  // prepend operation, equivalent to SET / INSERT / UPDATE
  #[inline]
  fn set_value(&mut self, value: V) {
      self.value = value
  }

  #[inline]
  fn set_red(&mut self) {
      self.black = false
  }

  #[inline]
  fn set_black(&mut self) {
      self.black = true
  }

  #[inline]
  fn toggle_link(&mut self) {
      self.black = !self.black
  }

  #[inline]
  fn is_black(&self) -> bool {
      self.black
  }
}

// TODO: implement Display for Stats.

/// Statistics on [`Llrb`] tree. Serves two purpose:
///
/// * To get partial but quick statistics via [`Llrb::stats`] method.
/// * To get full statisics via [`Llrb::validate`] method.
#[derive(Default)]
pub struct Stats {
  entries: usize, // number of entries in the tree.
  node_size: usize,
  blacks: Option<usize>,
  depths: Option<Depth>,
}

impl Stats {
  fn new(entries: usize, node_size: usize) -> Stats {
      Stats {
          entries,
          node_size,
          blacks: Default::default(),
          depths: Default::default(),
      }
  }

  #[inline]
  fn set_blacks(&mut self, blacks: usize) {
      self.blacks = Some(blacks)
  }

  #[inline]
  fn set_depths(&mut self, depths: Depth) {
      self.depths = Some(depths)
  }

  /// Return number entries in [`Llrb`] instance.
  #[inline]
  pub fn entries(&self) -> usize {
      self.entries
  }

  /// Return node-size, including over-head for `Llrb<k,V>`. Although
  /// the node overhead is constant, the node size varies based on
  /// key and value types. EG:
  ///
  /// ```
  /// use llrb_index::Llrb;
  /// let mut llrb: Llrb<u64,i128> = Llrb::new("myinstance");
  ///
  /// // size of key: 8 bytes
  /// // size of value: 16 bytes
  /// // overhead is 24 bytes
  /// assert_eq!(llrb.stats().node_size(), 48);
  /// ```
  #[inline]
  pub fn node_size(&self) -> usize {
      self.node_size
  }

  /// Return number of black nodes from root to leaf, on both left
  /// and right child.
  #[inline]
  pub fn blacks(&self) -> Option<usize> {
      self.blacks
  }

  /// Return [`Depth`] statistics.
  pub fn depths(&self) -> Option<Depth> {
      if self.depths.as_ref().unwrap().samples() == 0 {
          None
      } else {
          self.depths.clone()
      }
  }
}

#[derive(Copy, Clone)]
enum IFlag {
  Left,
  Center,
  Right,
}

struct Fragment<'a, K, V>
where
  K: Clone + Ord,
  V: Clone,
{
  flag: IFlag,
  nref: &'a Node<K, V>,
}

fn build_iter<'a, K, V>(
  flag: IFlag,
  nref: Option<&'a Node<K, V>>, // subtree
  mut paths: Vec<Fragment<'a, K, V>>,
) -> Vec<Fragment<'a, K, V>>
where
  K: Clone + Ord,
  V: Clone,
{
  match nref {
      None => paths,
      Some(nref) => {
          let item = Fragment { flag, nref };
          let nref = match flag {
              IFlag::Left => nref.left_deref(),
              IFlag::Right => nref.right_deref(),
              IFlag::Center => unreachable!(),
          };
          paths.push(item);
          build_iter(flag, nref, paths)
      }
  }
}

fn find_start<'a, K, V, Q>(
  nref: Option<&'a Node<K, V>>,
  low: &Q,
  incl: bool,
  mut paths: Vec<Fragment<'a, K, V>>,
) -> Vec<Fragment<'a, K, V>>
where
  K: Clone + Ord + Borrow<Q>,
  V: Clone,
  Q: Ord + ?Sized,
{
  match nref {
      None => paths,
      Some(nref) => {
          let cmp = nref.key.borrow().cmp(low);
          let flag = match cmp {
              Ordering::Less => IFlag::Right,
              Ordering::Equal if incl => IFlag::Left,
              Ordering::Equal => IFlag::Center,
              Ordering::Greater => IFlag::Left,
          };
          paths.push(Fragment { flag, nref });
          match cmp {
              Ordering::Less => {
                  let nref = nref.right_deref();
                  find_start(nref, low, incl, paths)
              }
              Ordering::Equal => paths,
              Ordering::Greater => {
                  let nref = nref.left_deref();
                  find_start(nref, low, incl, paths)
              }
          }
      }
  }
}

fn find_end<'a, K, V, Q>(
  nref: Option<&'a Node<K, V>>,
  high: &Q,
  incl: bool,
  mut paths: Vec<Fragment<'a, K, V>>,
) -> Vec<Fragment<'a, K, V>>
where
  K: Clone + Ord + Borrow<Q>,
  V: Clone,
  Q: Ord + ?Sized,
{
  match nref {
      None => paths,
      Some(nref) => {
          let cmp = nref.key.borrow().cmp(high);
          let flag = match cmp {
              Ordering::Less => IFlag::Right,
              Ordering::Equal if incl => IFlag::Right,
              Ordering::Equal => IFlag::Center,
              Ordering::Greater => IFlag::Left,
          };
          paths.push(Fragment { flag, nref });
          match cmp {
              Ordering::Less => {
                  let nref = nref.right_deref();
                  find_end(nref, high, incl, paths)
              }
              Ordering::Equal => paths,
              Ordering::Greater => {
                  let nref = nref.left_deref();
                  find_end(nref, high, incl, paths)
              }
          }
      }
  }
}