use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::ops::Deref;
use std::mem::drop;

// 1. Box Smart Pointer
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil
}


use List::{Cons, Nil};
fn main_box() {
    let list = Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil))))));
    println!("Lets test smart pointers Box: {list:?}");

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    println!("count after creating a = {}", Rc::strong_count(&a));
}


// 2. Deref Trait
/* Deref coercion converts a reference to a type that implements the Deref trait into a reference to another type. 
    For example, deref coercion can convert &String to &str \
        because String implements the Deref trait such that it returns &str. 
*/


struct CustomBox<T>(T);

impl<T> CustomBox<T> {
    fn new(x:T) -> CustomBox<T> {
        Self(x)
    }
}

impl<T> Deref for CustomBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

/*
Rust does deref coercion when it finds types and trait implementations in three cases:
    From &T to &U when T: Deref<Target=U>
    From &mut T to &mut U when T: DerefMut<Target=U>
    From &mut T to &U when T: Deref<Target=U>
*/ 

fn main_deref() {
    let x = 5;
    let y = CustomBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = CustomBox::new(String::from("Rust"));
    hello(&m);
}


// Reference Cycle
#[derive(Debug)]
enum List2 {
    Cons(i32, RefCell<Rc<List2>>),
    Nil,
}

impl List2 {
    fn tail(&self) -> Option<&RefCell<Rc<List2>>> {
        match self {
        List2::Cons(_, item) => Some(item),
            List2::Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
