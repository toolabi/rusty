use std::fs::{self, File};
use std::io;
use std::io::Read;
fn read() -> Result<String, io::Error>{
    println!("Hello, world!");

    // let mut f = File::open("data.json")?;
    // // the return value is relust you can you match to handle it

    // // let mut f = match f {
    // //     Ok(file) => file,
    // //     Err(e)=> return Err(e),
    // // };

    // let mut s = String::new();
    // match f.read_to_string(&mut s) {
    //     Ok(_)=>Ok(s),
    //     Err(e)=>Err(e)
    // }

    fs::read_to_string("data.json")
    // returning an error in the function to be handled by the caller <= error propagation

}

fn main(){
    let a = read();
    // ?  when encounters an error will return the error
    // but unwrap will panic
}
