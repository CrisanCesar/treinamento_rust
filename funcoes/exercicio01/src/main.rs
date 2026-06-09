use std::io;

fn solicita_parematros_para_calculo(soma: bool) {
    let mut x: String = String::new();
    let mut y: String = String::new();
    println!("Digite o primeiro valor:");
    io::stdin()
        .read_line(&mut x)
        .expect("Erro ao ler a entrada");
    println!("Digite o segundo valor:");
    io::stdin()
        .read_line(&mut y)
        .expect("Erro ao ler a entrada");
    let x: i16 = x.trim().parse().expect("Erro ao converter para inteiro");
    let y: i16 = y.trim().parse().expect("Erro ao converter para inteiro");
    if soma {
        println!("O resultado da soma é: {}", x + y);
    } else {
        println!("O resultado da subtração é: {}", x - y);
    }
}
fn solicita_tabuada() {
    let mut x: String = String::new();
    println!("Digite o valor para criar a tabuada:");
    io::stdin()
        .read_line(&mut x)
        .expect("Erro ao ler a entrada");
    let x: i16 = x.trim().parse().expect("Erro ao converter para inteiro");
    for i in 1..=10 {
        println!("{} x {} = {}", x, i, x * i);
    }
}

fn menu() {
    loop {
        println!("Digite uma das opções abaixo:");
        println!(
            r#"
            1) Soma entre valores
            2) Subtração entre valores
            3) Criar tabuada
            4) Sair
        "#
        );

        let mut opcao = String::new();
        io::stdin()
            .read_line(&mut opcao)
            .expect("Erro ao ler a entrada");
        let opcao: i16 = opcao
            .trim()
            .parse()
            .expect("Erro ao converter para inteiro");
        match opcao {
            // Usando match para lidar com as opções
            1 => solicita_parematros_para_calculo(true),
            2 => solicita_parematros_para_calculo(false),
            3 => solicita_tabuada(),
            4 => break,
            _ => println!("Opção inválida, por favor escolha entre 1 e 4"),
        }
    }
}

fn main() {
    menu();
}
