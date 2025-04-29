// smart pointer 
// box is a smart pointer that lets you allocate memory on the heap
// used when:
// you have a data that its size is not known at compile time and you need to use a value of that time in a way that it should be known at compile time
// when you have large amount of data and you want to transfer it's ownership but make sure it's not copied.
// when you own a value and you care that value implements a trait


use crate::List::{Cons, Nil};
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {



    {
        // pointers are variables that hold a memory address that point to a data in memory
        // basic pointer id refrence & 
        // box allows you to allocate memory on the heap
        // let a = Box::new(5);
        // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    }

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
        let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Nil)))));
        println!("{:?}", Rc::strong_count(&a));
        let b = Cons(4, Rc::clone(&a));
        println!("{:?}", Rc::strong_count(&a));
        {
            let c = Cons(5, Rc::clone(&a));
            println!("{:?}", Rc::strong_count(&a));

        }
        println!("{:?}", Rc::strong_count(&a));

    }
}
