extern crate rand;
use rand::Rng;

use std::fs::File;
use std::io::prelude::*;

use std::io;

const ALLOWED_ATTEMPTS:u8 = 5;

struct Letter{
    character: char,
    revealed: bool
}

enum GameProgress {
    InProgress,
    Won,
    Lost
} 

fn main() {
    let mut remaining_attempt = ALLOWED_ATTEMPTS;
    let selected_word = select_word();
    let mut letters = create_letters(&selected_word);

    loop{
        println!("You have {} attempts left! ", remaining_attempt);
        display_progress(&letters);

        println!("please enter a letter");
        let user_char = read_user_input_character();

        //End the game if the input is an *
        if user_char == '*'{
            break;
        }
        
        let mut at_least_one_revealed = false;
        for letter in letters.iter_mut() {
            if letter.character == user_char {
                letter.revealed = true;
                at_least_one_revealed = true;
            }
        }

        if !at_least_one_revealed{
            remaining_attempt -= 1;
        }
        
        match check_progress(remaining_attempt, &letters) {
            GameProgress::InProgress => continue,
            GameProgress::Lost => {
                println!("\nSorry, You lost!");
                break;
            },
            GameProgress::Won => {
                println!("\nCongratulations, You won!. The selected word is {}", selected_word);
                break;
            }
        }
    }
}

fn select_word() -> String {
    let mut file = File::open("words.txt").expect("could not open file!");

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("An error has occured while reading the file!");

    let available_words : Vec<&str> = file_contents.trim().split(',').collect();

    let rand_index = rand::thread_rng().gen_range(0..available_words.len());

    return String::from(available_words[rand_index]);
}

fn create_letters(word: &String) -> Vec<Letter> {
    let mut letters: Vec<Letter> = Vec::new();

    for c in word.chars(){
        letters.push(Letter { character: c, revealed: false });
    }

    return letters;
}

fn display_progress(letters: &Vec<Letter>) {
    let mut display_string = String::from("Progress:");

    for letter in letters{
        display_string.push(' ');

        if letter.revealed{
            display_string.push(letter.character);
        }
        else{
            display_string.push('_');
        }
        display_string.push(' ');
    }
    println!("{}",display_string);
}

fn read_user_input_character() -> char {
    let mut user_input = String::new();

    match io::stdin().read_line(&mut user_input){
        Ok(_) => {
            match user_input.chars().next() {
                Some(c) => { return c; }
                None => {return '*'; }
            }
        }
        Err(_) => { return  '*'; }
    }
}

fn all_letters_revealed(letters : &Vec<Letter>) -> bool {
    for letter in letters{
        if letter.revealed == false {
            return false;
        }
    }
    return true;
}

fn check_progress(remaining_attempt:u8, letters: &Vec<Letter>) -> GameProgress {
    if all_letters_revealed(letters) {
        return GameProgress::Won;
    }
    if remaining_attempt > 0 {
        return GameProgress::InProgress;
    }

    return GameProgress::Lost;
}