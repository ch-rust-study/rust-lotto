use crate::{model::{lotto_issuer::LottoIssuer, lotto_matcher::LottoMatcher}, view::lotto_input::{self, get_bonus_number, get_winning_numbers}};

pub struct LottoController {
  issuer: LottoIssuer,
  matcher: LottoMatcher
}

impl LottoController {
  pub fn new(issuer: LottoIssuer, matcher: LottoMatcher) -> LottoController {
    LottoController {
      issuer,
      matcher,
    }
  }

  fn get_count(&self) -> u32 {
    let amount = lotto_input::get_amount();
    // TODO: 에러처리
    amount / 1000
  }

  fn set_winning_numbers(&mut self) {
    self.matcher.set_numbers(get_winning_numbers(), get_bonus_number())
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