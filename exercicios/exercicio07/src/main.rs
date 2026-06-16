fn say_hello(name: &str, color: &str) { 
    println!("Hello, {name}, your color {color}");
}
fn add_numbers(x: i32, y: i32) -> i32 { 
    if x == 0 {
        return y;
    }
    x + y
}



fn main() {
    say_hello("Alice", "blue");
    say_hello("Bob", "red");

    let y = {
        say_hello("Alice", "blue");
        let x = 5;
        x + 94
    };
    println!("{:?}", y);

    let res: i32 = add_numbers(8, 9);
    println!("The result is: {}", res);
}


