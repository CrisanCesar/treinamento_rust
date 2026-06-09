fn main() {
    // memoria stack
    static xx: i16 = 10;
    // memoria heap
    let s: string::String = String::from("Hello, world!");
    println!("Valor de xx: {}", xx);
    println!("Valor de s: {}", s);
}
