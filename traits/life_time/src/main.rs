fn main() {
    // life time
    // dangling refrence => a refrence that points to an unvalid data
    // rusts borrow checker runs at compile time
    // generic life time annotation
    println!("Hello, world!");
    let mut result; 

    {
        // they have the same life time
        let a = "abnasdkljfaf".to_string();
        let b = "abnasdkl".to_string();

        result = get_longest(&a, &b);
        println!("longest {:?}", result);
        
    }
    {
        let a = "abnasdkljfaf".to_string();
        {
            let b = "abnasdkl".to_string();
             result = get_longest(&a, &b);
            // this work bc result life time is as much as b 
            // and we call result in the scope that b is still valid
            println!("longest {:?}", result);

        }
        // result does not exist here
        // println!("longest {:?}", result);

    }
    {
        let a = "abnasdkljfaf".to_string();
        {
            let b = "abnasdkl".to_string();
             result = get_first(&a, &b);

        }
        // result works here becasue it's life time is as much as a which is outside this scope
        println!("longest {:?}", result);

    }

    // static life time means the variables life time is as long as the progarm
}

// throws error
// fn get_longest(a: &String, b: &String)-> &String{
//     if a.len() > b.len() {
//         a
//     }else {
//         b
//     }
// }
// in this way we specify that the results life time is as much as the shortest life time between a and b
fn get_longest<'a>(a: &'a String, b: &'a String)-> &'a String{
    if a.len() > b.len() {
        a
    }else {
        b
    }
}
fn get_first<'a>(a: &'a String, b: & String)-> &'a String{
    a
}
