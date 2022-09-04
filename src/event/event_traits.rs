use crate::webidl::types::{
  DOMString,
  UnsignedShort,
  DOMHighResTimeStamp,
  Boolean,
};

use super::event_target::EventTarget;

pub struct EventFlags(pub UnsignedShort);

impl EventFlags {
  // https://dom.spec.whatwg.org/#stop-propagation-flag
  pub const STOP_PROPAGATION: UnsignedShort = 1;
  // https://dom.spec.whatwg.org/#stop-immediate-propagation-flag
  pub const STOP_IMMEDIATE_PROPAGATION: UnsignedShort = 1 << 1;
  // https://dom.spec.whatwg.org/#canceled-flag
  pub const CANCELED: UnsignedShort = 1 << 2;
  // https://dom.spec.whatwg.org/#in-passive-listener-flag
  pub const IN_PASSIVE_LISTENER: UnsignedShort = 1 << 3;
  // https://dom.spec.whatwg.org/#composed-flag
  pub const COMPOSED: UnsignedShort = 1 << 4;
  // https://dom.spec.whatwg.org/#initialized-flag
  pub const INITIALIZED: UnsignedShort = 1 << 5;
  // https://dom.spec.whatwg.org/#dispatch-flag
  pub const DISPATCH: UnsignedShort = 1 << 6;

  // Flag operations
  pub fn set(&mut self, flags: UnsignedShort) {
    self.0 |= flags;
  }

  pub fn remove(&mut self, flags: UnsignedShort) {
    self.0 ^= flags;
  }

  pub fn is_set(&self, flag: UnsignedShort) -> Boolean {
    self.0 & flag != 0
  }

  pub fn is_unset(&self, flag: UnsignedShort) -> Boolean {
    !self.is_set(flag)
  }
}

pub trait IEvent {
  // https://dom.spec.whatwg.org/#ref-for-dom-event-type%E2%91%A1
  fn type_(&self) -> &DOMString;
  // https://dom.spec.whatwg.org/#ref-for-dom-event-target%E2%91%A1
  fn target(&self) -> Option<&Box<dyn EventTarget>>;
  // https://dom.spec.whatwg.org/#ref-for-dom-event-srcelement
  fn src_element(&self) -> Option<&Box<dyn EventTarget>> { self.target() }
  // https://dom.spec.whatwg.org/#ref-for-dom-event-currenttarget%E2%91%A0
  fn current_target(&self) -> Option<&Box<dyn EventTarget>>;
  // https://dom.spec.whatwg.org/#ref-for-dom-event-composedpath
  fn composed_path(&self) -> Vec<&Box<dyn EventTarget>>;

  // https://dom.spec.whatwg.org/#ref-for-dom-event-none
  const NONE: UnsignedShort = 0;
  // https://dom.spec.whatwg.org/#ref-for-dom-event-capturing_phase%E2%91%A0
  const CAPTURING_PHASE: UnsignedShort = 1;
  // https://dom.spec.whatwg.org/#ref-for-dom-event-at_target%E2%91%A0
  const AT_TARGET: UnsignedShort = 2;
  // https://dom.spec.whatwg.org/#ref-for-dom-event-bubbling_phase%E2%91%A0
  const BUBBLING_PHASE: UnsignedShort = 3;
  // https://dom.spec.whatwg.org/#ref-for-dom-event-eventphase%E2%91%A1
  fn event_phase(&self) -> UnsignedShort;

  // https://dom.spec.whatwg.org/#ref-for-dom-event-stoppropagation
  fn stop_propagation(&mut self);
  // https://dom.spec.whatwg.org/#ref-for-dom-event-cancelbubble
  fn cancel_bubble(&mut self, value: Option<Boolean>) -> Option<Boolean>;
  // https://dom.spec.whatwg.org/#ref-for-dom-event-stopimmediatepropagation
  fn stop_immediate_propagation(&mut self);

  // https://dom.spec.whatwg.org/#ref-for-dom-event-bubbles%E2%91%A0
  fn bubbles(&self) -> Boolean;
  // https://dom.spec.whatwg.org/#ref-for-dom-event-cancelable
  fn cancelable(&self) -> Boolean;
  // https://dom.spec.whatwg.org/#ref-for-dom-event-returnvalue
  fn return_value(&mut self, value: Option<Boolean>) -> Option<Boolean>;
  // https://dom.spec.whatwg.org/#ref-for-dom-event-preventdefault%E2%91%A0
  fn prevent_default(&mut self);
  // https://dom.spec.whatwg.org/#ref-for-dom-event-defaultprevented
  fn default_prevented(&self) -> Boolean;
  // https://dom.spec.whatwg.org/#ref-for-dom-event-composed
  fn composed(&self) -> Boolean;

  // https://dom.spec.whatwg.org/#ref-for-dom-event-istrusted
  fn is_trusted(&self) -> Boolean {
    // TODO: Do we really need an implementation of this?
    false
  }
  // https://dom.spec.whatwg.org/#ref-for-dom-event-timestamp
  fn time_stamp(&self) -> DOMHighResTimeStamp;

  // https://dom.spec.whatwg.org/#ref-for-dom-event-initevent
  fn init_event(&mut self, type_: &DOMString, bubbles: Boolean, cancelable: Boolean);
}
