use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct A {
    v: i32,
    s: Weak<RefCell<A>>,
}

impl A {
    fn new() -> Rc<RefCell<A>> {
        let a = A {
            v: 20,
            s: Weak::new(),
        };
        let p = Rc::new(RefCell::new(a));
        p.borrow_mut().s = Rc::downgrade(&p);
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ref_self() {
        let a = A::new();
        let aa = a.borrow().s.upgrade().unwrap();

        assert!(Rc::ptr_eq(&a, &aa));
    }
}
