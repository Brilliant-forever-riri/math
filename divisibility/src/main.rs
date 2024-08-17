// Start by importing the library for big integers and the other one for integer operations
use num_bigint::BigUint; // This is for big numbers 
use num_integer::Integer; // This is for integer operation div_rem

fn main() {
    // Convert the large integers into BigUint
    let number = BigUint::parse_bytes(b"2547239208364572682092387645537298", 10).unwrap();
    let divisor = BigUint::parse_bytes(b"463782892903278436783298", 10).unwrap();

    // Perform division and get quotient and remainder
    let (quotient, remainder) = number.div_rem(&divisor);

    // Print the results
    println!("The quotient is {quotient}");
    println!("The remainder is {remainder}");
}



