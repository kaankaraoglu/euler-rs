pub fn solve(mut number: u64) -> u64 {
    let mut factor = 2;

    while factor * factor <= number {
        if number % factor == 0 {
            number /= factor;
        } else {
            factor += 1;
        }
    }

    number
}
