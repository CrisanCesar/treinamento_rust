fn main() {
    // Exemplo de String
    let mut s1: String = String::from("Olá, mundo!"); // s1 é uma String alocada na memória heap
    s1 += "- teste"; // Modificando a String s1

    let s2: String = s1.clone(); // Clonando a String s1 para criar a s2

    println!("String s1: {} - referência: {:p}", s1, &s1);
    println!("String s2: {} - referência: {:p}", s2, &s2);

    // Exemplo de &str (slice)
    let s3: &str = "Olá, mundo!"; // s3 é uma &str (slice de string)
    let s4: String = format!("{} - teste", s3); // Criando um novo &str a partir de s3

    println!("String s3: {} - referência: {:p}", s3, s3);
    println!("String s4: {} - referência: {:p}", s4, &s4);

}
