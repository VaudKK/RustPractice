use std::{collections::HashMap, io};


fn main() {
    //median and mode
    let mut test_vec = vec![10,20,304,5,43,23,64,64,234,54,6,2,5,4,64];
    println!("The media is {}",median(&mut test_vec));
    println!("The mode is {}", mode(&test_vec));

    //pig latin
    let mut test_sentence = String::from("first apple is dope");
    print!("Translated to pig latin {}", pig_latin(&mut test_sentence));

    //mini hr
    mini_hr();
}

fn median(numbers: &mut Vec<i32>) -> i32{
    let len = numbers.len();
    numbers.sort();
    numbers[len/2]
}

fn mode(numbers: &Vec<i32>) -> i32 {
    let mut max_key: i32 = 0;
    let mut max_value: i32 = 0;

    let mut modes = HashMap::new();
    for i in numbers{
        let count = modes.entry(i).or_insert(0);
        *count += 1;

        if *count > max_value {
            max_value = *count;
            max_key = *i;
        }
    }


    return max_key;
}

fn pig_latin(sentence: &mut String) -> String{

    let vowels = "aeiou".as_bytes();
    let mut new_sentence = String::from("");

    for word in sentence.split_whitespace(){
        let word_vec = Vec::from(word);
        
        if vowels.contains(&word_vec[0]){
            new_sentence.push_str(format!("{}-hay",&word[..]).as_str());
        }else{
            new_sentence.push_str(format!("{}-{}ay",&word[1..],&word[..1]).as_str());
        }
        new_sentence.push(' ');
    }

    new_sentence
}

fn mini_hr(){
    let mut command = String::new();
    let mut tokens: Vec<String> = Vec::new();

    let mut sales_employees: Vec<String> = Vec::new();
    let mut engineering_employees: Vec<String> = Vec::new();

    println!("\n\nWelcome to mini hr\n\nSample command(Add [Username] to [department (Sales/Engineering)])\nType show to display list");
    get_user_input(&mut command);

    let mut i = 0;

    for token in command.split_whitespace(){
        tokens.insert(i, String::from(token));
        i+=1;
    }

    if tokens[3].eq("Sales") {
        sales_employees.push(String::from(&tokens[1]));
    }else if tokens[3].eq("Engineering") {
        engineering_employees.push(String::from(&tokens[1]));
    }else{
        println!("Invalid department")
    }


    //print
    sales_employees.sort();
    engineering_employees.sort();
    println!("Sales employees:\n{:?}",sales_employees);
    println!("Engineering employees:\n{:?}",engineering_employees);
    
}

fn get_user_input(field: &mut String) {
    io::stdin().read_line(field).expect("Failed to read input");
}

