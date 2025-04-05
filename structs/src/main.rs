fn main() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32
    }

    // methods are tied to an instance of the strcut (self) and can be accesed with . notation
    impl Rectangle {
        fn area(&self)->u32{
            self.width * self.height
        }
    }


    // associated functions are not tied to an instance if the struct and can be accessed with :: notation
    impl Rectangle {
        fn new(width: u32, height: u32)->Rectangle{
            Rectangle {
                width,
                height

            }
        }
    }

    let rec = Rectangle::new(12,32);
    let rec_area = rec.area();
    println!("{:#?}", rec_area);
}
