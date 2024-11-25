/*The main purpose of this program is for the user to be able to convert english word to pig latin */

//All import goes here
use std::io;

//All functions here
fn is_vowel(c: char) -> bool{
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}

fn main() {
    //Users input here
    println!("PIG LATIN CONVERTER");
    println!();
    println!("Enter the word to be converted: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Couldn't read input");
    let input = input.trim().to_lowercase();

    //create an empty vector
    let mut input_collection = Vec::new();

    //using a for loop let's push them inside the vector
    for letters in input.chars(){
        input_collection.push(letters);
    }

    let mut word_added = vec!['-', 'h', 'a', 'y'];
    let first_letter = input_collection[0];
    if let Some(first) = input_collection.get(0){
        if is_vowel(*first) {
            input_collection.append(&mut word_added);
            let result: String = input_collection.into_iter().collect();
            println!("{:?}", result);
        }
        else {
            input_collection.remove(0);
            input_collection.push('-');
            input_collection.push(first_letter);
            input_collection.push('a');
            input_collection.push('y');
            let result: String = input_collection.into_iter().collect();
            println!("{:?}", result);
            
        }
    }
    
}
