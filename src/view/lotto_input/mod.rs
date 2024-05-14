use super::common::prompt;

pub fn get_amount() -> u32 {
  let input = prompt("금액을 입력해주세요");

  match input.trim().parse() {
    Ok(amount) => amount,
    Err(e) => {
      print!("{}", e.to_string());
      return get_amount()
    }
}
}