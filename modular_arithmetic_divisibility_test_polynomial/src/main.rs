use num_bigint::BigInt;
use num_traits::Zero;
use std::str::FromStr;

/// Function to check if a number in a given base is divisible by a divisor (ps, the divisoris base 10)
fn is_divisible_by(number: &str, base: u32, divisor: &BigInt) -> bool {
    let mut result = BigInt::zero();
    let base_bigint = BigInt::from(base);

    for (i, digit_char) in number.chars().enumerate() {
        // Convert the character to a number based on its ASCII value.
        let digit_value = if digit_char.is_digit(10) {
            digit_char.to_digit(10).unwrap()
        } else {
            digit_char.to_digit(36).unwrap()
        };

        let digit_bigint = BigInt::from(digit_value);
        let position = number.len() - i - 1;
        let term_mod = base_bigint.modpow(&BigInt::from(position), divisor);
        result = (result + digit_bigint * term_mod) % divisor;
    }

    result.is_zero()
}

fn main() {
    let number = "37654438998755544549087745";
    let base = 6;
    let divisor = BigInt::from(4598676);

    if is_divisible_by(number, base, &divisor) {
        println!("The number {} in base {} is divisible by {}.", number, base, divisor);
    } else {
        println!("The number {} in base {} is NOT divisible by {}.", number, base, divisor);
    }
}

