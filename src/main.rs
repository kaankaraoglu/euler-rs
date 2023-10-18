fn main() {
    // Multiples of two numbers below a limit.
    let a: i32 = 3;
    let b: i32 = 5;
    let limit: i32 = 1000;
    println!(
        "Multiples of {} and {} up to {}: {}",
        a,
        b,
        limit,
        multiples_of_two_numbers_below_limit(a, b, limit)
    );

    // Sum of all even fibonacci numbers below a limit.
    let limit: i64 = 4_000_000;
    println!(
        "Sum of all even fibonacci numbers up to {}: {}",
        limit,
        sum_of_all_even_fibonacci_numbers_below_limit(limit)
    );
}

fn multiples_of_two_numbers_below_limit(a: i32, b: i32, limit: i32) -> i32 {
    (0..limit).filter(|&i| i % a == 0 || i % b == 0).sum()
}

fn sum_of_all_even_fibonacci_numbers_below_limit(max: i64) -> i64 {
    let (mut a, mut b) = (1, 1); // Initial Fibonacci numbers
    let mut sum = 0;

    while a <= max {
        if a % 2 == 0 {
            sum += a;
        }

        // Calculate the next Fibonacci numbers
        let temp = a + b;
        a = b;
        b = temp;
    }

    sum
}
