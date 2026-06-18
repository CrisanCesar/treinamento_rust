
fn converter(input: &str) -> Vec<i32> {
    input
         
    .split_whitespace()
    .map(|s| s.parse::<i32>().unwrap())
    .collect()
}

fn somar_pares(numeros: Vec<i32>) -> i32 {
    numeros.into_iter().filter(|n| n % 2 == 0).sum()
}


fn main() {
   let numeros = "10 25 8 42 17 3 50"; 

   let vetor = converter(numeros);

   let soma = somar_pares(vetor);

   println!("{}", soma);
}
