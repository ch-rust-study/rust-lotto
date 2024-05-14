use rand::{self, Rng};
use std::io;

pub struct Lotto {
    pub count: u32,
    pub lottos: Vec<Vec<u32>>,
}

impl Lotto {
    pub fn new() -> Lotto {
        Lotto {
            count: 0,
            lottos: vec![vec![]],
        }
    }

    pub fn start(&mut self) {
        loop {
            println!("> 구입 금액을 입력해주세요.");
            let mut money = String::new();
            io::stdin()
                .read_line(&mut money)
                .expect("Failed to read line");

            self.count = match money.trim().parse::<u32>() {
                Ok(num) => num / 1000,
                Err(_) => continue,
            };

            println!("{}개를 구매했습니다.", self.count);

            self.lottos = self.make_lotto(self.count);
            break;
        }
    }

    fn make_lotto(&self, lotto_num: u32) -> Vec<Vec<u32>> {
        let mut lotto_numbers = Vec::new();

        for _ in 0..lotto_num {
            let mut lotto = Vec::new();
            while lotto.len() < 6 {
                let number: u32 = rand::thread_rng().gen_range(1..46);
                if !lotto.contains(&number) {
                    lotto.push(number);
                }
            }

            lotto.sort();
            lotto_numbers.push(lotto);
        }

        lotto_numbers
    }

    pub fn print_lotto(&mut self) {
        for lotto in self.lottos.iter() {
            print!("[");
            for (i, &num) in lotto.iter().enumerate() {
                if i != 0 {
                    print!(", ");
                }
                print!("{}", num);
            }
            println!("]");
        }
    }
}
