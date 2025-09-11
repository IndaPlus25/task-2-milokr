/* new sketch:

    let amount_of_names: usize = input
        .split_whitespace()
        .next()
        .unwrap()
        .parse()                            
        .unwrap();

    let input_vector: Vec<String> = input
        .split_whitespace()
        .skip(1)
        .map(|x| x.to_string())
        .collect();

    type Pair = (String, String);

    let mut pair_vector: Vec<Pair> = Vec::new();

    for i in 0..amount_of_names {
        let firstname = input_vector[i].clone();
        let lastname = input_vector[i + amount_of_names].clone();           // chatgpt helped with formatting this part as i was more used to python lists and not at all used to ownership/borrowing (.clone())
        pair_vector.push((firstname, lastname));
    }

*/

use std::io::{self, Read};

type Pair = (String, String);

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)         // chatgpt helped me make this take multiple lines of input
        .unwrap();

    let amount_of_names: usize = input
        .split_whitespace()
        .next()
        .unwrap()
        .parse()                            
        .unwrap();

    let input_vector: Vec<String> = input
        .split_whitespace()
        .skip(1)
        .map(|x| x.to_string())
        .collect();

    let mut pair_vector: Vec<Pair> = Vec::new();

    for i in 0..amount_of_names {
        let firstname = input_vector[i].clone();
        let lastname = input_vector[i + amount_of_names].clone();           // chatgpt helped with formatting this part as i was more used to python lists and not at all used to ownership/borrowing (.clone())
        pair_vector.push((firstname, lastname));
    }

    let current_pair: usize = 0;
    let names_to_remove: Vec<usize> = compare_pairs(current_pair, &pair_vector, amount_of_names);



    for &i in names_to_remove.iter().rev() {        // got help from chatgpt with reversing this (to avoid issues with removing pairs from the vector affecting the index)
        pair_vector.remove(i);
    }
    
    println!("{:?}", pair_vector.len());
}


fn compare_pairs(current_pair: usize, list_of_pairs: &Vec<Pair>, vector_length: usize) -> Vec<usize> {
    let mut names_to_remove: Vec<usize> = Vec::new();
    for i in (current_pair+1)..vector_length {
        if list_of_pairs[current_pair].0 == list_of_pairs[i].0 &&
        list_of_pairs[current_pair].1 == list_of_pairs[i].1 {
            names_to_remove.push(i);
        }
    }
    if current_pair < vector_length {
        names_to_remove.extend(compare_pairs(current_pair + 1, list_of_pairs, vector_length));
    }
    return names_to_remove;
}