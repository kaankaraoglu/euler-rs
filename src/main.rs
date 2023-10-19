mod problems;

fn main() {
    // Sum of multiples of two numbers below a limit.
    let a: i32 = 3;
    let b: i32 = 5;
    let limit: i32 = 1000;
    println!(
        "Sum of multiples of {} and {} up to {} is: {}",
        a,
        b,
        limit,
        problems::sum_of_multiples_of_two_numbers_below_limit::solve(a, b, limit)
    );

    // Sum of all even fibonacci numbers below a limit.
    let limit: i64 = 4_000_000;
    println!(
        "Sum of all even fibonacci numbers up to {} is: {}",
        limit,
        problems::sum_of_all_even_fibonacci_numbers_below_limit::solve(limit)
    );

    // Largest prime factors of a number
    let number: u64 = 600851475143;
    println!(
        "Largest prime factor of {} is: {}",
        number,
        problems::largest_prime_factors_of_a_number::solve(number)
    );
}
