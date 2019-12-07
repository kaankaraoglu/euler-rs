fn main() {
    println!("Multiples of 3 and 5 up to 1000: {}", multiples_of_3_and_5());
    println!("Sum of all even fibonacci numbers up to 4 million: {}", even_fibonacci_numbers(1,2,4000000));
}

fn multiples_of_3_and_5() -> i32 {
    let mut sum: i32 = 0;
    let mut vector: Vec<i32> = Vec::new();

    for i in 0..1000 {
        if i % 3 == 0 || i % 5 == 0 {
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