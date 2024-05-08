use add_one;
use add_two;

fn main() {
    let num = 10;
    println!("Hello, world! {num} plust one is {}", add_one::add_one(num));
    print!("Adding two: {}", add_two::add_two(12));
}
