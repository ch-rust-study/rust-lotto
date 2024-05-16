use super::lotto::Lotto;
use rand::seq::SliceRandom; 
use rand::thread_rng;

pub struct LottoIssuer {
  lottos: Vec<Lotto>
}

fn generate_number(min: i32, max: i32, count: u32) -> Vec<i32> {
  let range = min..=max;
  let mut candidates: Vec<i32> = range.collect();
  let mut rng = thread_rng();
  candidates.shuffle(&mut rng);
  candidates[0..(count as usize)].to_vec()
}

impl LottoIssuer {
  pub fn new() -> LottoIssuer{
    LottoIssuer {
      lottos: Vec::new()
    }
  }

  pub fn issue_lottos(&mut self, count: u32) {
    for _ in 0..count {
      let mut new_lotto = Lotto::new();
      new_lotto.issue(&generate_number)
    }
  }
}