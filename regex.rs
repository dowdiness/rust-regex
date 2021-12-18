enum State {
  Locked,
  Unlocked
}

enum Event {
  Push,
  Coin
}

fn next(state: State, event: Event) -> State {
  match state {
    State::Locked => todo!(),
    State::Unlocked => todo!(),
  }
}

fn main() {
  println!("Hello, World!");
}
