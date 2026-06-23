fn main() {
   // Memória stack (variáveis do tipo copy no rust)
   let x: i32 = 4;
   let y: &i32 = &x; // Copia de dados

   imprime_valor(&x);
   imprime_valor(&y);



}
fn imprime_valor(valor: &i32){
    println!("Endereço de memória de y: {:p}", valor);
}
