use crate::event::{event::Event, event_traits::IEvent};

mod event;
mod webidl;

fn main() {
  let mut ev = Event::new("test", None);

  ev.return_value(None);

  println!("Hello, world!");
}
