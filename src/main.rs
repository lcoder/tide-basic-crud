
#[derive(Debug)]
struct TreeNode<T> {
  pub element: T,
  left: BinaryTree<T>,
  right: BinaryTree<T>,
}

#[derive(Debug)]
enum BinaryTree<T> {
  Empty,
  NonEmpty(Box<TreeNode<T>>),
}

impl<T: Copy + Ord> BinaryTree<T> {
  fn add(&mut self, value: T) {
      match *self {
        BinaryTree::Empty => {
          let new = TreeNode { element: value, left: BinaryTree::Empty, right: BinaryTree::Empty };
          let new = Box::new(new);
          *self = BinaryTree::NonEmpty(new);
        }

        BinaryTree::NonEmpty(ref mut node) => {
          let has = value <= node.element;
          if has {
            node.left.add(value);
          } {
            node.right.add(value);
          }
        }
      }
  }
}


#[async_std::main]
async fn main() {
  let mut tree = BinaryTree::Empty;
  tree.add("hi");
  tree.add("maotingfeng");


  if let BinaryTree::NonEmpty(aa) = tree {
    println!("tree={:?}", aa.element);
  }

}
