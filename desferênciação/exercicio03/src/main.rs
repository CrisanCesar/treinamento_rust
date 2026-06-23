fn main() {
    let idade: i32 = 24;

    let refe01: &i32 = &idade;

    let refe02 = refe01;

    let copia = *refe01;

    println!("refe01: {:p}", refe01);
    println!("refe02: {:p}", refe02);
    println!("copia: {}, endereço: {:p}", copia, &copia);
}
