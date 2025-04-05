
#[derive(Debug)]
enum IpKind {
    V4(u8, u8, u8, u8),
    V5(String)
}

fn main() {
    let ip1 = IpKind::V4(127,0,0,1);
    println!("{:?}", ip1);
    let ip2 =IpKind::V5(String::from("127.0.0.1"));
    println!("{:?}", ip2);
}
