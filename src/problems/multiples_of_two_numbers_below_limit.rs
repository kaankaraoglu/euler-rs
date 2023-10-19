pub fn solve(a: i32, b: i32, limit: i32) -> i32 {
    (0..limit).filter(|&i| i % a == 0 || i % b == 0).sum()
}
