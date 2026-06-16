fn main() {
    let input = "10 20 30 40 50";
    println!("Números originais:{}", input);

    let result: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .map(|n| n * 3)
        .collect();

    println!("Números multiplicados:{:?}", result);

    let soma: i32 = result.iter().sum();
    println!("Soma:{:?}", novoset);
}
