pub fn solve(mut n: u64) -> u64 {
    let mut factor = 2;

    while factor * factor <= n {
        if n % factor == 0 {
            n /= factor;
        } else {
            factor += 1;
        }
    }

    n
}
