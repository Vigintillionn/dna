pub use two_three::{Node, TTTree};

pub trait TracksDepth {
    fn depth(&self) -> usize;
}

impl<V: Ord> TracksDepth for TTTree<V> {
    fn depth(&self) -> usize {
        match self.root {
            None => 0,
            Some(ref root) => root.depth(),
        }
    }
}

impl<V: Ord> TracksDepth for Node<V> {
    fn depth(&self) -> usize {
        match self {
            Node::LeafTwo(_) => 1,
            Node::LeafThree(_) => 1,
            Node::Two(two) => 1 + two.1.depth(),
            Node::Three(three) => 1 + three.2.depth(),
        }
    }
}