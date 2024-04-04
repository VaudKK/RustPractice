// enum IpAddrKind{
//     V4(u8,u8,u8,u8),
//     V6(String)
// }

fn main() {
    // let home = IpAddrKind::V4(127,0,0,1);
    // let loopback = IpAddrKind::V6(String::from("::!"));

    let x = Some(10);
    // let y = 10_i32;

    if let Option::Some(y) = x {
        println!("Has some value {}", y);
    }else{
        println!("Has no value");
    }

    let sum = plus_one(x);
    println!("The sum is {}", sum.unwrap());
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
