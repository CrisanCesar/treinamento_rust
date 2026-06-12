use std::io;

fn main() {
    println!("{:-^40}", "calculadora");

    let mut s = String::new(); 
    let banner = 
        "Digite uma sequencia de numeros\n 
        separados por virgula\n
        exemplo: 1, 2, 3, 4, 52";
    println!("{banner}");
    io::stdin()
        .read_line(&mut s) 
        .expect("Falha ao ler a linha");
    let nums: Vec<i32> = s.split(",").map(|c| c.trim().parse().expect("Error")).collect(); //dividindo a string em partes usando a vírgula como separador, removendo espaços em branco e convertendo cada parte para um número inteiro, armazenando os números em um vetor
    let result: i32 = nums.iter().sum(); //somando os números do vetor usando iter() para criar um iterador e sum() para calcular a soma
    println!("A soma dos números é: {}", result); //imprimindo o resultado da soma
    


    println!(
        "O número de caracteres digitados é: {}",
        s.trim().chars().count()
    );
    println!("{}", "===".repeat(40)); 
}
