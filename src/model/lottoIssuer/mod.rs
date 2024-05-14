use super::lotto::Lotto;
use rand::seq::SliceRandom; 
use rand::thread_rng;

pub struct LottoIssuer {
  lottos: Vec<Lotto>
}

impl LottoIssuer {
  pub fn new() -> LottoIssuer{
    LottoIssuer {
      lottos: Vec::new()
    }
  }

  fn generate_number(min: i32, max: i32, count: u32) -> Vec<i32> {
    let range = min..=max;
    let candidates: Vec<i32> = range.collect();
    let mut rng = thread_rng();
    candidates.shuffle(&mut rng);
    candidates[0..count].collect()
  }

  pub fn issue_lottos(&mut self, count: u32) {
    for _ in 0..count {
      let new_lotto = Lotto::new();
      new_lotto.issue(&generate_number)
    }
  }
}