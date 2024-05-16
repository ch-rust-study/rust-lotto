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

pub fn get_winning_numbers() -> Vec<i32> {
  let input = prompt("당첨번호를 ','로 구분하여 입력해주세요");
  let mut winning_numbers_buffer = Vec::<i32>::new();

  for num_str in input.trim().split(",") {
    match num_str.parse::<i32>() {
      Ok(n) => winning_numbers_buffer.push(n),
      Err(e) => {
        println!("{}", e.to_string());
        return get_winning_numbers();
      }
    }
  }

  if winning_numbers_buffer.len() != 6 {
    println!("당첨 번호는 6개를 입력해주세요.");
    return get_winning_numbers();
  }


  return winning_numbers_buffer;
}

pub fn get_bonus_number() -> i32 {
  let input = prompt("보너스 번호를 입력해주세요");

  match input.parse::<i32>() {
    Ok(n) => n,
    Err(e) => {
      println!("{}", e.to_string());
      return get_bonus_number();
    }
  }
}