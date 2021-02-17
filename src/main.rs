use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  let f = &args[1];
  let r = &args[2];
  let repl = &args[3];

  let tests = repl
    .replace(r, ">>>")
    .lines()
    .map(|l| String::from("-- ") + l )
    .collect::<Vec<_>>()
    .join("\n");

  println!("-- | {}", f);
  println!("{}", tests);
}
