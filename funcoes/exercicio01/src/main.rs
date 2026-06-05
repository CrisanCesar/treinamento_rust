use std::io;

fn soma_entre_valores(x: i16, y: i16) -> i16 {

    x + y
   
}
fn subtracao_entre_valores(x: i16, y: i16) -> i16 {
    x - y
}


fn menu() {
    loop { 
        println!("Digite uma das opções abaixo:");
        println!(r#"
            1) Soma entre valores
            2) Subtração entre valores
            3) Criar tabuada
            4) Sair
        "#);

        let mut opcao = String::new();
        io::stdin()
            .read_line(&mut opcao)
            .expect("Erro ao ler a entrada");
        let opcao: i16 = opcao.trim().parse().expect("Erro ao converter para inteiro");
        match opcao {     // Usando match para lidar com as opções
            1 => solicita_parematros_para_calculo(true),
            2 => solicita_parematros_para_calculo(false),
            3 => println!("Você escolheu a opção 3"),
            4 => break,
            _ => println!("Opção inválida, por favor escolha entre 1 e 4"),
        }
        }





    }

fn main() {
    menu();
}