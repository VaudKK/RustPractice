use std::{fs::{self, File}, io::{self, ErrorKind, Read}};

fn main() {
    let text_file_result = File::open("hello.txt");

    let text_file = match text_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file {:?}",e)
            },
            other_error => panic!("Problem opening the file {:?}", other_error)
        }
    };

    println!("Text file {:?}", text_file);

    //another way

    let another_file = File::open("hello.txt").unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound{
            File::create("hello.txt").unwrap_or_else( |error| {
                panic!("Error creating the file: {:?}",error);
            })
        }else{
            panic!("Problem opening the file {:?}", error);
        }
    });

    println!("Another Text file {:?}", another_file);

    let name_result = read_something_from_file().unwrap_or_else(|error|{
        panic!("Could not read name: {:?}", error)
    });

    println!("The name is: {name_result}");

}


fn read_something_from_file() -> Result<String,io::Error>{
    let file_result = File::open("hello.txt");

    let mut file = match file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut content = String::new();

    match file.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(e) => Err(e)
    }
}

fn read_something_from_file_clean() -> Result<String,io::Error> {
    let mut content = String::new();

    File::open("hello.txt")?.read_to_string(&mut content)?;
    
    Ok(content)
}

fn read_something_from_file_shorter() -> Result<String,io::Error>{
    fs::read_to_string("hello.txt")
}