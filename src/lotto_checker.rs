use crate::constants::LOTTO_PRICE;

pub struct LottoChecker {
    win_numbers: Vec<u32>,
    bonus_num: u32,
}

impl LottoChecker {
    pub fn new(win_numbers: Vec<u32>, bonus_num: u32) -> LottoChecker {
        LottoChecker {
            win_numbers,
            bonus_num,
        }
    }

    pub fn count_matching_numbers(&self, lotto: &Vec<u32>) -> usize {
        lotto
            .iter()
            .filter(|&&num| self.win_numbers.contains(&num))
            .count()
    }

    pub fn count_matching_numbers_bonus(&self, lottos: &Vec<Vec<u32>>) -> usize {
        lottos
            .iter()
            .filter(|&lotto| {
                self.count_matching_numbers(lotto) == 5 && lotto.contains(&self.bonus_num)
            })
            .count()
    }

    pub fn print_result(&self, lottos: &Vec<Vec<u32>>) {
        let mut matchs: Vec<u32> = vec![0; 7];
        let checker = LottoChecker::new(self.win_numbers.clone(), self.bonus_num);

        let match_bonus: u32 = checker
            .count_matching_numbers_bonus(lottos)
            .try_into()
            .unwrap();

        for lotto in lottos {
            let match_count = checker.count_matching_numbers(&lotto);
            if !(match_count == 5 && lotto.contains(&self.bonus_num)) {
                matchs[match_count] += 1;
            }
        }

        println!("\n당첨 통계");
        println!("--------------------");
        println!("3개 일치 (5,000원) - {}개", matchs[3]);
        println!("4개 일치 (50,000원) - {}개", matchs[4]);
        println!("5개 일치 (1,500,000원) - {}개", matchs[5]);
        println!(
            "5개 일치, 보너스 볼 일치 (30,000,000원) - {}개",
            match_bonus
        );
        println!("6개 일치 (2,000,000,000원) - {}개", matchs[6]);

        let total_prize: u32 = matchs[3] * 5000
            + matchs[4] * 50000
            + matchs[5] * 1500000
            + match_bonus * 30000000
            + matchs[6] * 2000000000;

        let total_profit = total_prize as f64 * 100.0 / (lottos.len() as f64 * LOTTO_PRICE as f64);
        println!("총 수익률은 {}%입니다.\n", total_profit);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_matching_numbers() {
        let checker = LottoChecker::new(vec![1, 2, 3, 4, 5, 6], 7);
        assert_eq!(
            checker.count_matching_numbers(&vec![1, 2, 3, 10, 11, 12]),
            3
        );
        assert_eq!(checker.count_matching_numbers(&vec![1, 2, 3, 4, 5, 6]), 6);
    }

    #[test]
    fn test_count_matching_numbers_bonus() {
        let checker = LottoChecker::new(vec![1, 2, 3, 4, 5, 6], 7);
        let lottos = vec![
            vec![1, 2, 3, 4, 5, 7],
            vec![1, 2, 3, 4, 5, 6],
            vec![1, 2, 3, 4, 5, 8],
        ];
        assert_eq!(checker.count_matching_numbers_bonus(&lottos), 1);
    }
}
