use crate::{Node, NodeRef};
use std::rc::{Rc, Weak};

/// A doubly linked list that maintains references to its first and last nodes.
///
/// Each node contains a key-value pair and maintains links to its previous
/// and next nodes. The list supports efficient insertion, removal, and
/// reordering of nodes.
pub struct DoublyLL<K, V> {
    pub head: Option<NodeRef<K, V>>,
    pub tail: Option<NodeRef<K, V>>,
}

impl<K, V> Default for DoublyLL<K, V> {
    fn default() -> Self {
        return Self::new();
    }
}

impl<K, V> DoublyLL<K, V> {
    /// Creates a new empty doubly linked list.
    ///
    /// # Returns
    /// A new `DoublyLL` instance with both `head` and `tail` set to `None`.
    pub fn new() -> Self {
        return Self {
            head: None,
            tail: None,
        };
    }

    /// Creates a new node and inserts it at the front of the list.
    /// The inserted node becomes the new head of the list.
    ///
    /// # Arguments
    /// * `key` - Key to store in the new node.
    /// * `value` - Value to store in the new node.
    ///
    /// # Returns
    /// A reference-counted pointer to the newly created node.
    pub fn add_to_front(&mut self, key: K, value: V) -> NodeRef<K, V> {
        let new_node = Node::new(key, value);
        self.insert_to_front(new_node.clone());
        return new_node;
    }

    /// Moves an existing node to the front of the list.
    ///
    /// If the node is already the head, the list remains unchanged.
    /// Otherwise, the node is removed from its current position and
    /// inserted at the front.
    ///
    /// # Arguments
    /// * `node` - Reference to the node that should become the new head.
    pub fn move_to_front(&mut self, node: &NodeRef<K, V>) {
        if let Some(head) = &self.head
            && Rc::ptr_eq(head, node)
        {
            return;
        }

        self.remove(node);
        self.insert_to_front(node.clone());
    }

    /// Removes the last node from the list.
    ///
    /// # Returns
    /// * `Some(NodeRef)` - Reference to the removed node.
    /// * `None` - If the list is empty.
    pub fn remove_last(&mut self) -> Option<NodeRef<K, V>> {
        let node = self.tail.clone()?;

        self.remove(&node);

        return Some(node);
    }

    /// Converts the linked list into a vector of key-value pairs.
    ///
    /// The resulting vector preserves the list's traversal order,
    /// starting from the head and ending at the tail.
    ///
    /// # Returns
    /// A vector containing cloned key-value pairs in list order.
    ///
    /// # Note
    /// Both `K` and `V` must implement `Clone`.
    pub fn ll_to_array(&self) -> Vec<(K, V)>
    where
        K: Clone,
        V: Clone,
    {
        let mut arr = Vec::new();
        let mut current = self.head.clone();

        while let Some(node) = current {
            let (key, value, next) = {
                let node_ref = node.borrow();

                (
                    node_ref.key.clone(),
                    node_ref.value.clone(),
                    node_ref.next.clone(),
                )
            };

            arr.push((key, value));
            current = next;
        }

        return arr;
    }

    fn insert_to_front(&mut self, node: NodeRef<K, V>) {
        let mut node_ref = node.borrow_mut();
        node_ref.prev = None;
        node_ref.next = self.head.clone();
        drop(node_ref);

        if let Some(head) = &self.head {
            head.borrow_mut().prev = Some(Rc::downgrade(&node));
        }

        if self.tail.is_none() {
            self.tail = Some(node.clone());
        }

        self.head = Some(node);
    }

