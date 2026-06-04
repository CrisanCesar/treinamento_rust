use std::io;


fn main() {

    let mut valor_tabuada = String::new();

    println!("Digite o valor para a tabuada:");

    io::stdin()
        .read_line(&mut valor_tabuada)
        .expect("Erro ao ler a entrada");

    let valor_tabuada: i32 = valor_tabuada.trim().parse().expect("Erro ao converter para inteiro");

    for multiplicador in 1..=10 {
        println!("{} x {} = {}", valor_tabuada, multiplicador, valor_tabuada * multiplicador);
    }
}

