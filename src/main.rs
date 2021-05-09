use::std::fs::File;
use std::io::prelude::*;
use rand::Rng;

struct Letter {
  character: char,
  revealed: bool

}
fn main() {
   let selected_word = selected_word();

    let mut letters = create_letters(&selected_word);
}

fn selected_word() -> String{
    //open file
let mut file = File::open("words.txt").expect("could not open file");

//set file contents to a string
let mut file_contents = String::new();
file.read_to_string(&mut file_contents)
.expect("failure to write data to string");

//divides each word by a comma, removes white space and then collects them as a vector of strings
let avalible_words:Vec<&str> = file_contents.trim().split(',').collect();

//output a random number
let random_index = rand::thread_rng().gen_range(0,avalible_words.len());

return String::from(avalible_words[random_index])

}

fn create_letters(word: &String) -> Vec<Letter>{
    let mut letters: Vec<Letter> = Vec::new();

    /*takes in in string from selected word (as word)
    then iterates through string and pushes a Letter struct
   to the back of letters */
    for c in word.chars(){
        letters.push(Letter{
           character: c,
           revealed: false
        });
    }
    return letters
}