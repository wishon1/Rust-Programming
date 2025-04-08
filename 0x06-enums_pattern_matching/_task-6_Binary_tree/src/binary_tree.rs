// 0x06. Rust - Enums and Pattern Matching

/// A binary tree that can store integers
#[derive(Debug)]
pub enum BinaryTree {
    /// A node with a value and left/right subtrees
    Node {
        value: i32,
        left: Box<BinaryTree>,
        right: Box<BinaryTree>,
    },
    /// An empty node (leaf)
    Empty,
}

impl BinaryTree {
    /// Inserts a value into the binary tree
    /// # Arguments
    ///     * `value` - The integer value to insert
    pub fn insert(&mut self, value: i32) {
        // Match on self to handle insertion based on current state
        match self {
            // If the tree is empty, create a new node
            BinaryTree::Empty => {
                *self = BinaryTree::Node {
                    value,
                    left: Box::new(BinaryTree::Empty),
                    right: Box::new(BinaryTree::Empty),
                }
            }
            
            // If the tree has a node, compare the value and insert in the appropriate subtree
            BinaryTree::Node { value: node_value, left, right } => {
                if value < *node_value {
                    // Insert into left subtree if value is less than node value
                    left.insert(value);
                } else if value > *node_value {
                    // Insert into right subtree if value is greater than node value
                    right.insert(value);
                }
                // If value equals node value, do nothing (no duplicates)
            }
        }
    }
    
    /// Checks if the tree contains a specific value
    /// # Arguments
    ///     * `value` - The integer value to search for
    /// # Returns
    ///     * `bool` - true if the value is found, false otherwise
    pub fn contains(&self, value: i32) -> bool {
        // Match on self to check if value exists
        match self {
            // Empty tree doesn't contain any values
            BinaryTree::Empty => false,
            
            // If the tree has a node, compare the value
            BinaryTree::Node { value: node_value, left, right } => {
                if value == *node_value {
                    // Found the value
                    true
                } else if value < *node_value {
                    // Check left subtree if value is less than node value
                    left.contains(value)
                } else {
                    // Check right subtree if value is greater than node value
                    right.contains(value)
                }
            }
        }
    }
    
    /// Prints the tree values in order (left, root, right)
    pub fn print_in_order(&self) {
        // Match on self to perform in-order traversal
        match self {
            BinaryTree::Empty => {
                // Nothing to print for empty tree
            }
            BinaryTree::Node { value, left, right } => {
                // Recursively print left subtree
                left.print_in_order();
                // Print current node value
                println!("{}", value);
                // Recursively print right subtree
                right.print_in_order();
            }
        }
    }
}