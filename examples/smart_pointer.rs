use std::rc::Rc;

// 1. Box Smart Pointer
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil
}


use List::{Cons, Nil};
fn main() {
    let list = Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil))))));
    println!("Lets test smart pointers Box: {list:?}");

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    println!("count after creating a = {}", Rc::strong_count(&a));
}


// 2. Deref Trait
use std::ops::Deref;
use std::mem::drop;
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
