fn main() {
    let mut x: i32 =4; // Declaro x como mutável
    let y: &i32 = &x;

    println!("O valor de x é {}", x);
    println!("O valor de y é {}", y);

    // Modifique x para invalidar y
    x = 42; // modificando o owner

    // Agora, y se tornou uma referência invalida
    // Tentar imprimir y resultará em um erro de tempo de compilação
    //println!("O valor de y é {}", y); 
    println!("O valor de x é {}", x);
}
