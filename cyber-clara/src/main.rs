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

    type Pair = (String, String);

    let mut pair_vector: Vec<Pair> = Vec::new();

    for i in 0..amount_of_names {
        let firstname = input_vector[i].clone();
        let lastname = input_vector[i + amount_of_names].clone();           // chatgpt helped with formatting this part as i was more used to python lists and not at all used to ownership/borrowing (.clone())
        pair_vector.push((firstname, lastname));
    }

    println!("{:?}", pair_vector);
}
