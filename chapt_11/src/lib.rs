pub fn add(left: usize, right: usize) -> usize {
    left + right
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1. Got {}", value);
        }else if value > 100 {
            panic!("Guess value must be less than or equal to 100. Got {}", value);
        }

        println!("Life is amazing");
        Guess {value}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller(){
        let larger = Rectangle {
            width: 10,
            height: 12,
        };

        let smaller =  Rectangle {
            width: 3,
            height: 4,
        };

        assert!(larger.can_hold(&smaller),
            "larger rectangle could not hold the smaller rectangle")
    }

    #[test]
    fn smaller_cannot_hold_larger(){
        let smaller = Rectangle {
            width: 2,
            height: 2,
        };

        let larger = Rectangle {
            width: 10,
            height: 10,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100(){
        Guess::new(200);
    }
}
