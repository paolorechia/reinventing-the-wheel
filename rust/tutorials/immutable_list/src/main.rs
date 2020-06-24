#[derive(Debug, PartialEq)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};


fn head(l: &List) -> Option<i32> {
    match l {
        Cons(n, _) => Some(*n),
        Nil => None
    }
}

fn tail(l: &List) -> Option<&List> {
    match l {
        Cons(n, b) => Some(&b),
        Nil => None
    }
}


fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
    
    let h = head(&list);
    println!("{:?}", h);

    let mut t = tail(&list).unwrap();
    let h = head(&t);
    println!("{:?}", h);
    while *t != List::Nil {
        t = tail(&t).unwrap();
        let h = head(&t);
        println!("{:?}", h);
    }
}
