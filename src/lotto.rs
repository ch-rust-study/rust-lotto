use rand::{self, Rng};
use std::{collections::HashSet, io};

use crate::constants::{LOTTO_LENGTH, LOTTO_PRICE, MAX_NUMBER, MIN_NUMBER};

pub struct Lotto {
    pub count: u32,
    pub win_numbers: Vec<u32>,
    pub bonus_num: u32,
    pub lottos: Vec<Vec<u32>>,
}

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

impl Lotto {
    pub fn new() -> Lotto {
        Lotto {
            count: 0,
            win_numbers: Vec::new(),
            bonus_num: 0,
            lottos: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        loop {
            let money = read_input(">구입 금액을 입력해주세요.");

            self.count = match money.trim().parse::<u32>() {
                Ok(num) => num / LOTTO_PRICE,
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

    fn is_valid_lotto(&self, num: u32) -> bool {
        return MIN_NUMBER <= num && num <= MAX_NUMBER;
    }

    fn has_invalid_number(&self, numbers: &Vec<u32>) -> bool {
        return numbers.iter().find(|&x| !self.is_valid_lotto(*x)).is_some();
    }

    fn has_duplicated_number(&self, numbers: &Vec<u32>) -> bool {
        let unique: HashSet<&u32> = numbers.iter().collect();
        unique.len() != numbers.len()
    }

    pub fn input_win_number(&mut self) {
        loop {
            let win_numbers = read_input("\n> 당첨 번호를 입력해 주세요.");

            let win_numbers: Vec<u32> = win_numbers
                .trim()
                .split(',')
                .filter_map(|num_str| num_str.trim().parse::<u32>().ok())
                .collect();

            if self.has_invalid_number(&win_numbers) {
                println!("번호는 1에서 45사이의 숫자여야 합니다.");
                continue;
            }

            if self.has_duplicated_number(&win_numbers) {
                println!("중복된 번호가 있으면 안됩니다.");
                continue;
            }

            if win_numbers.len() != LOTTO_LENGTH.try_into().unwrap() {
                println!("6개의 숫자를 컴마로 구분해서 입력해야 합니다.");
                continue;
            }

            self.win_numbers = win_numbers;
            break;
        }
    }

    pub fn input_bonus_number(&mut self) {
        loop {
            let bonus_num = read_input("\n> 보너스 번호를 입력해주세요.");

            let bonus_numbers: Vec<u32> = bonus_num
                .trim()
                .split(',')
                .filter_map(|num_str| num_str.trim().parse::<u32>().ok())
                .collect();

            if bonus_numbers.len() != 1 {
                println!("보너스 번호는 1개여야 합니다.");
                continue;
            }

            if self.has_invalid_number(&bonus_numbers) {
                println!("번호는 1에서 45사이의 숫자여야 합니다.");
                continue;
            }

            if self.win_numbers.contains(&bonus_numbers[0]) {
                println!("보너스 번호는 당첨 번호와 달라야 합니다.");
                continue;
            }

            self.bonus_num = bonus_numbers[0];
            break;
        }
    }
}
