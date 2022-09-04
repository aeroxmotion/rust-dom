pub trait EventTarget {
  fn add_event_listener<'a>(&self, typ: &'a str, callback: fn() -> ());
  fn remove_event_listener<'a>(&self, typ: &'a str, callback: fn() -> ());
}

impl PartialEq for dyn EventTarget {
  fn eq(&self, other: &Self) -> bool {
    self == other
  }
}
