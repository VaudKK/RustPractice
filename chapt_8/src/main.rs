
use std::collections::HashMap;

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

    let mut scores = HashMap::new();

    scores.insert(String::from("Red"), 10);
    scores.insert(String::from("Yellow"), 20);

    scores.entry(String::from("Blue")).or_insert(30);

    for (_,_) in &scores{
        println!("The scores are {:#?}",scores);
    }

    let score = scores.get("Blue").copied().unwrap_or(0);

    println!("{score}");

    if let Some(my_score) = scores.get("Blue"){
        print!("Value of my score {my_score}");
    }else{
        println!("Just practising on if let");
    }

    let text = "hello africa we are amazing as africa";

    let mut map = HashMap::new();

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count +=1;
    }

    println!("\n{:#?}", map);

}
