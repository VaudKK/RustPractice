
fn main() {
    let _v:Vec<i32>= Vec::new();
    let v = vec![1,2,3,4,5];
    let third = &v[2];

    println!("The third item is {third}");

    let third = v.get(2);

    match third {
        Some(i) => println!("Has option {i}"),
        None => println!("No value found")
    };

    if let Some(y) = third {
        println!("My if let worked with value {y}");
    }

    let mut v = vec![10,202,303,404,040];

    for i in &mut v{
        *i += 3000;
        println!("{i}");
    }

    let mut a = String::from("Hello");
    a.push_str(" world");

    println!("{a}");

    let b = String::from("from Vaud");

    let s =  format!("{a} {b}");
    println!("{s}");

    let some_chars = "Здравствуйте";

    for b in some_chars.bytes() {
        println!("{b}");
    }

    println!("The number of bytes is {}", some_chars.bytes().len());
}
