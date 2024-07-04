use core::slice;

static HELLO_WORLD: &str = "Hello, world";

extern "C" {
    fn abs(input: i32) -> i32;
}

type kilometers = i32;

#[derive(Debug,Copy,Clone,PartialEq)]
struct Point{
    x: i32,
    y: i32,
}

use std::ops::Add;

impl Add for Point{
    type Output = Point;
    fn add(self, rhs: Self) -> Point {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog{
    fn baby_name() -> String{
        String::from("Guss")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("Puppy")
    }
}

fn generic<T>(t: T) {
    // --snip--
}

fn genericSizedOrUnsized<T: ?Sized>(t: &T){

}

fn add_one(x: i32) -> i32{
    x +1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32{
    f(arg) + f(arg)
}

#[derive(Debug)]
enum Status{
    Value(u32),
    Stop
}

macro_rules! vauds_vec {
    ($( $x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {

    let my_vec = vauds_vec!(1,4,5,6);

    println!("My vector {:?}",my_vec);

    let list_of_statuses: Vec<Status> = (0u32..=20).map(Status::Value).collect();

    println!("List values {:#?}",list_of_statuses);

    let answer = do_twice(add_one, 5);

    println!("The answer is {answer}");

    let some_number = 40;
    let k: kilometers = 300;

    println!("Type alias summation: {}", some_number + k);

    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}",<Dog as Animal>::baby_name());

    assert_eq!(
        Point{x:1,y:0} + Point{x:2,y:3},
        Point{x:3,y:3}
    );

    let mut num = 5;

    let r1 = &num as *const i32; // immutable raw pointer
    let r2 = &mut num as *mut i32; // mutable raw pointer

    unsafe{
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    let mut v = vec![1,2,3,4,5,6];

    let r = &mut v[..];

    let (a,b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1,2,3]);
    assert_eq!(b, &mut [4,5,6]);

    unsafe{
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

#[no_mangle]
pub extern "C" fn call_from_c(){
    println!("Just called a Rust function from C!");
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32],&mut [i32]){
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe{
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid)
        )
    }
}
