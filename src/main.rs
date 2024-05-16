mod model;
mod view;
mod controller;

use model::lotto_issuer::LottoIssuer;
use controller::lotto_controller::LottoController;

fn main() {
  let issuer = LottoIssuer::new();
  let mut controller = LottoController::new(issuer);

  controller.run();
}
