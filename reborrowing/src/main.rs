fn main() {
   // memória stack (variáveis do tipo copy no rust)
   let mut x: i32 = 4;


   imprime_valor(&x);
   imprime_valor(&x);
}
 
fn imprime_valor(valor: &i32){
    // valor += 1; // não pode porque tenho imutabilidade nas referências
    println!("Valor {}, Endereço de memória de x: {:p}", valor, valor);
} 