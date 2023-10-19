pub fn solve(max: i64) -> i64 {
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
