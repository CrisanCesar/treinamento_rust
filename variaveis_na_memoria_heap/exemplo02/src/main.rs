fn main() {
    // Exemplo de String

    let s1: String = String::from("Olá"); // s1 possui a propriedade da String
    let s2: String = s1.clone(); // Clonando a String s1 para criar a s2

    println!("String s1: {} - referência: {:p}", s1, &s1);
    println!("String s2: {} - referência: {:p}", s2, &s2);

    // Exemplo de &str (slice)

    let s3: &str = "Olá, mundo!"; // s3 é uma &str (slice de string)
    let s4: &str = s3; // s4 é uma referência para o mesmo &str

    println!("String s3: {} - referência: {:p}", s3, s3);
    println!("String s4: {} - referência: {:p}", s4, s4);


}
