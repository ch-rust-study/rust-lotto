mod model;
mod view;
mod controller;

use model::lotto_issuer::LottoIssuer;
use controller::lotto_controller::LottoController;

fn main() {
  let issuer = LottoIssuer::new();
  let controller = LottoController::new(issuer);

  controller.run();
}
