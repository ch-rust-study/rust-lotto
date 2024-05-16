use crate::{model::lotto_issuer::LottoIssuer, view::lotto_input::{self, get_bonus_number, get_winning_numbers}};

pub struct LottoController {
  issuer: LottoIssuer,
  winning_numbers: Option<Vec<i32>>,
  bonus_number: Option<i32>,
}

impl LottoController {
  pub fn new(issuer: LottoIssuer) -> LottoController {
    LottoController {
      issuer,
      winning_numbers: None,
      bonus_number: None,
    }
  }

  fn get_count(&self) -> u32 {
    let amount = lotto_input::get_amount();
    // TODO: 에러처리
    amount / 1000
  }

  fn set_winning_numbers(&mut self) {
    // TODO: 에러처리
    self.winning_numbers = Some(get_winning_numbers());
    self.bonus_number = Some(get_bonus_number());
  }

  fn issue_lottos(&mut self) {
    let count = self.get_count();
    self.issuer.issue_lottos(count);
  }

  pub fn run(&mut self) {
    self.issue_lottos();
    self.set_winning_numbers();
  }
}