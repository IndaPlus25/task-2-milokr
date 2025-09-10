    /*notes:
    input:
    heltal n som anger tal i sekvens
    n heltal separerade med space

    output:
    n/2 största tal ((n+1)/2 om udda)

    (JÄTTE ROUGH) sketch:

    int antalTal(n) {
    if (n % 2) {    return n/2  }
    else {  return (n+1)/2   }
    }


    let mut v = listaAvHeltal.parse()
    v = v.sort()

    let mut summa = 0

    for i in range((v.len() - n), v.len()) {
        summa = summa + v[i]
    }

    print(summa)
    
     */

use prompted::input;

fn main() {
    let number_sequence_string: String = input!("Input list of positive integers (with spaces separating each one): ");
    let mut n: u32 = input!("Please input the amount of positive integers in the list you just wrote: ").parse::<u32>().unwrap();
    let mut numbers_vector: Vec<u32> = parse_number_sequence(number_sequence_string);

    println!("{:?}", numbers_vector); // for testing

    numbers_vector.sort();

    println!("{:?}", numbers_vector); // for testing
    println!("{}", n.to_string()); // for testing

    n = divide_by_2(n);

    println!("{}", n.to_string()); // for testing

    let result: u32 = sum_biggest_half(numbers_vector, n);

    println!("{}", result.to_string()); // for testing
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
    let vector_length: u32 = v.len().try_into().unwrap();   
    for i in (vector_length-n)..vector_length {
        sum += v[i as usize];               //  "as usize" comes from this comment https://www.reddit.com/r/rust/comments/385em5/comment/crskat9/
    }
    return sum
}