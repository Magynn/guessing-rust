// Importar a biblioteca padrão de entrada e saída
use std::io;

// Importar a biblioteca padrão de números aleatórios
use rand::Rng;

fn main() {
    // Gerar um número aleatório entre 1 e 10
    let secret_number = rand::thread_rng().gen_range(1..=10);

    // Imprimir uma mensagem de boas-vindas
    println!("Bem-vindo ao jogo de adivinhação!");
    println!("Eu pensei em um número entre 1 e 10.");

    // Criar um loop para repetir até o usuário acertar o número
    loop {
        // Pedir ao usuário para digitar um palpite
        println!("Por favor, digite seu palpite.");

        // Criar uma variável mutável para armazenar o palpite
        let mut guess = String::new();

        // Ler a entrada do usuário e colocá-la na variável guess
        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler a linha");

        // Converter a entrada do usuário em um número inteiro
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Isso não é um número válido.");
                continue;
            }
        };

        // Imprimir o palpite do usuário
        println!("Você chutou: {}", guess);

        // Comparar o palpite do usuário com o número secreto
        match guess.cmp(&secret_number) {
            // Se o palpite for menor, imprimir uma dica
            std::cmp::Ordering::Less => println!("Muito baixo."),
            // Se o palpite for maior, imprimir uma dica
            std::cmp::Ordering::Greater => println!("Muito alto."),
            // Se o palpite for igual, imprimir uma mensagem de parabéns e sair do loop
            std::cmp::Ordering::Equal => {
                println!("Você acertou!");
                break;
            }
        }
    }
}