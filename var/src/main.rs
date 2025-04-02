fn main() {
    const VAR : i32= 2000;
    // you cant change a const with mut keyword
    // shadowing
    let x = 2;
    println!("x {:?}",x );
    let x = String::new();
    println!("x {:?}",x );

    // scalar and compound data types
    // scalar(int(signed, unsigned), float, boolean, char)
    // number representaion 
    let num :i32 = 10_22; //decimal
    println!("decimal {:?}",num );
    let num :i32 = 0xff; //hex
    println!("hex {:?}",num );
    let num :i32 = 0o77 ;//octal
    println!("octal {:?}",num );
    let num :i32 = 0b1111_0000; //binary
    println!("binary {:?}",num );
    let num :u8 = b'A' ;//byte
    println!("byte {:?}",num );
    let num :f32 = 2.3 ;//float
    println!("float {:?}",num );


}
