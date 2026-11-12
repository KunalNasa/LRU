use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type NodeRef<K, V> = Rc<RefCell<Node<K, V>>>;
pub struct Node<K, V> {
    pub next: Option<NodeRef<K, V>>,
    pub prev: Option<Weak<RefCell<Node<K, V>>>>,
    pub key: K,
    pub value: V,
}

impl<K, V> Node<K, V> {
    pub fn new(key: K, value: V) -> NodeRef<K, V> {
        return Rc::new(RefCell::new(Self {
            key,
            value,
            next: None,
            prev: None,
        }));
    }
}
