extern crate rand;
use rand::Rng;

use std::fs::File;
use std::io::prelude::*;

struct Letter{
    character: char,
    revealed: bool
}

fn main() {
    let selected_word = select_word();
    let mut letters = create_letters(&selected_word);

    display_progress(&letters);
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