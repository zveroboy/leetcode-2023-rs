use std::cell::RefCell;
use std::rc::Rc;

type SharedNode<T> = Rc<RefCell<Node<T>>>;

pub struct Node<T: Copy> {
    value: T,
    next: Option<SharedNode<T>>,
}

impl<T: Copy> Node<T> {
    pub fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node { value, next: None }))
    }
}

pub struct List<T: Copy> {
    head: Option<SharedNode<T>>,
    tail: Option<SharedNode<T>>,
}

impl<T: Copy> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push(&mut self, value: T) {
        let node = Node::new(value);

        self.head.get_or_insert_with(|| node.clone());

        if let Some(ref mut tail) = self.tail {
            tail.borrow_mut().next = Some(node.clone());
        }

        self.tail = Some(node);
    }

    pub fn to_vec(&self) -> Vec<T> {
        let mut res = vec![];

        let mut current = self.head.as_ref().and_then(|rc| Some(rc.clone()));

        while let Some(node) = current {
            let node_ref = node.borrow();

            res.push(node_ref.value);
            current = node_ref.next.as_ref().and_then(|rc| Some(rc.clone()));
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut ll = List::<usize>::new();

        ll.push(9);
        ll.push(1);

        assert_eq!(ll.to_vec(), vec![9, 1]);
    }
}
