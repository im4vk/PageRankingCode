use std::collections::HashMap;
use std::fs;
use rand::Rng;

struct NGram {
    transitions: HashMap<String, Vec<String>>,
}

impl NGram {
    fn new() -> Self {
        Self { transitions: HashMap::new() }
    }

    fn add_transition(&mut self, from: &str, to: &str) {
        self.transitions
            .entry(from.to_string())
            .or_insert_with(Vec::new)
            .push(to.to_string());
    }

    fn load_from_file(&mut self, filename: &str) -> Result<(), std::io::Error> {
        let content = fs::read_to_string(filename)?;
        self.process_text(&content);
        Ok(())
    }

    fn process_text(&mut self, text: &str) {
        // Split text into words and clean them
        let words: Vec<String> = text
            .split_whitespace()
            .map(|word| {
                word.chars()
                    .filter(|c| c.is_alphabetic() || c.is_whitespace())
                    .collect::<String>()
                    .to_lowercase()
            })
            .filter(|word| !word.is_empty())
            .collect();

        // Create transitions between consecutive words
        for window in words.windows(2) {
            if let [from, to] = window {
                self.add_transition(from, to);
            }
        }
    }

    fn generate_text(&self, start_word: &str, length: usize) -> String {
        let mut result = vec![start_word.to_string()];
        let mut current_word = start_word;
        let mut rng = rand::thread_rng();

        for _ in 1..length {
            if let Some(next_words) = self.transitions.get(current_word) {
                if !next_words.is_empty() {
                    let index = rng.gen_range(0..next_words.len());
                    let next_word = &next_words[index];
                    result.push(next_word.clone());
                    current_word = next_word;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        result.join(" ")
    }

    fn get_random_start_word(&self) -> Option<String> {
        if self.transitions.is_empty() {
            return None;
        }
        
        let mut rng = rand::thread_rng();
        let keys: Vec<_> = self.transitions.keys().collect();
        let index = rng.gen_range(0..keys.len());
        Some(keys[index].clone())
    }

    fn print_stats(&self) {
        // println!("Markov Chain Statistics:");
        println!("\n- Total unique words: {}", self.transitions.len());
        // println!("- Total transitions: {}", 
        //          self.transitions.values().map(|v| v.len()).sum::<usize>());
        
        // Show some example transitions
        // println!("\nSample transitions:");
        // for (word, transitions) in self.transitions.iter().take(5) {
        //     println!("  '{}' can be followed by: {:?}", word, 
        //              &transitions[..transitions.len().min(3)]);
        // }
    }
}

fn main() {
    // println!("Loading Markov chain from file...");
    
    let mut ngram = NGram::new();
    
    // Load the text file
    match ngram.load_from_file("large_training_data.txt") {
        Ok(()) => {
            // println!("✅ Successfully loaded markov_test_data.txt");
            ngram.print_stats();
            
            // Generate some sample text
            println!("\n{}", "=".repeat(50));
            println!("Generated text samples:");
            println!("{}", "=".repeat(50));
            
            for i in 1..=3 {
                if let Some(start_word) = ngram.get_random_start_word() {
                    println!("\nSample {}:", i);
                    println!("{}", ngram.generate_text(&start_word, 30)); // 30 is the length of the generated text
                }
            }
            
            // Try specific starting words
            // println!("\n{}", "=".repeat(50));
            // println!("Targeted generations:");
            // println!("{}", "=".repeat(50));
            
            // let start_words = vec!["the", "life", "science", "human", "future"];
            // for start_word in start_words {
            //     if ngram.transitions.contains_key(start_word) {
            //         println!("\nStarting with '{}':", start_word);
            //         println!("{}", ngram.generate_text(start_word, 25));
            //     }
            // }
            
        },
        Err(e) => {
            eprintln!("❌ Error loading file: {}", e);
            eprintln!("Make sure file exists in the current directory");
        }
    }
}

