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

    
    /*
    fn _find_tail(n: &mut Option<Rc<RefCell<List>>>) -> &Option<Rc<RefCell<List>>> {
        while n.hasNext {
            n = &mut n.as_ref().unwrap().borrow_mut().next;
        }
    }
    */
    
}

fn main () {
    let l = &mut List::new(0);
    println!("{:?}", l);
    l.push(4);
    l.push(5);
    l.push(6);

//    List::_find_tail(&mut l.next);
}
