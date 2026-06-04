use std::io;


fn main() {

    loop { // Iniciando um loop infinito//
        println!("Digite uma das opções abaixo:");
        println!(r#"
        opção 1
        opção 2
        opção 3
        opção 4
        "#);    
    
     // prinrln!("Opção 1\nOpção 2\nOpção 3\nOpção 4"); --- Segunda forma de fazer ---

    let mut opcao = String::new();   // Criando uma variável mutável para armazenar a entrada do usuário

    io::stdin()   // Lendo a entrada do usuário
        .read_line(&mut opcao)
        .expect("Erro ao ler a entrada");

    let opcao: i8 = opcao.trim().parse().expect("Por favor, digite um número inteiro válido");    // Convertendo a entrada para um número inteiro

    match opcao {     // Usando match para lidar com as opções
        1 => println!("Você escolheu a opção 1"),
        2 => println!("Você escolheu a opção 2"),
        3 => println!("Você escolheu a opção 3"),
        4 => break,
        _ => println!("Opção inválida, por favor escolha entre 1 e 4"),
    }
    } }

