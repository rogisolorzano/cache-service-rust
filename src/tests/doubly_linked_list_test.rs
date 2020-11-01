#[cfg(test)]
mod tests {
    use crate::common::data_structures::doubly_linked_list::{DoublyLinkedList, DoublyLinkedListNode};
    use std::cell::RefCell;
    use std::rc::Rc;
    use actix_web::web::head;

    #[test]
    fn can_assign_to_head_and_tail() {
        let mut list = DoublyLinkedList{
            head: None,
            tail: None
        };
        let node = DoublyLinkedListNode {
            value: String::from("First!"),
            previous: None,
            next: None
        };

        list.set_head(Rc::new(RefCell::new(node)));

        assert_eq!("First!", list.head.unwrap().borrow().value);
        assert_eq!("First!", list.tail.unwrap().borrow().value);
    }

    #[test]
    fn can_remove_tail() {
        let head_node = Some(Rc::new(RefCell::new(
            DoublyLinkedListNode {
                value: String::from("Head!"),
                previous: None,
                next: None,
            }
        )));
        let tail_node = Some(Rc::new(RefCell::new(
            DoublyLinkedListNode {
                value: String::from("Tail!"),
                previous: head_node.clone(),
                next: None,
            }
        )));
        let mut list = DoublyLinkedList{
            head: head_node,
            tail: tail_node,
        };

        list.remove_tail();

        assert_eq!("Head!", list.tail.unwrap().borrow().value);
    }


    #[test]
    fn removes_equal_tail_and_head() {
        let mut list = DoublyLinkedList{
            head: None,
            tail: None
        };
        let node = DoublyLinkedListNode {
            value: String::from("Head!"),
            previous: None,
            next: None
        };

        list.set_head(Rc::new(RefCell::new(node)));
        list.remove_tail();

        assert_eq!(true, list.head.is_none());
        assert_eq!(true, list.tail.is_none());
    }
}
