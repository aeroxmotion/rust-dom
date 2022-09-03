pub struct EventInit {
  pub bubbles: bool,
  pub cancelable: bool,
  pub composed: bool,
}

impl EventInit {
  pub fn new() -> EventInit {
    EventInit {
      bubbles: false,
      cancelable: false,
      composed: false,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_create_an_empty_dict() {
    let event_init_dict = EventInit::new();

    assert_eq!(event_init_dict.bubbles, false);
    assert_eq!(event_init_dict.cancelable, false);
    assert_eq!(event_init_dict.composed, false);
  }
}
