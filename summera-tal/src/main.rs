use std::io;

fn main() {
    //  old input macro from external cargo, did not work in kattis so changed to standard std::io
    // let number_sequence_string: String = input!("Input list of positive integers (with spaces separating each one): ");      
    // let mut n: u32 = input!("Please input the amount of positive integers in the list you just wrote: ").parse::<u32>().unwrap();

    let mut n_input = String::new();
    // println!("Please input the amount of integers in the list: ");
    io::stdin()
        .read_line(&mut n_input)
        .expect("Failed to read line");

    let mut n: u32 = n_input
        .trim()
        .parse::<u32>()
        .unwrap();

    let mut number_sequence_string = String::new();
    // println!("Input list of positive integers (with spaces separating each one): ");
    io::stdin()
        .read_line(&mut number_sequence_string)
        .expect("Failed to read line");

    let mut numbers_vector: Vec<u32> = parse_number_sequence(number_sequence_string);
    numbers_vector.sort();
    n = divide_by_2(n);

    let result: u32 = sum_biggest_half(numbers_vector, n);

    println!("{}", result.to_string());
}


// divides by 2 but rounds up in case of odd input
fn divide_by_2(x: u32) -> u32 {
    if x % 2 == 0 {  return x/2;  }
    else {  return (x+1)/2; }
}

fn parse_number_sequence(numbers_string: String) -> Vec<u32> {
    let numbers_vector: Vec<u32> = numbers_string
    .split_whitespace()
    .map(|x| x.parse::<u32>().unwrap())         // inspiration: https://stackoverflow.com/a/34090825 
    .collect();                                                                 //          ==||==
    return numbers_vector;
}

fn sum_biggest_half(v: Vec<u32>, n: u32) -> u32 {
    let mut sum: u32 = 0;
    let vector_length: u32 = v
        .len()
        .try_into()
        .unwrap();   
    for i in (vector_length-n)..vector_length {
        sum += v[i as usize];               //  "as usize" comes from this comment https://www.reddit.com/r/rust/comments/385em5/comment/crskat9/
    }
    return sum
}