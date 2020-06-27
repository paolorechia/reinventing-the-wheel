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
            next: None
        }
    }

    fn append(n: &mut Option<Rc<RefCell<List>>>, v: i32) -> () {
       println!("Calling append on {:?}", n);
       if *n != None {
           List::append(&mut n.as_ref().unwrap().borrow_mut().next, v);
       } else {
           println!("Found None!");
           *n = Some(Rc::new(RefCell::new(List::new(v))))
       }
    }
    
    fn pop(n: &mut Option<Rc<RefCell<List>>>) -> () {
        let mut should_pop = false;
        {
            let next = &mut n.as_ref().unwrap().borrow_mut().next;
            if *next != None {
                List::pop(next);
            } else {
                should_pop = true;
            }
        }
        if should_pop {
            *n = None;
        }
    }
}

fn main () {
    let mut l = &mut List::new(2);
    println!("{:?}", l);
    l.value = 1;
    println!("{:?}", l);

    let l2 = Some(Rc::new(RefCell::new(List::new(2))));
    
    let l3 = Some(Rc::new(RefCell::new(List::new(3))));
    
    l.next = l2;
    
    let p = &(l.next);

    let link = p.as_ref();
    println!("{:?}", link);
    
    link.unwrap().borrow_mut().next = l3;

    println!("{:?}", l);

    List::append(&mut l.next, 4);
    List::append(&mut l.next, 5);
    List::append(&mut l.next, 6);
    List::append(&mut l.next, 7);

    println!("{:?}", l);
    
    List::pop(&mut l.next);
    println!("{:?}", l);
    
}
