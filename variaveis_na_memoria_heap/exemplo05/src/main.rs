fn main() {
    // Convertendo String em &str usando as_str()
    let s1: String = String::from("Olá, mundo!"); 
    let reference_to_s1: &str = s1.as_str(); // Convertendo String em &str

    println!("String s1: {} - referência: {:p}", s1, &s1);
    println!("&str reference_to_s1: {} - referência: {:p}", reference_to_s1, reference_to_s1);

    // Convertendo String para &str fazendo uma referência direta
    let s2: String = String::from("Rust é incrível!");
    let reference_to_s2: &str = &s2; // Fazendo uma referência direta para a String
    println!("String s2: {} - referência: {:p}", s2, &s2);
    println!("&str reference_to_s2: {} - referência: {:p}", reference_to_s2, reference_to_s2);
}
