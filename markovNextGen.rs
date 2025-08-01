use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use rand::seq::IteratorRandom;

fn main() {
    println!("Loading Markov chain from file...");

    let mut ngrams = HashMap::<String, Vec<char>>::new();
    let order = 3;
    
    let file = File::open("large_training_data.txt");
    match file {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut text = String::new();
            reader.read_to_string(&mut text).expect("Failed to read file");
            println!("✅ Successfully loaded large_training_data.txt");
            println!("File contains {} characters", text.len());


            for i in 0..text.len() - order {
                let ngram = text[i..i + order].to_string();
                let next_char = text.chars().nth(i + order).unwrap();
                ngrams.entry(ngram).or_insert(Vec::new()).push(next_char);
            }

            // println!("N-grams: {:?}", ngrams);


            let mut rng = rand::thread_rng();
            // select a random ngram from the ngrams
            let mut current_ngram = ngrams.keys().choose(&mut rng).unwrap().to_string();
            // let mut current_ngram = random_ngram.clone();
            print!("Random ngram: {}", current_ngram);

            // generate 100 characters
            // let mut current_ngram = String::from("The");
            for _ in 0..100 {
                let mut rng = rand::thread_rng();
                let next_char = ngrams[&current_ngram].iter().choose(&mut rng).unwrap();
                // current_ngram.push(*next_char);
                current_ngram = current_ngram[1..].to_string() + next_char.to_string().as_str();
                print!("{}", next_char);
            }
            println!();
        },
        Err(e) => {
            eprintln!("❌ Error opening file: {}", e);
            eprintln!("Make sure large_training_data.txt exists in the current directory");
        }
    }
}

