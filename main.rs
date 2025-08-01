use rand::Rng;

/// Generates n random probabilities that sum to 1.0
/// Uses the method of generating random numbers and normalizing by their sum.
fn generate_random_probabilities(n: usize) -> Vec<f64> {
    if n == 0 {
        return vec![];
    }
    
    let mut rng = rand::thread_rng();
    
    // Generate n random numbers from uniform distribution [0, 1)
    let mut random_numbers: Vec<f64> = (0..n)
        .map(|_| rng.gen_range(0.0..1.0))
        .collect();
    
    // Calculate the sum of all random numbers
    let sum: f64 = random_numbers.iter().sum();
    
    // Normalize each number by dividing by the sum
    // This ensures all probabilities sum to 1.0
    random_numbers.iter_mut().for_each(|x| *x /= sum);
    
    random_numbers
}

/// Alternative implementation using exponential distribution
/// This is mathematically equivalent to the Dirichlet distribution with all alpha=1
fn generate_random_probabilities_exp(n: usize) -> Vec<f64> {
    if n == 0 {
        return vec![];
    }
    
    let mut rng = rand::thread_rng();
    
    // Generate n random numbers from exponential distribution
    // We simulate exponential by using -ln(uniform_random)
    let mut random_numbers: Vec<f64> = (0..n)
        .map(|_| -(rng.gen_range(0.0..1.0) as f64).ln())
        .collect();
    
    // Calculate the sum and normalize
    let sum: f64 = random_numbers.iter().sum();
    random_numbers.iter_mut().for_each(|x| *x /= sum);
    
    random_numbers
}

#[allow(dead_code)]
fn test_probabilities(n: usize) {
        println!("Generating {} random probabilities that sum to 1.0", n);
    println!();
    
    // Method 1: Using uniform distribution
    println!("Method 1 (Uniform distribution):");
    let probabilities1 = generate_random_probabilities(n);
    for (i, prob) in probabilities1.iter().enumerate() {
        println!("  p{} = {:.6}", i + 1, prob);
    }
    let sum1: f64 = probabilities1.iter().sum();
    println!("  Sum = {:.10}", sum1);
    println!();
    
    // Method 2: Using exponential distribution
    println!("Method 2 (Exponential distribution):");
    let probabilities2 = generate_random_probabilities_exp(n);
    for (i, prob) in probabilities2.iter().enumerate() {
        println!("  p{} = {:.6}", i + 1, prob);
    }
    let sum2: f64 = probabilities2.iter().sum();
    println!("  Sum = {:.10}", sum2);
    println!();
    
    // Demonstrate with different sizes
    println!("Examples with different sizes:");
    for size in [3, 7, 10] {
        let probs = generate_random_probabilities(size);
        let sum: f64 = probs.iter().sum();
        println!("  n={}: sum={:.10}, values={:?}", size, sum, 
                 probs.iter().map(|x| format!("{:.4}", x)).collect::<Vec<_>>());
    }
}

#[derive(Clone, Debug)]
struct Id{
    id:u64,
    rank:f64,
}

impl Id{
    fn new(id:u64, rank:f64) -> Self {
        Self { id, rank }
    }
}

#[derive(Clone, Debug)]
struct StringId{
    id:u64,
    rank:String,
}

impl StringId{
    fn new(id:u64, rank:String) -> Self {
        Self { id, rank }
    }
}


fn page_rank(markov_chain: Vec<Vec<f64>>, ids: Vec<Id>) -> Vec<Id> {
    let threshold: f64 = 0.000001; // change this to the threshold for convergence else you will get same ranks
    let n = ids.len();
    // transpose the markov chain
    let mut markov_chain_transpose: Vec<Vec<f64>> = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            markov_chain_transpose[j][i] = markov_chain[i][j] as f64;
        }
        // println!("Markov Chain Transpose {i}: {:?}", markov_chain_transpose[i]);
    }
    // println!("Markov Chain Transpose: {:?}", markov_chain_transpose);
    let mut page_rank: Vec<Id> = vec![];
    for i in 0..n {
        page_rank.push(Id::new(ids[i].id, 1.0 / n as f64));
    }

    let mut page_rank_new: Vec<Id> = ids.clone();
    // let mut iterations = 0;
    loop {
        for i in 0..n {
            for j in 0..n {
                page_rank_new[i].rank += markov_chain_transpose[i][j] * page_rank[j].rank;
            }
        }
        // println!("Page Rank New: {:?}", page_rank_new);
        if page_rank.iter().zip(page_rank_new.iter()).all(|(a, b)| (a.rank - b.rank).abs() < threshold) {
            break;
        }
        page_rank = page_rank_new.clone();
        page_rank_new = ids.clone(); // create a vector of with the same ids and 0 ranks for each id
        // iterations += 1;
        // if iterations > 10 {
        //     break;
        // }
    }

    // page_rank.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());
    // println!("Page Rank: {:?}", page_rank);
    // page_rank
    // for id in page_rank.iter().take(10) {
    //     println!("Id: {:?}, Rank: {:?}", id.id, id.rank);
    // }
    page_rank
}

fn main() {
    let n = 5000; // change this to the number of ids and threshold for convergence else you will get same ranks
    

    let mut ids: Vec<Id> = vec![];
    let mut string_ids: Vec<StringId> = vec![];
    for i in 0..n {
        let id = Id::new(i as u64, 0.0);
        ids.push(id);
        // string_ids.push(StringId::new(i as u64, format!("{:.10}", 0.0)));
    }
    // println!("Ids: {:?}", ids);

    //generate a nxn markov chain matrix from the probabilities
    let mut markov_chain = vec![vec![0.0; n]; n];
    for i in 0..n {
        let probs = generate_random_probabilities(n);
        for j in 0..n {
            markov_chain[i][j] = probs[j]; 
            //(probs[j] * 1000.0).round() / 1000.0; // round to 3 decimal places
        }
    }

    // damping probabilities to a/N, a =.15, 1-a =.85   
    let a: f64 = 0.15;
    for i in 0..n {
        for j in 0..n {
            markov_chain[i][j] = markov_chain[i][j] * (1.0 - a) + (a / n as f64);
        }
        // println!("Markov Matrix Chain {i}: {:?}", markov_chain[i]);
    }
    // println!("Page Rank: {:?}", page_rank(markov_chain, n));

    let page_rank = page_rank(markov_chain, ids);
    for id in page_rank.iter(){
        string_ids.push(StringId::new(id.id, format!("{:.70}", id.rank))); // max of 64 bits for the rank
    }
    string_ids.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());
    for id in string_ids.iter().take(20) {
        println!("Id: {:?}, Rank: {:?}", id.id, id.rank);
    }
}

