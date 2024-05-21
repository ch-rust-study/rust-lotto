mod constants;
mod lotto;
mod lotto_checker;

use std::io;

use lotto::Lotto;
use lotto_checker::LottoChecker;

pub fn is_retry() -> bool {
    loop {
        println!("\n> 다시 시작하겠습니까? (y/n)");

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
        let mut lotto = Lotto::new();
        lotto.start();
        lotto.print_lotto();
        lotto.input_win_number();
        lotto.input_bonus_number();

        let checker = LottoChecker::new(lotto.win_numbers.clone(), lotto.bonus_num);
        checker.print_result(&lotto.lottos);

        if !is_retry() {
            break;
        }
    }
}
