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

    
        // interior mutability 
        // allows mutate data when there are immutable refrence to that data
        // box inforces bowrowing rules at compile time
        // refcell inforces bowrowing rules at compile time <= only in signle thread programs
        
        // you can have mutable value inside refCell<T> even when the  refCell<T> is imutable.
        
        // targe change a mut data thats been refrenced by in immutable data
        let mut num = 10;
        let num_ref = &num;

        // this throws an error 
        // cannot assign to `*num_ref`, which is behind a `&` reference
        // *num_ref = 12;


    
}


pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker <'a, T: Messenger>{
    messanger: &'a T,
    value:usize,
    max: usize

}

impl<'a,T> LimitTracker<'a, T> 
where T : Messenger
{
    pub fn new(messenger: &'a T, max: usize)->LimitTracker<'a, T>{
        LimitTracker{
            messanger :messenger,
            value:0,
            max: max
        }

    }

    pub fn set_value(&mut self, value: usize){
        self.value = value;

        let div = value as f64 / self.max as f64;

        if div > 1.0 {
            self.messanger.send("Error: you are over your qouta");
        }else if div > 0.9{
            self.messanger.send("Warning: you used over 90 percent of your qouta");
            
        }else if div > 0.75 {
            self.messanger.send("Warning: you used over 75 percent of your qouta");
        }


    }

    
}


#[cfg(test)]
mod tests{

    use super::*;
    use super::*;
    use super::Messenger;
    use std::cell::RefCell;


    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(Vec::new()),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
            // fails at run time if you borrow twice
            // self.sent_messages.borrow_mut().push(String::from(message));
        }
    }
    
    #[test]

    fn test(){
        let meesanger = MockMessenger::new();

        let mut limit_tracker = LimitTracker::new(&meesanger, 100);

        limit_tracker.set_value(80);

        assert_eq!(meesanger.sent_messages.borrow().len(),1);
        

    }
    

}
