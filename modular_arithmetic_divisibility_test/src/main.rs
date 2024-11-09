fn is_divisible_by(number: &str, base: u32, divisor: u32) -> bool {
    let mut remainder = 0;

    for digit in number.chars() {
        let digit_value = digit.to_digit(base).unwrap();
        remainder = (remainder * base + digit_value) % divisor;
    }

    remainder == 0
}

fn main() {
    let number = "1100"; 
    let base = 7;        // Base of the number 
    let divisor = 23;     // Divisor to check for divisibility

    if is_divisible_by(number, base, divisor) {
        println!("The number {} in base {} is divisible by {}", number, base, divisor);
    } else {
        println!("The number {} in base {} is NOT divisible by {}", number, base, divisor);
    }
}

