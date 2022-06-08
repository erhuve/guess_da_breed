extern crate reqwest;
extern crate json;

use rand::prelude::*;
use std::io;
use std::error::Error;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Breeds {
    message: HashMap<String, Vec<String>>,
    status: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct BreedImage {
    message: String,
    status: String,
}

fn main() {
    // Start da main program
    println!("Ｗｅｌｃｏｍｅ ｔｏ ＧＵＥＳＳ ＤＡ ＢＲＥＥＤ");
    println!("You'll be presented with a shuffled breed name, and your goal is to guess it in as few tries as possible");
    println!("If you'd like a full list of breeds, type \'breedlist\'");
    println!("Once you get it right, you'll get a reward! So keep on guessin'");
    println!("And lastly, to quit, type \'quit\'");

    // Fetch breeds list
    let breed_list = fetch_breeds().unwrap();

    let mut playing = true;
    // Outer loop to play multiple rounds
    while playing {
        let rand_num = rand::thread_rng().gen_range(0..breed_list.len());
        let curr_breed = &breed_list[rand_num];
        println!("Dendrobyte prints the breed here, so I will too: {} {}", curr_breed, shuffle(curr_breed));

        // Get a random image of the breed for later
        let curr_image_url = fetch_breed_image(curr_breed).unwrap();

        // Initial breed information
        println!("Your scrambled breed: {}", shuffle(curr_breed).to_uppercase());

        let mut guess_counter = 1;
        let mut is_guessed = false;
        //Loop through various guesses
        loop {
            if is_guessed {
                println!("Type \'continue\' to play again, or \'quit\' to quit!");
            } else {
                println!("Guess {}:", guess_counter);
            }
            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line.");
            
            let guess = guess.trim();

            match guess.to_lowercase().as_str() {
                "continue" => if is_guessed {
                    playing = true;
                    // dwai
                    is_guessed = false;
                    break;
                  } else {
                      println!("You haven't gotten it yet lol");
                    },
                "breedlist" => println!("Here's yo list o' breeds: {:?}", breed_list),
                "quit" => {
                    println!("Thanks for playing! The breed was {}", curr_breed);
                    playing = false;
                    break;
                  },
                a if a == curr_breed => {
                    println!("CONGRAUTLIATIOTALANTS!!!!! THE BREED WAS {} BUT YOU KNEW THAT", curr_breed);
                    println!("Enjoy an image... on us (:");
                    println!("Go here: {}", curr_image_url);
                    is_guessed = true;
                  },
                _ => {
                    println!("Nope.");
                    if guess_counter % 4 == 0 {
                        println!("Perhaps a reshuffle might help? -- RESHUFFLED: {}", shuffle(curr_breed));
                    }
                    guess_counter += 1;
                }
                
            }
        }
        
    }
}

// Takes a string and shuffles the letters the way Dendobyte did it
fn shuffle(breed: &str) -> String {
    let mut shuffled = String::from("");
    let mut unshuffled = String::from(breed);
    while unshuffled.len() > 0 {
        let curr_length = unshuffled.len() - 1;
        let rand_num = rand::thread_rng().gen_range(0..curr_length + 1);
        shuffled.push(unshuffled.as_bytes()[rand_num] as char);
        unshuffled.remove(rand_num);
    }
    shuffled
}

#[tokio::main]
async fn fetch_breeds() -> Result<Vec<String>, Box<dyn Error>> {
    let body = reqwest::get("https://dog.ceo/api/breeds/list/all")
      .await?
      .json::<Breeds>()
      .await?;

    Ok(body.message.into_keys().collect())
}

#[tokio::main]
async fn fetch_breed_image(breed: &str) -> Result<String, Box<dyn Error>> {
    let get_breed_url = format!("https://dog.ceo/api/breed/{}/images/random", breed);
    let body = reqwest::get(get_breed_url)
      .await?
      .json::<BreedImage>()
      .await?;
    
    Ok(body.message)
}

