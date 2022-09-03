pub trait EventTarget {
  fn add_event_listener<'a>(&self, typ: &'a str, callback: fn() -> ()) -> String;
  fn remove_event_listener<'a>(&self, typ: &'a str, callback: fn() -> ()) -> String;
}
