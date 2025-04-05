enum IpKind {
    V4(u8, u8, u8, u8),
    V5(String),
}

fn main() {
    let ip1 = IpKind::V4(127, 0, 0, 1);
    let ip2 = IpKind::V5(String::from("127.0.0.1"));
    {
        // options
        let a = Some(5);
        let s = Some("string");
        let n :Option<i32> = None;

        println!("{:?}", a.unwrap_or(0));
        match a {
            Some(a) => println!("some {:?}", a),
            None => println!("None ")
            // for other senarios
            // _ => None
        }
    }
}
