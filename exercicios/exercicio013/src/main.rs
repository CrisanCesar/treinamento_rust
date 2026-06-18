fn converter_notas(input: &str) -> Vec<i32> {
    input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn calcular_media(notas: &Vec<i32>) -> f32 {
    let soma: i32 = notas.iter().sum();

    soma as f32 / notas.len() as f32
}

fn contar_aprovados(notas: &Vec<i32>) -> usize {
    notas
        .iter()
        .filter(|&&nota| nota >= 6)
        .count()
}

fn maior_nota(notas: &Vec<i32>) -> i32 {
    *notas.iter().max().unwrap()
}

fn main() {
    let input = "7 10 5 8 4 9 6";

    let notas = converter_notas(input);

    let media = calcular_media(&notas);
    let aprovados = contar_aprovados(&notas);
    let maior = maior_nota(&notas);

    println!("Notas: {:?}", notas);
    println!("Média: {}", media);
    println!("Aprovados: {}", aprovados);
    println!("Maior nota: {}", maior);
}