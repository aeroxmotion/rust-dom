use std::any::Any;

use crate::webidl::types::{
  DOMString,
  UnsignedShort,
  DOMHighResTimeStamp,
  Boolean,
};

use super::{
  event::Event,
  event_target::EventTarget,
  event_traits::{IEvent, EventFlags},
  event_init::EventInit,
};

pub struct CustomEventInit {
  pub bubbles: Boolean,
  pub cancelable: Boolean,
  pub composed: Boolean,
  pub detail: Option<Box<dyn Any>>,
}

impl CustomEventInit {
  pub fn new() -> Self {
    Self {
      bubbles: false,
      cancelable: false,
      composed: false,
      detail: None,
    }
  }
}

pub struct CustomEvent {
  pub event: Event,

  // https://dom.spec.whatwg.org/#rref-for-dom-customevent-detail
  pub _detail: Option<Box<dyn Any>>,
}

impl IEvent for CustomEvent {
  fn type_(&self) -> &DOMString {
    self.event.type_()
  }

  fn target(&self) -> Option<&Box<dyn EventTarget>> {
    self.event.target()
  }

  fn current_target(&self) -> Option<&Box<dyn EventTarget>> {
    self.event.current_target()
  }

  fn composed_path(&self) -> Vec<&Box<dyn EventTarget>> {
    self.event.composed_path()
  }

  fn event_phase(&self) -> UnsignedShort {
    self.event.event_phase()
  }

  fn stop_propagation(&mut self) {
    self.event.stop_propagation()
  }

  fn cancel_bubble(&mut self, value: Option<Boolean>) -> Option<Boolean> {
    self.event.cancel_bubble(value)
  }

  fn stop_immediate_propagation(&mut self) {
    self.event.stop_immediate_propagation()
  }

  fn bubbles(&self) -> Boolean {
    self.event.bubbles()
  }

  fn cancelable(&self) -> Boolean {
    self.event.cancelable()
  }

  fn return_value(&mut self, value: Option<Boolean>) -> Option<Boolean> {
    self.event.return_value(value)
  }

  fn prevent_default(&mut self) {
    self.event.prevent_default()
  }

  fn default_prevented(&self) -> Boolean {
    self.event.default_prevented()
  }

  fn composed(&self) -> Boolean {
    self.event.composed()
  }

  fn is_trusted(&self) -> Boolean {
    self.event.is_trusted()
  }

  fn time_stamp(&self) -> DOMHighResTimeStamp {
    self.event.time_stamp()
  }

  fn init_event(&mut self, type_: &DOMString, bubbles: Boolean, cancelable: Boolean) {
    self.event.init_event(type_, bubbles, cancelable)
  }
}

impl CustomEvent {
  pub fn new(type_: &str, event_init_dict: Option<CustomEventInit>) -> Self {
    let event_init_dict = match event_init_dict {
      None => CustomEventInit::new(),
      Some(dict) => dict,
    };

    Self {
      event: Event::new(type_, EventInit {
        bubbles: event_init_dict.bubbles,
        cancelable: event_init_dict.cancelable,
        composed: event_init_dict.composed,
      }.into()),
      _detail: event_init_dict.detail,
    }
  }

  pub fn init_custom_event(&mut self, type_: &DOMString, bubbles: Boolean, cancelable: Boolean, detail: Option<Box<dyn Any>>) {
    if self.event._flags.is_set(EventFlags::DISPATCH) {
      return
    }

    self.event.init_event(type_, bubbles, cancelable);
    self._detail = detail;
  }
}
