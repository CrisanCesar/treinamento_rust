#![allow(unused, dead_code)]
fn main() {

    let a: i32 = 1; // valores do tipo copy (i32, f64, bool, char)
    let b = &a;
    println!("O valor de A é {}", a);
    println!("O valor de B é {}", *b);
}
