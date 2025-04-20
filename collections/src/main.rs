// collections are allocated in heap(vectors, strings, hashmaps)
fn main() {
    let arr = [1,2,3,4];
    // can not grow arrays
    // arr.push(5);
    let mut v : Vec<u8> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v);

    // refrencing a valur in this way can cause the code to crash
    // cuase we dont know the length at compile time to throw an error
    let n = v[2]; 

    // to resolve having a error at compile time we can use this to adress a variable
    let second_variable = v.get(40);
    match  second_variable  {
        Some(num)=>println!("we got a number: {:?}", num),
        None => print!("no number!!")
        
    }


    // strings as a collection

    let st = "abcdef".to_string();
    // get the bytes
    println!("{:?}", st.bytes());


    {
        //hash map
        use std::collections::HashMap;

        let mut hm = HashMap::new();

        hm.insert("rust", 1);
        hm.insert("solidity", 2);
        hm.insert("circom", 3);

        // the return is an option because at the time of inssering we dont know the value is valid
        let get_rust = hm.get("rust");


        let message = String::from("rust solidity rust hey hi");

        let mut message_count = HashMap::new();

        for i in message.split_whitespace(){
            let count = message_count.entry(i).or_insert(0);
            *count +=1;
        }

        println!("{:?}", message_count);
    }
}
