use std::io;
use rand::Rng;

fn column_vertical_and_horizontal(points: &mut i32) {
    println!("Fale a distância horizontal 1-16");
    let mut horizontal_column = String::new();
    io::stdin()
        .read_line(&mut horizontal_column)
        .expect("Failed to read line");

    println!("Fale a distância vertical 1-6");
    let mut vertical_column = String::new();
    io::stdin()
        .read_line(&mut vertical_column)
        .expect("Failed to read line");

    let mut rng = rand::thread_rng();

    let random_number_horizontal = rng.gen_range(1..=16);

    let random_number_vertical = rng.gen_range(1..=6);

    let horizontal_guess: i32 = horizontal_column.trim().parse().expect("Invalid input");
    let vertical_guess: i32 = vertical_column.trim().parse().expect("Invalid input");

    if horizontal_guess == random_number_horizontal || vertical_guess == random_number_vertical {
        *points += 1;
        println!("Você acertou! +1 ponto\nAgora você tem {} ponto(s)!", *points);
    } else {
        *points -= 1;
        println!("Você errou! -1 ponto\nAgora você tem {} ponto(s)", *points);
    }
}

fn game() {
    println!("Iniciando o jogo...");
    let mut points = 3;

    loop {
        for _ in 0..6 {
            println!("****************");
        }

        column_vertical_and_horizontal(&mut points);

        if points <= 0 {
            println!("FIM DE JOGO!");
            break;
        }

        println!("Deseja jogar novamente? (s/n)");
        let mut play_again = String::new();
        io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read line");

        if play_again.trim().to_lowercase() != "s" {
            println!("Encerrando o programa...");
            break;
        }
    }
}

fn main() {
    loop {
        println!("1. Começar\n2. Sair");

        let mut option_menu = String::new();

        io::stdin()
            .read_line(&mut option_menu)
            .expect("Failed to read line");

        let option_menu: u32 = match option_menu.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrada inválida. Por favor, insira 1 para começar ou 2 para sair.");
                continue;
            }
        };

        match option_menu {
            1 => game(),
            2 => {
                println!("Encerrando o programa...");
                break;
            }
            _ => println!("Opção inválida. Por favor, insira 1 para começar ou 2 para sair."),
        }
    }
}
