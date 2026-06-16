fn main() {
    let input = "10 25 8 42 17 3 50";
    let result: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .filter(|&n| n % 2 == 0)
        .collect();
    let soma: i32 = result.iter().sum();

    println!("Soma dos números restantes: {}", soma);
}
