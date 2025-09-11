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

    loop in some kind of way to compare the pairs. how tho :(


    function to compare the pairs:          // while thinking i realised it might be best to either: just use a vector with all the names, or to make a vector storing all the pairs
                                            //  ill be writing this function as if we have a vector filled with all our pairs

    list_of_pairs: Vec<Pair> = [pair_1, pair_2 .... pair_n]
    let mut current_pair: u32 = 0
    let mut matching_pairs: u32 = 0

    func compare_pairs() {
        for i in 1..(amount_of_pairs - current_pair_number + 1) {
            if list_of_pairs[current_pair_number].1 == list_of_pairs[current_pair_number + i].1 { matching pairs += 1; }
            elif list_of_pairs[current_pair_number].1 == list_of_pairs[current_pair_number + i].2 { matching pairs += 1; }
            elif list_of_pairs[current_pair_number].2 == list_of_pairs[current_pair_number + i].1 { matching pairs += 1; }
            elif list_of_pairs[current_pair_number].2 == list_of_pairs[current_pair_number + i].2 { matching pairs += 1; }

        }
        if current_pair_number < amount_of_pairs {

        }
    }
                                HOLY i read the assignment wrong. smh my head
*/


fn main() {
    
}
