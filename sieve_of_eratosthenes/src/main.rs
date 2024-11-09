fn sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; limit + 1];
    let mut primes = Vec::new();
    // Mark 0 and 1 as not prime
    is_prime[0] = false;
    if limit >= 1 {
        is_prime[1] = false;
    }

    for number in 2..=limit {
        if is_prime[number] {
            primes.push(number); // Number is prime, add to list
            let mut multiple = number * number;
            while multiple <= limit {
                is_prime[multiple] = false;
                multiple += number;
            }
        }
    }

    primes
}

fn main() {
    let limit = 10000000; 
    let primes = sieve_of_eratosthenes(limit);
    println!("Found {} primes up to {}.", primes.len(), limit);
}

