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

  fn set_winning_numbers(&mut self) {
    self.matcher.set_numbers(get_winning_numbers(), get_bonus_number())
  }

  fn issue_lottos(&mut self) {
    let amount = lotto_input::get_amount();
    self.issuer.issue_lottos(amount);
  }

  pub fn run(&mut self) {
    self.issue_lottos();
    self.set_winning_numbers();
  }
}