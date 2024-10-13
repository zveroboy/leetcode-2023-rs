use std::cell::RefCell;
use std::rc::Rc;

pub struct Node<T: Copy> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Copy> Node<T> {
    pub fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node { value, next: None }))
    }
}

pub struct List<T: Copy> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
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

        self.head.get_or_insert_with(|| Rc::clone(&node));

        if let Some(ref mut tail) = self.tail {
            tail.borrow_mut().next = Some(Rc::clone(&node));
        }

        self.tail = Some(node);
    }

    pub fn to_vec(&self) -> Vec<T> {
        let mut res = vec![];

        let mut current = match &self.head {
            Some(node) => Rc::clone(&node),
            None => return res,
        };

        loop {
            res.push(current.as_ref().borrow().value); //works because value is Copy

            let node = match &current.borrow().next {
                Some(node) => Rc::clone(&node),
                None => break,
            };
            current = node;
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