    fn remove(&mut self, node: &NodeRef<K, V>) {
        let (prev, next) = {
            let node_ref = node.borrow();
            (
                node_ref.prev.as_ref().and_then(Weak::upgrade),
                node_ref.next.clone(),
            )
        };

        if let Some(prev) = &prev {
            prev.borrow_mut().next = next.clone();
        } else {
            self.head = next.clone();
        }

        if let Some(next) = &next {
            next.borrow_mut().prev = prev.as_ref().map(Rc::downgrade);
        } else {
            self.tail = prev;
        }

        let mut mutable_node_ref = node.borrow_mut();
        mutable_node_ref.next = None;
        mutable_node_ref.prev = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_list() -> DoublyLL<String, i32> {
        DoublyLL::new()
    }

    fn add(list: &mut DoublyLL<String, i32>, key: &str, value: i32) -> NodeRef<String, i32> {
        list.add_to_front(key.to_string(), value)
    }

    fn expected(items: &[(&str, i32)]) -> Vec<(String, i32)> {
        items
            .iter()
            .map(|(key, value)| (key.to_string(), *value))
            .collect()
    }

    fn assert_links(list: &DoublyLL<String, i32>) {
        let mut current = list.head.clone();
        let mut previous: Option<NodeRef<String, i32>> = None;

        while let Some(node) = current {
            let node_ref = node.borrow();
            let actual_prev = node_ref.prev.as_ref().and_then(Weak::upgrade);

            match (&previous, &actual_prev) {
                (Some(expected), Some(actual)) => {
                    assert!(Rc::ptr_eq(expected, actual));
                }
                (None, None) => {}
                _ => panic!("Invalid previous pointer"),
            }

            current = node_ref.next.clone();
            previous = Some(node.clone());
        }

        match (&previous, &list.tail) {
            (Some(expected), Some(actual)) => {
                assert!(Rc::ptr_eq(expected, actual));
            }
            (None, None) => {}
            _ => panic!("Invalid tail pointer"),
        }
    }

    #[test]
    fn test_new_empty_list() {
        let list = create_list();

        assert_eq!(list.ll_to_array(), vec![]);
        assert!(list.head.is_none());
        assert!(list.tail.is_none());
    }

    #[test]
    fn test_add_to_front() {
        let mut list = create_list();

        add(&mut list, "A", 1);
        add(&mut list, "B", 2);
        add(&mut list, "C", 3);

        assert_eq!(
            list.ll_to_array(),
            expected(&[("C", 3), ("B", 2), ("A", 1)])
        );

        assert_links(&list);
    }

    #[test]
    fn test_add_to_front_single_node() {
        let mut list = create_list();

        let node = add(&mut list, "A", 1);

        assert!(Rc::ptr_eq(list.head.as_ref().unwrap(), &node));
        assert!(Rc::ptr_eq(list.tail.as_ref().unwrap(), &node));

        assert_eq!(list.ll_to_array(), expected(&[("A", 1)]));
        assert_links(&list);
    }

    #[test]
    fn test_move_middle_node_to_front() {
        let mut list = create_list();

        add(&mut list, "A", 1);
        let b = add(&mut list, "B", 2);
        add(&mut list, "C", 3);

        list.move_to_front(&b);

        assert_eq!(
            list.ll_to_array(),
            expected(&[("B", 2), ("C", 3), ("A", 1)])
        );

        assert_links(&list);
    }

    #[test]
    fn test_move_tail_to_front() {
        let mut list = create_list();

        let a = add(&mut list, "A", 1);
        add(&mut list, "B", 2);
        add(&mut list, "C", 3);

        list.move_to_front(&a);

        assert_eq!(
            list.ll_to_array(),
            expected(&[("A", 1), ("C", 3), ("B", 2)])
        );

        assert_links(&list);
    }

    #[test]
    fn test_move_head_to_front() {
        let mut list = create_list();

        add(&mut list, "A", 1);
        add(&mut list, "B", 2);
        let c = add(&mut list, "C", 3);

        list.move_to_front(&c);

        assert_eq!(
            list.ll_to_array(),
            expected(&[("C", 3), ("B", 2), ("A", 1)])
        );

        assert_links(&list);
    }

    #[test]
    fn test_remove_middle_node() {
        let mut list = create_list();

        let a = add(&mut list, "A", 1);
        let b = add(&mut list, "B", 2);
        add(&mut list, "C", 3);

        list.remove(&b);

        assert_eq!(list.ll_to_array(), expected(&[("C", 3), ("A", 1)]));

        assert!(b.borrow().next.is_none());
        assert!(b.borrow().prev.is_none());

        assert_links(&list);

        let _ = a;
    }

    #[test]
    fn test_remove_head() {
        let mut list = create_list();

        add(&mut list, "A", 1);
        add(&mut list, "B", 2);
        let c = add(&mut list, "C", 3);

        list.remove(&c);

        assert_eq!(list.ll_to_array(), expected(&[("B", 2), ("A", 1)]));

        assert!(c.borrow().next.is_none());
        assert!(c.borrow().prev.is_none());

        assert_links(&list);
    }

    #[test]
    fn test_remove_tail() {
        let mut list = create_list();

        let a = add(&mut list, "A", 1);
        add(&mut list, "B", 2);
        add(&mut list, "C", 3);

        list.remove(&a);

        assert_eq!(list.ll_to_array(), expected(&[("C", 3), ("B", 2)]));

        assert!(a.borrow().next.is_none());
        assert!(a.borrow().prev.is_none());

        assert_links(&list);
    }

    #[test]
    fn test_remove_only_node() {
        let mut list = create_list();

        let a = add(&mut list, "A", 1);

        list.remove(&a);

        assert!(list.head.is_none());
        assert!(list.tail.is_none());
        assert_eq!(list.ll_to_array(), vec![]);

        assert!(a.borrow().next.is_none());
        assert!(a.borrow().prev.is_none());
    }

    #[test]
    fn test_remove_last_from_empty_list() {
        let mut list = create_list();

        assert!(list.remove_last().is_none());

        assert!(list.head.is_none());
        assert!(list.tail.is_none());
    }

    #[test]
    fn test_remove_last_single_node() {
        let mut list = create_list();

        add(&mut list, "A", 1);

        let removed = list.remove_last().unwrap();

        assert_eq!(removed.borrow().key, "A");
        assert_eq!(list.ll_to_array(), vec![]);

        assert!(list.head.is_none());
        assert!(list.tail.is_none());

        assert!(removed.borrow().next.is_none());
        assert!(removed.borrow().prev.is_none());
    }

    #[test]
    fn test_remove_last_multiple_nodes() {
        let mut list = create_list();

        add(&mut list, "A", 1);
        add(&mut list, "B", 2);
        add(&mut list, "C", 3);

        let removed = list.remove_last().unwrap();

        assert_eq!(removed.borrow().key, "A");

        assert_eq!(list.ll_to_array(), expected(&[("C", 3), ("B", 2)]));

        assert_links(&list);
    }
}
