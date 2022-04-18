mod myprimal;

fn main() {
    let suspect = 5273;
    println!("{} is prime: {}", suspect, myprimal::sieve(suspect));
    let not_a_prime = 1024;
    println!("{} is prime: {}", not_a_prime, myprimal::sieve(not_a_prime));

    let n = 1000;
    match myprimal::nth(n) {
        Some(number) => println!("{}th prime is {}", n, number),
        None => println!("I don't know anything about {}th prime.", n),
    }
    let nother = 100000;
    match myprimal::nth(nother) {
        Some(number) => println!("{}th prime is {}", nother, number),
        None => println!("I don't know anything about {}th prime.", nother),
    }

    println!("{:?}", myprimal::factor(2610));

    println!("{:?}", myprimal::count_divisors(2610));
}
