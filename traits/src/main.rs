use std::fmt::{format, Display};

fn main() {
    // traits shared behavior
    println!("Hello, world!");

    struct Tweet {
        user: String,
        likes: i32
    }

    impl Sum for Tweet {
        fn summerize(&self)-> String {
            format!("{}, {}",self.user, self.likes)
            
        }
        
    }

    struct News {
        author: String,
        sold: i32
    }

    impl Sum for News {}


    pub trait Sum {
        fn summerize(&self)->String{
            format!("read more ... ")
        }
        // fn get_auth<T: Copy+ Display>()

    }

    let t = Tweet{
        user: "big head ben".to_string(),
        likes: 20
    };
    let n = News {
        author: "big head ben".to_string(),
        sold: 6
    };


    println!(" t sum : {:?}", t.summerize());
    println!(" n sum : {:?}", n.summerize());


    // traits as inputs

    fn notify(item: &impl Sum){
        println!("here is the sumery: {:?}", item.summerize());
    }

    notify(&n);
    notify(&t);

    // trais as outputs 
    // usefull inside closures and iterator
    fn return_summerizable()-> impl Sum {
        Tweet{
            user: "matty".to_string(),
            likes: 23
        }
    }

    println!("return_summerizable {:?}", return_summerizable().summerize())


}
