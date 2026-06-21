#![allow(unused, dead_code)]
fn to_uppercase(text: &mut String) {
    *text = text.to_uppercase()
}
fn add_prefix(text: &mut String) {
    *text = format!("FOO_{text}");
   
}
fn main() {
    let mut name = "Crisan".to_string(); 
    to_uppercase(&mut name); // mut borrow
    add_prefix(&mut name); 

    println!("{name}");

    
}



// Regrad de Barrowing

// 1. Podemos ter uma única referência caso ela seja mutável
// 2. Podemos ter varias quando são imutáveis