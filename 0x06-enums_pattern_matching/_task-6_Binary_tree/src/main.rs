// 0x06. Rust - Enums and Pattern Matching

mod binary_tree;
use binary_tree::BinaryTree;

fn main() {
    let mut tree = BinaryTree::Empty;
    
    for value in [5, 3, 7, 2, 4, 6, 8] {
        tree.insert(value);
    }
    
    println!("Tree contains 4: {}", tree.contains(4));
    println!("Tree contains 9: {}", tree.contains(9));
    
    println!("In-order traversal:");
    tree.print_in_order();
}