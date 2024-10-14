/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

//

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

// root: Option<Box<TreeNode<T>>>,
#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    fn insert(&mut self, value: T) {
        match &mut self.root {
            Some(node) => node.insert(value),
            None => {
                self.root = Some(Box::new(TreeNode::new(value)));
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        match &self.root {
            Some(node) => node.search(&value),
            None => false,
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
 // Insert a node into the tree
 fn insert(&mut self, value: T) {
    match value.cmp(&self.value) {
        Ordering::Less => {
            // Insert into the left subtree
            if let Some(left_node) = &mut self.left {
                left_node.insert(value);
            } else {
                self.left = Some(Box::new(TreeNode::new(value)));
            }
        }
        Ordering::Greater => {
            // Insert into the right subtree
            if let Some(right_node) = &mut self.right {
                right_node.insert(value);
            } else {
                self.right = Some(Box::new(TreeNode::new(value)));
            }
        }
        Ordering::Equal => {
            // Do nothing on duplicate insertions
        }
    }
}

// Search for a value in the tree
fn search(&self, value: &T) -> bool {
    match value.cmp(&self.value) {
//         2. as_ref()
// as_ref() 方法用于将 Option<Box<TreeNode<T>>> 转换为 Option<&Box<TreeNode<T>>>。这意味着你将从 Option 中获得对其内部值的引用，而不是拥有它。使用 as_ref() 的好处是你可以在不转移所有权的情况下对 Option 中的值进行操作。

// 如果 self.left 是 Some(Box<TreeNode<T>>)，调用 as_ref() 会返回 Some(&Box<TreeNode<T>>)。
// 如果 self.left 是 None，调用 as_ref() 会返回 None。
        Ordering::Less => {
            self.left.as_ref().map_or(false, |node| node.search(value))
            // map or  方法是 Option 类型的一个方法，它接受一个闭包作为参数。如果 Option 是 Some，则 map_or() 会调用闭包并将 Some 中的值传递给它。如果 Option 是 None，则 map_or() 会返回闭包的默认值。
        }
        Ordering::Greater => {
            self.right.as_ref().map_or(false, |node| node.search(value))
        }
        Ordering::Equal => true,
    }
}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


