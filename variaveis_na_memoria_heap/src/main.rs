fn main() {
    // variáveis na memória heap
    
    // Não é uma variável by copy
    let s1: String = String::from("Olá"); // s1 possui a propriedade da String
    let s2: String = s1; // A propriedade é transferida de s1 para s2 (Borrowing)


    // Isso causa um erro, porque a s1 não é mais válido após a transferência
    // println!("s1: {} - referência: {:p}", s1, &s1);

    // s2 é válido e pode ser usado
    println!("s2: {} - referência: {:p}", s2, &s2);
}
