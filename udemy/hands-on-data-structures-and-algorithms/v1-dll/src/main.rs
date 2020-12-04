// Immutable outside, but can mutate interior.
use std::cell::RefCell;
// Reference Counting pointer
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct DLLNode<T> {
    data: T,
    next: Option<Rc<RefCell<DLLNode<T>>>>,
    prev: Option<Weak<RefCell<DLLNode<T>>>>,
}

#[derive(Debug)]
pub struct DLL<T> {
    first: Option<Rc<RefCell<DLLNode<T>>>>,
    last: Option<Weak<RefCell<DLLNode<T>>>>,
}

impl<T> DLL<T> {
    pub fn new() -> DLL<T> {
        DLL {
            first: None,
            last: None,
        }
    }
    pub fn push_front(&mut self, data: T) {
        match self.first.take() {
            Some(r) => {
                let new_front = Rc::new(RefCell::new(DLLNode {
                    data,
                    next: Some(r.clone()),
                    prev: None,
                }));
                let mut m = r.borrow_mut();
                m.prev = Some(Rc::downgrade(&new_front));
                self.first = Some(new_front);
            }
            None => {
                let new_data = Rc::new(RefCell::new(DLLNode {
                    data,
                    next: None,
                    prev: None,
                }));
                self.last = Some(Rc::downgrade(&new_data));
                self.first = Some(new_data);
            }
        }
    }
    pub fn push_back(&mut self, data: T) {
        match self.last.take() {
            Some(r) => {
                let new_back = Rc::new(RefCell::new(DLLNode {
                    data,
                    next: None,
                    prev: Some(r.clone()),
                }));
                let st = Weak::upgrade(&r).unwrap();
                let mut m = st.borrow_mut();
                self.last = Some(Rc::downgrade(&new_back));
                m.next = Some(new_back);
            }
            None => {
                let new_data = Rc::new(RefCell::new(DLLNode {
                    data,
                    next: None,
                    prev: None,
                }));
                self.last = Some(Rc::downgrade(&new_data));
                self.first = Some(new_data);
            }
        }
    }
}

fn main() {
    let mut dll: DLL<i32> = DLL::new();
    dll.push_front(6);
    dll.push_back(11);
    dll.push_front(12);
    dll.push_back(13);
    println!("{:?}", dll);
}
