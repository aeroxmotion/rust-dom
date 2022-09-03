use crate::event::{
  event::Event,
  event_init::EventInit,
};

mod event;

fn main() {
  let mut ev = Event::new("test", EventInit::new());

  ev.stop_propagation();
  ev.stop_immediate_propagation();

  println!("Hello, world!");
}
