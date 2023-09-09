extern crate rand;
use rand::Rng;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let selected_word = select_word();
    println!("Selected word {}", selected_word);
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
