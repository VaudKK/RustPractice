use crate::garden::vegetables::Asparagus as MyPlant;

use std::io::{self,Write};

pub mod garden;

use crate::garden::vegetables::some_inner_module;

fn main() {
    println!("Hello garden {:#?}", MyPlant{});
    let sc = some_inner_module::Species{
        name: String::from("SomeHotName"),
        latin_name: String::from("ElLatinaDeLatin")
    };

    println!("Another hot stuff {:#?}",sc)
}
