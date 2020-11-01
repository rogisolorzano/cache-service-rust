use std::rc::{Rc};
use std::cell::{RefCell, Ref};

pub struct DoublyLinkedList<T> {
    pub head: Option<Rc<RefCell<DoublyLinkedListNode<T>>>>,
    pub tail: Option<Rc<RefCell<DoublyLinkedListNode<T>>>>,
}

// TODO: finish implementation
impl<T> DoublyLinkedList<T> {
    /// Sets the head of the linked list.
    pub fn set_head(&mut self, node: Rc<RefCell<DoublyLinkedListNode<T>>>) {
        if self.head.is_none() {
            self.head = Some(node.clone());
            self.tail = Some(node);
        }
    }

    /// Removes the tail of the linked list.
    pub fn remove_tail(&mut self) {
        if self.tail.is_none() {
            return;
        }

        if Rc::ptr_eq(self.head.as_ref().unwrap(), self.tail.as_ref().unwrap()) {
            self.tail = None;
            self.head = None;
            return;
        }

        let prev = self.tail.as_ref().unwrap().borrow().previous.clone();
        self.tail = prev;
        self.tail.as_ref().unwrap().borrow_mut().next = None;
    }

    /// Removes a node in the linked list.
    pub fn remove_node(&self, node: &mut Rc<RefCell<DoublyLinkedListNode<T>>>) {
        node.borrow_mut().next = None;
        node.borrow_mut().previous = None;
    }
}

pub struct DoublyLinkedListNode<U> {
    pub value: U,
    pub previous: Option<Rc<RefCell<DoublyLinkedListNode<U>>>>,
    pub next: Option<Rc<RefCell<DoublyLinkedListNode<U>>>>,
}
