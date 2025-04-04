use std::fmt;

// Needed for take if done manually, but `Option::take` is idiomatic
use std::mem;

struct Node<T> {
    elem: T,
    next: Link<T>,
}
// Link type alias remains the same
type Link<T> = Option<Box<Node<T>>>;

// Linked List definition remains the same
pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    // Create a new empty list - remains the same
    pub fn new() -> Self { List { head: None } }

    /// Adds an element to the front of the list - simplified steps.
    pub fn push(&mut self, elem: T) {
        // 1. Take ownership of the current head link, replacing it with None.
        //    `Option::take()` is the standard, concise way.
        //    A manual equivalent would be: let previous_head = mem::replace(&mut self.head, None);
        let previous_head: Link<T> = self.head.take();

        // 2. Create the new node struct.
        let node_to_insert = Node {
            elem: elem,
            next: previous_head,
        };

        // 3. Allocate the new node on the heap using Box.
        let new_node: Box<Node<T>> = Box::new(node_to_insert);

        // 4. Set the list's head to the new node.
        self.head = Some(new_node);
    }

    /// Removes the element from the front of the list and returns it
    pub fn pop(&mut self) -> Option<T> {
        // 1. Take ownership of the head link, replacing it with None.
        let head_option: Link<T> = self.head.take();

        // 2. Match on the Option to see if there was a node.
        match head_option {
            Some(popped_node_box) => { // If Some(Box<Node<T>>)
                // popped_node_box now owns the Box<Node<T>>
                // 3. Update self.head to the next node from the popped node.
                self.head = popped_node_box.next;
                // 4. Return the element wrapped in Some.
                Some(popped_node_box.elem)
            }
            None => {
                // List was empty, return None.
                None
            }
        }
    }

    /// Returns an immutable reference to the element at the given index - simplified loops/checks.
    pub fn get(&self, index: usize) -> Option<&T> {
        // Start iteration from the head
        let mut current_node_option = self.head.as_ref(); // Option<&Box<Node<T>>>

        // Loop `index` times to step through the list
        for _ in 0..index {
            // Instead of current = current?.next.as_ref(); use match
            match current_node_option {
                Some(current_node_box) => {
                    // Move to the next node's reference Option
                    current_node_option = current_node_box.next.as_ref();
                }
                None => {
                    // Reached end of list before reaching the desired index
                    return None;
                }
            }
        }

        // After the loop, current_node_option is the Option<&Box<Node<T>>> at the target index.
        // Instead of current.map(|node| &node.elem), use match
        match current_node_option {
            Some(target_node_box) => Some(&target_node_box.elem), // Return Some(&T)
            None => None, // Target index was out of bounds (or list was empty)
        }
    }


    /// Removes the element at the given index and returns it
    /// Removing `?` makes the explicit borrow checking more involved.
     pub fn remove(&mut self, index: usize) -> Option<T> {
        if index == 0 {
            return self.pop();
        }

        // Need a mutable reference to the *Link* (Option) containing the node *before* the target.
        let mut current_link_ref: &mut Link<T> = &mut self.head;

        // Iterate `index` times to get to the link *before* the one to remove.
        // (We need to modify the `next` field of node at index - 1)
        for _ in 0..index { // Loop index times to get the target link
             // Instead of current_link_ref = &mut current_link_ref.as_mut()?.next; use match
             match current_link_ref {
                 // Check if there is a *next* node to move into mutably
                 Some(current_node_box) => {
                      current_link_ref = &mut current_node_box.next;
                 }
                 None => {
                     // Reached end of list before reaching the desired index
                     return None;
                 }
             }
        }

        // current_link_ref is now `&mut Option<Box<Node<T>>>` referring to the link
        // containing the node we want to remove (the one at the target `index`).
        let node_to_remove_option: Option<Box<Node<T>>> = current_link_ref.take(); // Take ownership of the target node Box

        // Match on the result of take()
        match node_to_remove_option {
             Some(removed_node_box) => {
                  // Update the link we have mutable access to (`current_link_ref`)
                  // to point to the node *after* the removed one.
                  *current_link_ref = removed_node_box.next; // `removed_node_box.next` is Link<T>
                  Some(removed_node_box.elem) // Return the owned element
             }
             None => {
                 // Index was out of bounds (link was already None)
                 None
             }
        }
     }
}

// Implement Drop trait
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // Use `while let` which is concise and avoids `is_some()`
        while let Some(_) = self.pop() {
            // The body is empty; we just rely on pop() doing the work
            // of freeing the node until it returns None.
        }
    }
}

// --- Display trait implementation remains the same ---
impl<T: fmt::Display> fmt::Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            write!(f, "{} -> ", node.elem)?;
            current = node.next.as_ref();
        }
        write!(f, "Nil")
    }
}

// --- main function remains the same ---
fn main() {
    let mut list = List::new();
    list.push(3); list.push(2); list.push(1);
    println!("Linked List: {}", list);
    println!("Element at index 1: {:?}", list.get(1));
    list.remove(1);
    println!("List after removing element at index 1: {}", list);
}