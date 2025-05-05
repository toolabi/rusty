
use trait_objects::{Draw, Screen,Button};


pub struct SelectBox {
    pub option: usize
}

impl Draw for SelectBox {

    fn draw(&self) {
        // draw
    }
    
}


fn main() {
    // to implement the inheritance in rust we use trait objects 
    // we use object triats Vec<Box<dyn Draw>> instead of generic traits(Vec<T>) because we might want to have several types in the vector 
    // compiler doesnt know conrete methods you're calling at compile time => dynamic dispatch => adds run time
    // when using trait objects we use dynamic dispatch

    // object safty
    // you can ONLY make object safe traits into trait bounds
    // object safe trait:
    // 1.the return type of methods is not self
    // 2. there are no generic paramteres
    let s = Screen {
        components: vec![Box::new(
            Button{
                width:2
            }
        ),
        Box::new(
            SelectBox{
                option:2
            }
        )
        ]
    };
    s.run();
}


