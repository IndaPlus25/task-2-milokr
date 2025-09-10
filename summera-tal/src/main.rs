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
    let n: u32 = input!("Please input the amount of positive integers in the list you just wrote: ").parse::<u32>().unwrap();
    println!("{:?}", parse_number_sequence(number_sequence_string)); // for testing
    println!("{}", n.to_string()) // for testing
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