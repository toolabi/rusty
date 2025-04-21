// smart pointer 
// box is a smart pointer that lets you allocate memory on the heap
// used when:
// you have a data that its size is not known at compile time and you need to use a value of that time in a way that it should be known at compile time
// when you have large amount of data and you want to transfer it's ownership but make sure it's not copied.
// when you own a value and you care that value implements a trait


enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
