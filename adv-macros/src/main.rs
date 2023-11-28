// Define a trait for a generic LinkedList
pub trait LinkedList<T> {
    fn push(&mut self, value: T);
    fn pop(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
}

// Define a simple Node struct
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

// Implement the LinkedList trait for a singly-linked list
pub struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> Self {
        SinglyLinkedList { head: None }
    }
}

impl<T> LinkedList<T> for SinglyLinkedList<T> {
    fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }
}

fn main() {
    let mut list = SinglyLinkedList::new();
    
    list.push(1);
    list.push(2);
    list.push(3);
    
    println!("Peek: {:?}", list.peek()); // Some(3)
    
    println!("Pop: {:?}", list.pop()); // Some(3)
    println!("Pop: {:?}", list.pop()); // Some(2)
    println!("Pop: {:?}", list.pop()); // Some(1)
    println!("Pop: {:?}", list.pop()); // None
}
