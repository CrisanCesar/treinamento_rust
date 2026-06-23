#![allow(unused, dead_code)]
fn say_hello(text: &String) {
    println!("Hello, {text}")
}
fn say_goodbye(text: &String) {
    println!("Goodbye, {text}")
}
fn main() {
    let name = "Crisan".to_string(); 
    say_hello(&name); // Emprestar(Borrow) = Referencia
    say_goodbye(&name);

    
}
