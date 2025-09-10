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
    let numberSequence: &str = "5 3 2 1 1";
}


// divides by 2 but rounds up in case of odd input
fn divide_by_2(x: u32) -> u32 {
    if x % 2 == 0 {  return x/2;  }
    else {  return (x+1)/2; }
}