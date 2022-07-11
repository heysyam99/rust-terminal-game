use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
  let secret_number = 1;
  let mut input = String::new();

  println!("Welcome to ");
  loop {
    print!("Guess the number I'm thinking of: ");
    io::stdout().flush().unwrap();

    input.clear();

    io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");

      let guess: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
      };

      println!("\nYou guessed: {}", guess);

      match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
          println!("You win!");
          break;
        }
      }
  }
}
