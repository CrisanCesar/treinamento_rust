fn main() {

    let input = "12 5 30 18 7 42 9 60";




    let result: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .filter(|&n| n > 15)
        .map(|n| n * 2)
        .collect();


    let soma: i32 = result.iter().sum();

    println!("Soma dos números restantes: {}", soma);
}
