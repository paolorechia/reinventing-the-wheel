use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq)]
struct List {
    value: i32,
    next: Option<Rc<RefCell<List>>>,
}

impl List {
    fn new(v: i32) -> List {
        List {
            value: v,
            next: None,
        }
    }

    fn push(&mut self, v: i32) {
        if self.next == None {
            self.next = Some(Rc::new(RefCell::new(List::new(v))));
        } else {
            List::_append_iterate(&mut self.next, v);
        }
    }

    fn pop(&mut self) {
        if self.next != None {
            List::_pop_iterate(&mut self.next);
        } else {
            panic!("List is empty!");
        }
    }

    fn _append_iterate(n: &mut Option<Rc<RefCell<List>>>, v: i32) {
       if *n != None {
           List::_append_iterate(&mut n.as_ref().unwrap().borrow_mut().next, v);
       } else {
           *n = Some(Rc::new(RefCell::new(List::new(v))))
       }
    }
    
    fn _pop_iterate(n: &mut Option<Rc<RefCell<List>>>) {
        let mut should_pop = false;
        {
            let next = &mut n.as_ref().unwrap().borrow_mut().next;
            if *next != None {
                List::_pop_iterate(next);
            } else {
                should_pop = true;
            }
        }
        if should_pop {
            *n = None;
        }
    }

    fn get_next(n: &Option<Rc<RefCell<List>>>) -> Option<Rc<RefCell<List>>> {
        match n.as_ref().is_some() {
            true => {
                let is_some = n.as_ref().unwrap().borrow().next.is_some();
                match is_some {
                    true => Some(
                        Rc::clone(n.as_ref().unwrap().borrow().next.as_ref().unwrap())
                    ),
                    false => None
                }
            }
            false => None
        }
    }

    fn _find_tail(n: &Option<Rc<RefCell<List>>>) -> Option<Rc<RefCell<List>>> {
        let mut x = Some(Rc::clone(n.as_ref().unwrap()));
        let mut tail = Some(Rc::clone(n.as_ref().unwrap()));
        let mut has_next = n.as_ref().is_some();
        while has_next {
            let u = Some(Rc::clone(&List::get_next(&x).as_ref().unwrap()));
            println!("{:?}", u);
            has_next = u.as_ref().unwrap().borrow().next.as_ref().is_some();
            println!("has_next: {:?}", has_next);
            x = u;
            if !has_next {
                tail = Some(Rc::clone(x.as_ref().unwrap())); 
            }
        }
        tail
    }
}

fn main () {
    let l = &mut List::new(0);
    println!("{:?}", l);
    l.push(4);
    l.push(5);
    l.push(6);

    let n = List::get_next(&l.next);
    println!("{:?}", n);
    let n2 = List::get_next(&n);
    println!("{:?}", n2);
    let n3 = List::get_next(&n2);
    println!("{:?}", n3);
    println!("{:?}", l);
    println!("Running find tail");
    let tail = List::_find_tail(&l.next);
    println!("Found tail: {:?}", tail);
    println!("{:?}", l);
}
