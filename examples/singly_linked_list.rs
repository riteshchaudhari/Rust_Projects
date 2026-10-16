// A simple singly linked list.
// Run it with: cargo run --example singly_linked_list
//
// The list [10, 20, 30] looks like this:
//
//   head -> [10] -> [20] -> [30] -> None

// One box in the chain: a value and a link to the next box.
// `Option` = there may be no next node (None means "end of list").
// `Box`    = keep the next node on the heap.
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

// The list only remembers its first node.
struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    // Make an empty list.
    fn new() -> LinkedList {
        LinkedList { head: None }
    }

    // Add a value at the front.
    fn push_front(&mut self, value: i32) {
        // take() moves the old head out and leaves None behind.
        let old_head = self.head.take();
        let new_node = Node { value: value, next: old_head };
        self.head = Some(Box::new(new_node));
    }

    // Add a value at the back.
    fn push_back(&mut self, value: i32) {
        // Walk forward until we reach the empty link at the end.
        let mut current = &mut self.head;
        while let Some(node) = current {
            current = &mut node.next;
        }
        // Put the new node into that empty link.
        *current = Some(Box::new(Node { value: value, next: None }));
    }

    // Remove the first value and give it back.
    fn pop_front(&mut self) -> Option<i32> {
        match self.head.take() {
            Some(node) => {
                self.head = node.next; // the second node becomes the head
                Some(node.value)
            }
            None => None, // the list was empty
        }
    }

    // Print every value in the list.
    fn print(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} -> ", node.value);
            current = &node.next;
        }
        println!("None");
    }
}

fn main() {
    let mut list = LinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(30);
    list.print(); // 10 -> 20 -> 30 -> None

    list.push_front(5);
    list.print(); // 5 -> 10 -> 20 -> 30 -> None

    let first = list.pop_front();
    println!("Removed: {:?}", first); // Removed: Some(5)
    list.print(); // 10 -> 20 -> 30 -> None
}
