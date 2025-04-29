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

    {
        use std::ops::Deref;
        // smart pointers are struct that implement deref and drop trair
        // deref 
        #[derive(Debug)]
        struct My_box<T>(T);
        impl<T> My_box<T> {
            fn new(x: T)-> My_box<T>{
                My_box(x)
            }
            
        }
        impl<T: std::fmt::Debug> Deref for My_box<T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                println!("the data ({:?}) was derefed.", self.0);
                &self.0
            }
            
        }

        let base = "x".to_string();
        let data = My_box::new(base);
        
        assert_eq!("x".to_string(), *data);
        // drop trait
        struct Custom_pointer {
            data: String
        }

        impl Drop for  Custom_pointer {

            fn drop(&mut self) {
                println!("the data ({:?}) was dropped.", self.data)
            }
            
        }

        let a = Custom_pointer{
            data: "hi".to_string()
        };


    }

    {
        // reference couting and how it let us share ownership
    }
}
