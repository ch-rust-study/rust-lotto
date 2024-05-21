mod model;
mod view;
mod controller;

use model::{lotto_issuer::LottoIssuer, lotto_matcher::LottoMatcher};
use controller::lotto_controller::LottoController;

fn main() {
  let issuer = LottoIssuer::new();
  let matcher = LottoMatcher::new();
  let mut controller = LottoController::new(issuer, matcher);

  controller.run();
}
