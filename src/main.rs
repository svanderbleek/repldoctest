use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  let f = &args[1];
  let repl = &args[2];

  let tests = repl
    .replace("*Assignment05", ">>")
    .lines()
    .map(|l| String::from("-- ") + l )
    .collect::<Vec<_>>()
    .join("\n");

  println!("-- | {}", f);
  println!("{}", tests);
}
