use num_bigint::BigUint; //to export this library that can handle large numbers 
use num_traits::Zero; //allows me to start the operation from zero

fn is_divisible_by(number: &str, base: u32, divisor: u64) -> bool {
    let divisor_big: BigUint = BigUint::from(divisor);
    let base_big: BigUint = BigUint::from(base);
    let mut remainder: BigUint = BigUint::zero();

    for digit in number.chars() {
        let digit_value: BigUint = BigUint::from(digit.to_digit(base).unwrap());
        remainder = (remainder * &base_big + digit_value) % &divisor_big;
    }

    remainder.is_zero()
}

fn main() {
    let number: &str = "123456789012345678901234567890"; // Example large number in base 10 (decimal)
    let base: u32 = 2;       // Base of the number (decimal)
    let divisor: u64 = 123456564687; // Divisor to check for divisibility

    if is_divisible_by(number, base, divisor) {
        println!("The number {} in base {} is divisible by {}", number, base, divisor);
    } else {
        println!("The number {} in base {} is NOT divisible by {}", number, base, divisor);
    }
}
