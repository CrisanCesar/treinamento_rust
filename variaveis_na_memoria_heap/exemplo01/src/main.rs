fn main() {
   // Variáveis na memória heap

   let s1: String = String::from("Olá"); // s1 possui a propriedade da String
   let s2: String = s1.clone(); // Clonando a String s1 para criar a s2

   println!("Antes da transferência: {}", s2);
   // print_string(&s1);
   print_string(&s2);
}
fn print_string(s: &String) {
   println!("Valor da String: {} - referência: {:p}", s, s);



}
