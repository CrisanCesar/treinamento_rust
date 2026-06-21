#![allow(unused, dead_code)]
fn main() {

    let a = String::from("Crisan"); // No copy
    let b = &a; // movendo o valor de A para B (usando o & o valor é emprestado)
    println!("O valor de A é {}", a);
    println!("O valor de B é {}", b);
}


//Regras de Ownership em Rust
// 1 Cada valor tem um dono (owner)
// 2 Só pode ter um único dono
// 3 Quando o dono sai de escopo o valor é limpo
// 4 A posse (owwnership) pode ser movida a outro dono