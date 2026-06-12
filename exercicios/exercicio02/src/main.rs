
const SECONDS_IN_MINUTE: u32 = 60; // SECONDS_IN_MINUTE é uma constante que representa o número de segundos em um minuto. O tipo u32 indica que é um número inteiro sem sinal de 32 bits.
const MINUTES_IN_HOUR: u32 = 60; // MINUTES_IN_HOUR é outra constante que representa o número de minutos em uma hora.
const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE * MINUTES_IN_HOUR; // SECONDS_IN_HOUR é calculado multiplicando o número de segundos em um minuto (SECONDS_IN_MINUTE) pelo número de minutos em uma hora (MINUTES_IN_HOUR). Isso resulta no total de segundos em uma hora.

fn main() {
   let total = 30;
   let total_em_segundos = total * SECONDS_IN_HOUR; // total_em_segundos é uma variável que armazena o resultado da multiplicação de total pelo valor da constante SECONDS_IN_HOUR. Isso calcula o total de segundos em 30 horas.
    
    println!("Trabalhou {} segundos", total_em_segundos);
}
