mod lotto;

use std::io;

use lotto::Lotto;

pub fn is_retry() -> bool {
    loop {
        println!("다시 시작하겠습니까? (y/n)");

        let mut retry: String = String::new();
        io::stdin()
            .read_line(&mut retry)
            .expect("Failed to read line");

        match retry.trim().to_lowercase().as_str() {
            "y" => return true,
            "n" => return false,
            _ => println!("y 혹은 n을 입력해주세요"),
        }
    }
}

fn main() {
    loop {
        let mut game = Lotto::new();
        game.start();
        game.print_lotto();
        game.input_win_number();
        game.input_bonus_number();
        game.print_result();

        if !is_retry() {
            break;
        }
    }
}
