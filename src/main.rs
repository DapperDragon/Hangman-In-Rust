use std::fs;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
fn main() {

    // globals
    let mut guesses = 5;
    let hidden_word = "star fruit";
    let mut correct_letters = Vec::new();
    let mut correct_guesses = 0;
    let mut unique_possible_words = Vec::new();
    
    // access fruits.txt
    let mut file = File::open("../fruits.txt").expect("file error");
    let reader = BufReader::new(&mut file);

    // add text to a vector
    let mut lines: Vec<_> = reader
        .lines()
        .map(|l| l.expect("Couldn't read a line"))
        .collect();

    // remove duplicate entries in fruits.txt
    // sort alphabetically
    lines.sort();
    lines.dedup();


    let data = fs::read_to_string("../fruits.txt").expect("Unable to read file");
    println!("{:?}", lines);

    // add unique lines to second vector
    for line in lines {
        unique_possible_words.push(line);

   }

//   let mut split = data.split("\n");
//   println!("{}", split);


    

    // main game loop
    loop{
    
        // check all chars in hidden word
        // if already correcly guessed show them
        // leave spaces as spaces
        // unguessed words replaced with _
        for c in hidden_word.chars() {
        
            if correct_letters.contains(&c)
            { 
                print!("{}", &c);
            }
            
            else if c.is_whitespace()
            {
                print!(" ");
            }
            
            else
            {
                print!("_ ");
            }
        }

        // get player guess
        // if word is entered - only first character is used
        println!("\nGuesses: {}\n", guesses);
        let mut line = String::new();
        println!("Take a guess!\n");
        let _b1 = std::io::stdin().read_line(&mut line).unwrap();
        let ch = line.chars().next().unwrap();
        println!("You guessed: {}", ch);

        // if guess in the hidden word
        if hidden_word.contains(ch) {
            println!("Correct!\n");
            correct_guesses += 1;

            // for letters that appear in word multiple times ie "t" in "test"
            let c = hidden_word.matches(ch).count();
            //println!("{} appears {} times",ch, c);
            
            // add letter to correct_letters
            // as many times as letter appears in word
            let mut i = 0;
            while i < c {
                correct_letters.push(ch);
                i = i + 1;
            }


            // if correct letters is same length as word
            // player must have guessed the word - wins
            if correct_letters.len() == hidden_word.len()-1{
                println!("WINNER!");
                break;
            }
        }
        else{
            println!("No luck");
            guesses = guesses - 1;
        }

        // 0 lives breaks game loop and displays game over
        if guesses ==0 {
            println!("\nGame Over\n");
            break;
        }
    };
    
    // restart game y/n
    let mut restart_line = String::new();
    println!("\nAnother Round?\n");
    let _restart_b1 = std::io::stdin().read_line(&mut restart_line).unwrap();
    let _restart_ch = restart_line.chars();

    // if yes restart main()
    if restart_line.trim() == "y" || restart_line.trim() == "Y" {
        println!("\n...restarting...\n");
        main();
    }

    // if no, exit
    if restart_line.trim() == "n"|| restart_line.trim() =="N"{
        println!("\nThanks for Playing!\n");
        std::process::exit(1);
    }
}