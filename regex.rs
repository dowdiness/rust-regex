use std::io::{self, BufRead};
use std::io::Write;
// use std::fmt;

#[derive(Debug)]
enum State {
  Locked,
  Unlocked
}

#[derive(Debug)]
enum Event {
  Push,
  Coin
}

// Debug trait ではなくDisplay traitを使う場合
// impl fmt::Display for State {
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//     match self {
//       State::Locked => write!(f, "State::Locked"),
//       State::Unlocked => write!(f, "State::Unlocked"),
//     }
//   }
// }

fn next(state: State, event: Event) -> State {
  match state {
    State::Locked => match event {
      Event::Push => State::Locked,
      Event::Coin => State::Unlocked,
    },
    State::Unlocked => match event {
      Event::Push => State::Locked,
      Event::Coin => State::Unlocked,
    },
  }
}

fn main() {
  let mut state = State::Locked;
  print!("> ");
  io::stdout().flush().unwrap();
  for line in io::stdin().lock().lines() {
    match line.unwrap().as_str() {
      "coin" => {
        state = next(state, Event::Coin);
      },
      "push" => {
        state = next(state, Event::Push);
      },
      "quit" => {
        return
      },
      unknown => {
        eprintln!("ERROR: unknown event {}", unknown)
      },
    }
    println!("State : {:?}", state);
    print!("> ");
    io::stdout().flush().unwrap();
  }
}
