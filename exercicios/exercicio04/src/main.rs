static _y: u32 = 13;


fn main() {
    let x =5;
    let z = true;
    let numbers = (1, 2, 3);
         // todos acima vão ser armaxenados na stack
    let users = get_users(); // será armazenado na heap, pois é um tipo String, que tem um tamanho dinâmico
    
}
