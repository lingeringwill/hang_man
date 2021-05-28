use::std::fs::File;
use std::io::prelude::*;
use rand::Rng;


const ALLOWED_ATTEMPTS: u8 = 5;
fn main(){
  let mut turns_left:u8 = 5;
  let selected_word = select_word();
  let  letters = create_letters(&selected_word);
  
  
    display_progress(&letters);
  

 
}

struct Letter {
  character: char,
  revealed: bool
}

fn select_word() -> String {

/*open file*/
let mut file = File::open("words.txt").expect("could not open file");

/*load file contents*/
let mut file_contents = String::new();
file.read_to_string(&mut file_contents)
.expect("an error has occured");
/*Get individual words*/

let availible_words:Vec<&str> = file_contents.trim().split(',').collect();

/*generate random index*/
let random_index = rand::thread_rng().gen_range(0, availible_words.len());

 return String::from(availible_words[random_index]);
}

fn create_letters(word: &String) -> Vec<Letter>{
  /*creates empty vector of Letter struct*/
  let mut letters: Vec<Letter> = Vec::new();

  /*wrap each character in a Letter struct*/
  for c in word.chars(){
    letters.push(Letter{
      character: c,
      revealed: false
    });
  }
  return letters;
}

fn display_progress(letters: &Vec<Letter>){
  let mut display_string = String::from("Progress:");
  //Display appropriate character (letter or _) for each letter
  for letter in letters {
   display_string.push(' ');

   if letter.revealed{
   display_string.push(letter.character);
    }else{
      display_string.push('_');
    }

    display_string.push(' ');

  }
  println!("{}",display_string);
}