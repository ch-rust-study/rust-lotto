use std::io::stdin;

pub fn prompt(question: &str) -> String {
  println!("{}", question);

  let mut buffer = String::new();
  stdin().read_line(&mut buffer).unwrap();

  return buffer;
}