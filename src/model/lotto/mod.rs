const LOTTO_NUMBERS_LENGTH: usize = 6;
const MIN_LOTTO_NUMBER: i32 = 1;
const MAX_LOTTO_NUMBER: i32 = 45;

type LottoNumbers = Option<[i32; LOTTO_NUMBERS_LENGTH]>;

pub struct Lotto {
  numbers: LottoNumbers,
}

impl Lotto {
  pub fn new() -> Lotto {
    Lotto {
      numbers: None,
    }
  }

  pub fn getNumbers(&self) -> &LottoNumbers {
    &self.numbers
  }

  pub fn issue(&mut self, generate_number: &dyn Fn(i32, i32, u32) -> Vec<i32>) {
    let generated = generate_number(MIN_LOTTO_NUMBER, MAX_LOTTO_NUMBER, LOTTO_NUMBERS_LENGTH as u32).try_into().unwrap();
    self.numbers = Some(generated);
  }
}