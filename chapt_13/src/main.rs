use std::thread;

#[derive(Debug, PartialEq,Clone, Copy)]
enum ShirtColour{
    Red,
    Blue,
}

struct Inventory{
    shirts: Vec<ShirtColour>,
}

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

#[derive(Debug)]
enum Colour{
    Red,
    Green,
    Blue
}

#[derive(Debug)]
struct Shoe {
    size: u32,
    colour: Colour
}

fn shoes_in_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe>{
    shoes.into_iter().filter(|shoe| shoe.size == size).collect()
}

impl Inventory{
    fn giveaway(&self,user_preference: Option<ShirtColour>) -> ShirtColour{
        user_preference.unwrap_or_else(||self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColour{
        let mut num_red = 0;
        let mut num_blue = 0;

        for colour in &self.shirts{
            match colour{
                ShirtColour::Red => num_red += 1,
                ShirtColour::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColour::Red
        }else{
            ShirtColour::Blue
        }
    }
}


fn main() {
    let store = Inventory{
        shirts: vec![ShirtColour::Blue, ShirtColour::Red, ShirtColour::Blue],
    };

    let user_pref1 = Some(ShirtColour::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let add_one = |num: i32| -> i32 {num + 1};
    println!("Adding one: {}",add_one(10));

    let some_closure = |vaud: &str| {
        println!("You passed {vaud}")
    };

    some_closure("vaud keith kagong");

    let  list = vec![1,2,3,4];
    println!("Before defining the closure: {:?}", list);

    //let mut borrow_mutably = || list.push(30);
    //println!("In the middle: {:?}", list); causes an error

    thread::spawn(move || print!("From thread: {:?}", list))
    .join()
    .unwrap();

    //borrow_mutably();

    //println!("After defining the closure: {:?}", list);

    let mut list = [
        Rectangle {width: 10, height: 1},
        Rectangle {width: 3, height: 5},
        Rectangle {width: 7, height: 12},
    ];

    list.sort_by_key(|r| r.width);
    print!("{:#?}\n", list);

    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();

    //let v1_add = v1_iter.map(|x| x + 1).collect();

    // for val in v1_iter {
    //     println!("Got: {}", val);
    // }

    let shoes = vec![
        Shoe {size:10,colour:Colour::Blue},
        Shoe {size:10,colour:Colour::Green},
        Shoe {size:20,colour:Colour::Blue},
        Shoe {size:13,colour:Colour::Red},
        Shoe {size:15,colour:Colour::Blue},
    ];

    print!("Shoes of size 10 are {:#?}", shoes_in_size(shoes, 10));

    
}
