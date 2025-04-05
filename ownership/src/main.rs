fn main() {
    // memory
    // stack(functions and fixed size variables) and heap(dynamic size variables)
    // ownership
    // 1. each value has one variable called owner in rust
    // 2. there can be one owner at the time
    // 3. owner out of scope => value is dropped

    let s = String::from("99");
    let newOwner = s;


    // println!("{:?}", s);
    // throws error
    // |
    // 9  |     let s = String::from("99");
    //    |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
    // 10 |     let newOwner = s;
    //    |                    - value moved here
    // 11 |
    // 12 |     println!("{:?}", s);
    //    |                      ^ value borrowed here after move
    //    |

    
    let s = String::from("99");
    let newOwner = s.clone();


    println!("s: {:?}", s);

    // if the varibale type implements the trait "copy", assiging the varibale to a new one does not destroy the old one
    let s: u8 = 32;
    // no need to clone
    let newOwner = s;
    println!("s: {:?}", s);

}
