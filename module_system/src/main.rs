// if you have main in your source dir a binary crate with the  name as your package is being created
// packages store crates
// main.rs is the crate root
// "crate root" is a source file that rusts compiler starts at when building your crate
// it also makes out the root module of your crate
// same for libraries as lib.rs

// rules
// a package must have at least one crate
// a package can have at least 0 or 1 library crate
// a package can have any number of binary crate(more in the bin folder)

// modules
// mod name {
// }
fn main() {
    mod front{
        pub mod print{
            pub fn hello(){
                println!("gelloooo");
            }
        }

    }
    front::print::hello();
    println!("Hello, world!");

    mod back {
        pub mod print{
            pub fn call_bye(){
                super::super::bye();
            }
        }

    }
    back::print::call_bye()
}
pub fn bye(){
    println!("byeeee");
}
