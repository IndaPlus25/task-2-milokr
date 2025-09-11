/*  project sketch:

    input: n
        namn(0) efternamn(0) ... namn(n-1) efternamn(n-1)

    let mut name_vector: Vec[String] = input
        .split_whitespace()
        .map(|x| x.parse::<String>().unwrap())
        .collect(); 
    name_vector = name_vector.remove(0);     // ta bort första entry, alltså siffran (n)

    (is this even the right way of solving this? maybe create tuples and skip the vector step?)

    let mut current_name_pair: u32 = 0;

    type Pair = (String, String);

    while (current_name_pair > name_vector.len();) == 0 {
        let (ny pair) = (name_vector[current_name_pair as usize], name_vector[current_name_pair+1 as usize])        // problem: how do i make a new pair for every loop? maybe this isn't the right way to solve this either.
        current_name_pair += 2
    }

*/


fn main() {
    
}
