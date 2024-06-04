
struct Point{
    x: i32,
    y: i32,
}

enum Message{
    Hello {id: i32}
}

fn main() {
    let favourite_colour: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8,_> = "34".parse();

    if let Some(color) = favourite_colour {
        println!("Using your favourite colur, {color} as the background");
    }else if is_tuesday{
        println!("Tuesday is green day");
    }else if let Ok(age) = age{
        if age > 30 {
            println!("Using purple as the background colour");
        }else{
            println!("Using orange as the background colour");
        }
    }else{
        println!("Using blue as the background colour");
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop(){
        print!("{}", top);
    }

    let point = (3,5);
    print_coordinates(&point);

    let p = Point{ x: 0, y: 7};

    //let Point { x: a, y : b} = p;
    let Point { x, y } = p;

    assert_eq!(0,x);
    assert_eq!(7,y);

    match p {
        Point { x: 0, y } => println!("On the y-axis at {y}"),
        Point { x, y:0 } => println!("On the x-axis at {x}"),
        Point { x:_,y:_ } => print!("On neither axis")
    };

    let origin = Point{x: 0, y: 0};

    match origin{
        Point { x, .. } => println!("x is {}", x)
    };

    let numbers = (2,3,4,5,6,7,8);

    match numbers{
        (first,..,last) => println!("First and last: {first}, {last}")
    };

    let num = Some(4);

    // the issue with using a match guard is the compile does not try to check for exhaustiveness
    match num {
        Some(x) if x % 2 == 0 => println!("Number is even"),
        Some(_x) => println!("Number is odd"),
        None => print!("Please provide a number")
    };

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => println!("Found another id in another range"),
        Message::Hello { id } => println!("Found some other id: {}", id)
    };

}

fn print_coordinates(&(x,y): &(i32,i32)){
    println!("\nCurrent location: ({}, {})", x, y);
}
