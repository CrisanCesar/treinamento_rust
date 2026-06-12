fn main() {
    let x = 5; // x é uma variável do tipo inteiro (i32 por padrão) que armazena o valor 5.
    let y = 200_154_123_9; // o Rust permite usar underscores para melhorar a legibilidade de números grandes.
    let h = 0xff; // hexadecimal
    let o = 0o77; // octal
    let b = 0b1111_0000; // binário 
    let f = 2.5; // f é uma variável do tipo número de ponto flutuante (f64 por padrão) que armazena o valor 2.5.
    let c = 'A'; // c é uma variável do tipo caractere (char) que armazena o valor 'A' ((tem que usar aspas simples '' para caracteres, e aspas duplas "" para strings)).
    let b = true; // b é uma variável do tipo booleano (bool) que armazena o valor true.
    let numbers = (1, 2, 3); // numbers é uma variável do tipo tupla (tuple) que armazena os valores 1, 2 e 3. As tuplas podem conter diferentes tipos de dados.
    let array = [1, 2, 3, 4, 5]; // array é uma variável do tipo array que armazena os valores 1, 2, 3, 4 e 5. Os arrays em Rust têm um tamanho fixo e todos os elementos devem ser do mesmo tipo.
}
 


// escalares (scalar types)
// Representa um único valor. Existem quatro tipos escalares em Rust: inteiros, números de ponto flutuante, booleanos e caracteres.

// compostos (compound types)
// Representa um grupo de valores. Existem dois tipos compostos em Rust: tuplas e arrays.
// tupla (tuple) ex: let tupla: (i32, f64, u8) = (500, 6.4, 1);
// array ex: let array: [i32; 5] = [1, 2, 3, 4, 5];

// Inteiros 
// bits: 8, 16, 32, 64, 128, arch
// signed: i8, i16, i32, i64, i128, isize
// unsigned: u8, u16, u32, u64, u128, usize