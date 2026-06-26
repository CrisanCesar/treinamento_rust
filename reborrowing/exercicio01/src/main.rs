fn main() {
    let mut x: i32 = 4;
    println!("[Original] - Valor de x após as modificações: {} - referência {:p}", x, &x);

    imprime_valor(&mut x); // passando uma referência mutaável para x

    println!("[Original] - Valor de x após as modificações: {} - referência {:p}", x, &x);

    imprime_valor(&mut x); // passando uma referência mutaável para x

    println!("[Original] -Valor de x após as modificações: {} - referência {:p}", x, &x);


}


fn imprime_valor(valor: &mut i32){
    *valor += 1; // modificando o valor referanciado por um valor utilizando um reborrowing
    // O compilador pode mover a variável temporariamente para uma localização diferente na memória durante a referência 
    // O objetivo é evitar possiveis problemas de aliasing e gerantir a segurança das referências mutáveis
    println!("[Reborrrowing] - Valor {}, Endereço de memória de x: {:p}", valor, &valor);

}    