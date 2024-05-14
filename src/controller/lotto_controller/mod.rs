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

  fn get_amount(&self) {

  }

  pub fn run(&self) {

  }
}