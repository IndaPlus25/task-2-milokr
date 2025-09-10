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


fn main() {
    let n: u32 = 5;
    let number_sequence_string: &str = "5 3 2 1 1";
    println!("{:?}", parse_number_sequence(number_sequence_string))
}


// divides by 2 but rounds up in case of odd input
fn divide_by_2(x: u32) -> u32 {
    if x % 2 == 0 {  return x/2;  }
    else {  return (x+1)/2; }
}

fn parse_number_sequence(numbers_string: &str) -> Vec<u32> {
    let numbers_vector: Vec<u32> = numbers_string
    .split_whitespace()
    .map(|x| x.parse::<u32>().unwrap())
    .collect();
    return numbers_vector;
}