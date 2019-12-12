fn main() {
    // Multiples of two numbers below a limit.
    let a: i32 = 3;
    let b: i32 = 5;
    let limit: i32 = 1000;
    println!("Multiples of {} and {} up to {}: {}", a, b, limit, multiples_of_two_numbers_below_limit(a, b, limit));

    // Sum of all even fibonacci numbers below a limit.
    let limit: i64 = 4_000_000;
    println!("Sum of all even fibonacci numbers up to {}: {}", limit, even_fibonacci_numbers(1, 2, limit));
}

fn multiples_of_two_numbers_below_limit(a: i32, b: i32, limit: i32) -> i32 {
    let mut sum: i32 = 0;
    let mut vector: Vec<i32> = Vec::new();

    for i in 0..limit {
        if i % a == 0 || i % b == 0 {
            vector.push(i);
        }
    }

    for i in 0..vector.len() {
        sum += vector[i];
    }
    sum
}

fn even_fibonacci_numbers(mut before_last: i64, mut last: i64, max: i64) -> i64 {
    let mut sum = 0;
    while before_last < max {
        if before_last % 2 == 0 {
            sum += before_last;
        }
        let c = before_last + last;
        before_last = last;
        last = c;
    }
    sum
}