#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn square(size: u32) -> Self{
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self,other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle::square(20);

    // dbg!(&rect);

    println!(
        "Rect is {:#?}",
        rect
    );

    println!(
        "The area of the rectangle is {} square pixels",
        rect.area()
    );

    println!("Can Rect1 hold Rect2 {}", rect.can_hold(&rect2))

}

