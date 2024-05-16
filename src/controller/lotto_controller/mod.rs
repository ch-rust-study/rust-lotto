use crate::{model::lotto_issuer::LottoIssuer, view::lotto_input};

pub struct LottoController {
  issuer: LottoIssuer,
}

impl LottoController {
  pub fn new(issuer: LottoIssuer) -> LottoController {
    LottoController {
      issuer
    }
  }

  fn get_count(&self) -> u32 {
    let amount = lotto_input::get_amount();
    // TODO: 에러처리
    amount / 1000
  }

  fn issue_lottos(&mut self) {
    let count = self.get_count();
    self.issuer.issue_lottos(count);
  }

  pub fn run(&mut self) {
    self.issue_lottos();
  }
}