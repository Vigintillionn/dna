// CONTEXT FOR ANYONE READING THIS CODE:
// I tried to implement a Red-Black Tree in Rust
// I did not succeed
// I decided to use someone else's implementation
// Below is my attempt at implementing a Red-Black Tree in Rust
// If you ever want a good laugh at my expense, read this code


use std::cmp::Ordering;

#[derive(Copy, Clone)]
pub enum RBColor {
  Red,
  Black,
}

impl PartialEq for RBColor {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (RBColor::Red, RBColor::Red) => true,
      (RBColor::Black, RBColor::Black) => true,
      _ => false,
    }
  }
}

// Red-Black Node
pub struct RBNode<T> {
  color: RBColor,
  left: Option<Box<RBNode<T>>>,
  right: Option<Box<RBNode<T>>>,
  parent: *mut RBNode<T>,
  value: T,
}

impl<T: Ord + Copy + Clone> PartialOrd for RBNode<T> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl<T: Ord + Copy + Clone> Ord for RBNode<T> {
  fn cmp(&self, other: &Self) -> Ordering {
    self.value.cmp(&other.value)
  }
}

impl<T: Ord + Copy + Clone> PartialEq for RBNode<T> {
  fn eq(&self, other: &Self) -> bool {
    self.value == other.value
  }
}

impl<T: Ord + Copy + Clone> Eq for RBNode<T> {}

impl<T: Ord + Copy + Clone> RBNode<T> {
  fn new(value: T) -> Self {
    RBNode {
      color: RBColor::Red,
      left: None,
      right: None,
      parent: std::ptr::null_mut(),
      value,
    }
  }

  fn set_color(&mut self, color: RBColor) {
    self.color = color;
  }

  fn is_red(&self) -> bool {
    self.color == RBColor::Red
  }

  fn set_parent(&mut self, parent: *mut RBNode<T>) {
    self.parent = parent;
  }

  fn set_left(&mut self, left: Option<Box<RBNode<T>>>) {
    self.left = left;
  }

  fn set_right(&mut self, right: Option<Box<RBNode<T>>>) {
    self.right = right;
  }

  fn is_left_child(&self) -> bool {
    if self.parent.is_null() {
      return false;
    }

    self.get_parent().unwrap().left.as_ref().unwrap().value == self.value
  }

  fn is_right_child(&self) -> bool {
    if self.parent.is_null() {
      return false;
    }

    self.get_parent().unwrap().right.as_ref().unwrap().value == self.value
  }

  fn get_parent(&self) -> Option<&RBNode<T>> {
    if self.parent.is_null() {
      return None;
    }
    Some(unsafe { &*self.parent })
  }

  fn get_left(&self) -> Option<&RBNode<T>> {
    self.left.as_ref().map(|node| &**node)
  }

  fn get_right(&self) -> Option<&RBNode<T>> {
    self.right.as_ref().map(|node| &**node)
  }

  fn rotate_left(&mut self) {
    if let Some(mut right) = self.right.take() {
      if let Some(mut right_left) = right.left.take() {
        // Set left child of right node to self
        right_left.set_parent(self);
        // Set right child of self to left child of right node
        self.right = Some(right_left);
      }
      // Set parent of right node to parent of self
      right.set_parent(self.parent);
      // Set parent of self to right node
      self.set_parent(&mut *right);
      // Set left child of right node to self
      right.set_left(Some(Box::new(RBNode {
        // Set color of right node to red
        color: RBColor::Red,
        left: self.left.take(),
        right: self.right.take(),
        parent: self.parent,
        value: self.value,
      })));
    }
  }

  fn rotate_right(&mut self) {
    if let Some(mut left) = self.left.take() {
      if let Some(mut left_right) = left.right.take() {
        // Set right child of left node to self
        left_right.set_parent(self);
        // Set left child of self to right child of left node
        self.left = Some(left_right);
      }
      // Set parent of left node to parent of self
      left.set_parent(self.parent);
      // Set parent of self to left node
      self.set_parent(&mut *left);
      // Set right child of left node to self
      left.set_right(Some(Box::new(RBNode {
        // Set color of left node to red
        color: RBColor::Red,
        left: self.left.take(),
        right: self.right.take(),
        parent: self.parent,
        value: self.value,
      })));
    }
  }

  fn color_swap(&mut self) {
    if let Some(left) = self.get_left() {
      if let Some(right) = self.get_right() {
        if left.is_red() && right.is_red() {
          left.set_color(RBColor::Black);
          right.set_color(RBColor::Black);
          self.set_color(RBColor::Red);
        }
      }
    }
  }
}

// Red-Black Tree
pub struct RBTree<T> {
  root: Option<Box<RBNode<T>>>,
}

impl<T: Ord + Copy + Clone> RBTree<T> {
  pub fn new() -> Self {
    RBTree { root: None }
  }

  pub fn insert(&mut self, value: T) {
    let mut new_node = RBNode::new(value);
    let mut parent: *mut RBNode<T> = std::ptr::null_mut();
    let mut current = self.root.as_ref();

    while !current.is_none() {
      parent = current.unwrap().as_ref() as *const RBNode<T> as *mut RBNode<T>;
      match new_node.cmp(&&mut current.unwrap()) {
        Ordering::Less => {
          current = current.unwrap().left.as_ref();
        },
        _ => {
          current = current.unwrap().right.as_ref();
        },
      }
    }

    new_node.set_parent(parent);
    if parent.is_null() {
      new_node.set_color(RBColor::Black);
      self.root = Some(Box::new(new_node));
    } else {
      if new_node.cmp(&&mut current.unwrap()) == Ordering::Less {
        unsafe {
          (*parent).set_left(Some(Box::new(new_node)));
        }
      } else {
        unsafe {
          (*parent).set_right(Some(Box::new(new_node)));
        }
      }
    }
  }

  fn fix_inser(&self, node: RBNode<T>) {
    while node.get_parent().unwrap().is_red() {
      // Case 1: Parent is left child
      
    }
  }

  pub fn print(&self) {
  }
}

