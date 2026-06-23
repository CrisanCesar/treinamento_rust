fn main() {
    let x: i32 = 4; // owner
    let y: &i32 = &x; // y é uma referência para x

    println!("O valor de x é {}", x);
    println!("O valor de y é {}", y);

    // Imprimindo os endereços de memória
    println!("Endereço de memória de x: {:p}", &x); // {:p} imprime o endereço de memória
    println!("Endereço de memória de y: {:p}", y); // y já é uma referência, então não precisa usar &

    let t: &i32 = y; // cria outra referência para o owner x
    println!("Endereço de memória de t: {:p}", t);

    let w: i32 = *y; // desreferência com Copy para w
    println!("O valor de w é {}, endereço de memória de w: {:p}", w, &w);
}
