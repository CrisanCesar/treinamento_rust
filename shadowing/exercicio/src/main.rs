fn main() {
    /*
    Dado que eu tenha um ano de nascimento, e faço a subtração pelo ano atual, devo resultar na idade da pessoa.
     */

    let nome: &str = "Crisan";

    let ano_nascimento: u16 = 2002;
    let mes_nascimento: u16 = 5;
    let dia_nascimento: u16 = 23;
    let ano_atual: u16 = 2026;
    let mes_atual: u16 = 6;
    let dia_atual: u16 = 2;
    let mut idade: u16 = ano_atual - ano_nascimento;
    if mes_nascimento >= mes_atual {
        idade -= 1;
    } else if mes_nascimento == mes_atual && dia_nascimento > dia_atual {
        idade -= 1;
    }
    println!(
        "A idade do {} calculada para o ano de nascimento {} é: {} anos",
        nome, ano_nascimento, idade
    );
}
