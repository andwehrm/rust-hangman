use std::io;
use std::io::{BufRead, BufReader};
use std::fs::File;
use rand::{thread_rng, Rng};
use crossterm::terminal;

fn random_word() -> String
{
    let mut rng = thread_rng();
    let line_num: usize = rng.gen_range(0, 1000);

    let reader = BufReader::new(File::open("words.txt").expect("Cannot open words.txt"));

    for line in reader.lines().nth(line_num) {
        for word in line.unwrap().split_whitespace() {
            return String::from(word);
        }
    }

    return String::from("Error");
}

fn custom_word() -> String
{
    println!("Please Enter the Word you would like to use (Max 30 Characters)");
    let mut input = String::with_capacity(30);
    io::stdin().read_line(&mut input).expect("Error reading input");
    return input;
}

fn game_loop(word:String)
{
    let screen = crossterm::Screen::default();
    let term = terminal::terminal(&screen);
    let is_running : bool = true;
    let mut letters_missing: bool = true;   
    let mut lives:i32 = 10;
    let mut found_letters = String::new();
    let mut guessed_letters = String::new();
    term.clear(terminal::ClearType::All);
    while is_running {
        let mut input = String::new();

        if lives <= 0 {
            term.clear(terminal::ClearType::All);
            println!("Sorry! You're out of lives. You have lost the game.");
            break;
        }
        if !letters_missing {
            term.clear(terminal::ClearType::All);
            println!("You have Won! You correctly guessed the word {}",word);
            break;
        }

        println!("");
        letters_missing = false;
        for c in word.chars() {
            if found_letters.contains(c) {
                print!(" {} ",c);
            }
            else {
                print!(" _ ");
                letters_missing = true;
            }
        }
        println!("\n\nTake a Guess! You have {} Lives left", lives);

        io::stdin().read_line(&mut input).expect("Error reading input");
        let guess = input.as_str().trim();
        term.clear(terminal::ClearType::All);
        if word.contains(guess) {
            if guessed_letters.contains(guess) {
                println!("Correct! But you have already guessed this Letter!")
            }
            else {
                println!("Correct!");
                found_letters += guess;
            }
           
        }
        else {
            if guessed_letters.contains(guess){
                println!("Wrong! You have already guessed this letter. You dont lose one life.");
            }
            else {
                println!("Wrong! You lose one life!");
                lives -= 1;
            }
        }
       
        guessed_letters += guess;
        
    }
}

fn main() {
    let mut input = String::with_capacity(1);
    let mut word = String::with_capacity(30);

    println!("Welcome to Hangman!\n1. Use Random Word\n2. Enter Custom Word");
    io::stdin().read_line(&mut input).expect("Error reading input");

    match input.as_str().trim() {
        "1" | "1." => word = random_word(),
        "2" | "2." => word = custom_word(),
        _ => println!("Unknown Input"),
    }

    println!("Game will Start now!");

    game_loop(String::from(word.to_lowercase().as_str().trim()));

    println!("Press any key to close...");
    io::stdin().read_line(&mut input).expect("Error reading input");
}
