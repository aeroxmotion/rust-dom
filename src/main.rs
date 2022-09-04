use crate::event::event::Event;

mod event;

fn main() {
  let mut ev = Event::new("test", None);

  ev.return_value(None);

  println!("Hello, world!");
}
