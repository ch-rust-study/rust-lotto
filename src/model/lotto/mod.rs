const LOTTO_NUMBERS_LENGTH: usize = 6;

type LottoNumbers = [i32; LOTTO_NUMBERS_LENGTH];

pub struct Lotto {
  pub numbers: LottoNumbers,
}

impl Lotto {
  pub fn new(numbers: LottoNumbers) -> Lotto {
    Lotto {
      numbers,
    }
  }

  pub fn getNumbers(&self) -> &LottoNumbers {
    &self.numbers
  }
}