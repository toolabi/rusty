use std::arch::aarch64::float32x2_t;

fn main() {
    // generics 
    // to reduce code duplication
    // use one function for a few types
    let nums = [1,3,65,75,32].to_vec();
    let chars = ['a','c','g','y'].to_vec();
    println!("largest number: {:?}", get_largest(nums));
    println!("largest char: {:?}", get_largest(chars));
    println!("Hello, world!");


    {
        // we can use multiple generics
        // for the times we have different types in one struct

        struct Nums<T,U>{
            a: T,
            b: U
        }

        let numy = Nums{
            a: 12,
            b: 12.0
        };
        impl<T,U> Nums<T,U> {
            fn get_a(&self) -> &T {
                &self.a
            }
            
        }
        impl Nums<f64,i32> {
            fn get_float_a(&self)->&f64{
                &self.a
            }
            
        }

        let numnum = Nums{
            a: 2.2,
            b:23
        };

        println!("get floaty num num {:?}", numnum.get_float_a());
    }
}

fn get_largest<T: PartialOrd + Copy >(list:Vec<T>)->T {

    let mut place_holder = list[0];
    for i in list {
        if i > place_holder{
            place_holder = i;
        }
    }
    place_holder
}
