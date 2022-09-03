use super::{
  event_target::EventTarget,
  event_init::EventInit,
};

enum Flags {
  StopImmediatePropagation = 1 << 1,
}

pub struct Event {
  pub typ: String,
  pub target: Option<Box<dyn EventTarget>>,
  pub src_element: Option<Box<dyn EventTarget>>,
  pub current_target: Option<Box<dyn EventTarget>>,

  pub event_phase: u8,

  /// Used as `stop propagation flag`
  pub cancel_bubble: bool,

  /// Used as `canceled flag`
  pub return_value: bool,

  pub bubbles: bool,
  pub cancelable: bool,
  pub default_prevented: bool,
  pub composed: bool,

  pub is_trusted: bool,
  pub time_stamp: f32,

  flags: u32,
}

impl Event {
  pub const NONE: u8 = 0;
  pub const CAPTURING_PHASE: u8 = 1;
  pub const AT_TARGET: u8 = 2;
  pub const BUBBLING_PHASE: u8 = 3;
}

impl Event {
  pub fn new(typ: &str, event_init_dict: EventInit) -> Event {
    Self {
      typ: typ.into(),
      target: None,
      src_element: None,
      current_target: None,
      event_phase: Self::NONE,
      cancel_bubble: false,
      bubbles: event_init_dict.bubbles,
      cancelable: event_init_dict.cancelable,
      return_value: false,
      default_prevented: false,
      composed: event_init_dict.composed,
      is_trusted: false,
      time_stamp: 0.0,
      flags: 0,
    }
  }

  pub fn init_event(&mut self, typ: &str, bubbles: bool, cancelable: bool) {
    self.typ = typ.into();
    self.bubbles = bubbles;
    self.cancelable = cancelable;
  }
}

impl Event {
  pub fn stop_propagation(&mut self) {
    self.cancel_bubble = true;
  }

  pub fn stop_immediate_propagation(&mut self) {
    self.flags |= Flags::StopImmediatePropagation as u32;
  }
}
