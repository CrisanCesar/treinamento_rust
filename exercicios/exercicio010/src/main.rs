fn main() {
    let input = "5 12 7 20 3 15 8";

    let result: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .filter(|&n| n > 10)
        .map(|n| n * 2)
        .collect();

    println!("{:?}", result);
}
