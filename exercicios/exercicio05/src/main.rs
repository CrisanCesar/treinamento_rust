use std::io;

fn main() {
    println!("{}", "===".repeat(50)); //imprimindo uma linha de separação
    let mut s = String::new(); //criando uma string vazia
    println!("Digite um texto:");
    io::stdin()
        .read_line(&mut s) //lendo a linha do usuário e armazenando na string
        .expect("Falha ao ler a linha"); //tratando erros caso a leitura falhe
    println!("Você digitou: {}", s); //imprimindo a string
    println!(
        "O número de caracteres digitados é: {}",
        s.trim().chars().count()
    ); // imprimindo o número de caracteres digitados, usando trim() para remover espaços em branco e chars().count() para contar os caracteres 
    println!("{}", "===".repeat(50)); //imprimindo uma linha de separação
}

//aspas simples estaremos criando um caracter
//aspas duplas estaremos criando uma string (texto literal)
//let mut s = String::new(); //criando uma string vazia
//s.push('c'); //adicionando um caracter a string
//s.push_str("urso"); //adicionando uma string a string
//println!("{}", s); //imprimindo a string

