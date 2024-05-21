pub struct LottoMatcher {
  winning_numbers: Option<Vec<i32>>,
  bonus_number: Option<i32>
}

impl LottoMatcher {
  pub fn new() -> LottoMatcher {
    LottoMatcher {
      winning_numbers: None,
      bonus_number: None
    }
  }

  pub fn set_numbers(&mut self, winning_numbers: Vec<i32>, bonus_number: i32) {
    // TODO: 에러처리
    self.winning_numbers = Some(winning_numbers);
    self.bonus_number = Some(bonus_number);
  }
}