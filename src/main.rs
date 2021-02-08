use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  let text = &args[1];

  let tests = text
    .replace("*Assignment05", ">>")
    .lines()
    .map(|l| String::from("-- ") + l )
    .collect::<Vec<_>>()
    .join("\n");

  println!("{}", tests);
}
