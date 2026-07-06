fn main() {
    let original_string: String = String::from("Rust é incrível!"); // original_string possui a propriedade da String

    // criando uma substring usando slicing
    let substring: &str = &original_string[0..4]; // substring é uma referência para os primeiros 4 bytes da original_string
    println!("String original: {} - referência: {:p}", original_string, &original_string);
    println!("String substring: {} - referência: {:p}", substring, substring);
}
