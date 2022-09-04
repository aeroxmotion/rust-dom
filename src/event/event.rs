use crate::webidl::types::{
  UnsignedShort,
  DOMString,
  Boolean,
  DOMHighResTimeStamp,
};

use super::{
  event_target::EventTarget,
  event_init::EventInit,
  event_traits::{IEvent, EventFlags},
};

pub struct PathTarget {
  pub invocation_target: Box<dyn EventTarget>,
  pub shadow_tree: bool,
  pub shadow_adjusted_target: Option<Box<dyn EventTarget>>,
  pub related_target: Option<Box<dyn EventTarget>>,
  pub touch_targets: Vec<Option<Box<dyn EventTarget>>>,
  pub root_closed_tree: bool,
  pub slot_closed_tree: bool,
}

pub struct Event {
  // https://dom.spec.whatwg.org/#event-target
  pub _target: Option<Box<dyn EventTarget>>,
  // https://dom.spec.whatwg.org/#event-relatedtarget
  pub _related_target: Option<Box<dyn EventTarget>>,
  // https://dom.spec.whatwg.org/#event-touch-target-list
  pub _touch_target_list: Vec<Option<Box<dyn EventTarget>>>,
  // https://dom.spec.whatwg.org/#event-path
  pub _path: Vec<PathTarget>,
  // associated flags (https://dom.spec.whatwg.org/#stop-propagation-flag)
  pub _flags: EventFlags,

  pub _type: DOMString,
  pub _current_target: Option<Box<dyn EventTarget>>,

  pub _event_phase: UnsignedShort,

  pub _bubbles: Boolean,
  pub _cancelable: Boolean,

  pub _is_trusted: Boolean,
  pub _time_stamp: DOMHighResTimeStamp,
}

impl Event {
  pub fn new(type_: &str, event_init_dict: Option<EventInit>) -> Event {
    let event_init_dict = match event_init_dict {
      None => EventInit::new(),
      Some(dict) => dict,
    };

    let mut event = Self {
      _type: type_.into(),
      _target: None,
      _related_target: None,
      _touch_target_list: vec![],
      _path: vec![],
      _flags: EventFlags(0),

      _current_target: None,

      _event_phase: Self::NONE,

      _bubbles: event_init_dict.bubbles,
      _cancelable: event_init_dict.cancelable,
      _is_trusted: false,
      _time_stamp: 0.0,
    };

    if event_init_dict.composed {
      event._flags.set(EventFlags::COMPOSED);
    }

    event
  }
}

impl IEvent for Event {
  fn type_(&self) -> &DOMString {
    &self._type
  }

  fn target(&self) -> Option<&Box<dyn EventTarget>> {
    self._target.as_ref()
  }

  fn current_target(&self) -> Option<&Box<dyn EventTarget>> {
    self._current_target.as_ref()
  }

  fn composed_path(&self) -> Vec<&Box<dyn EventTarget>> {
    let mut composed_path = vec![];
    let path = &self._path;

    if path.is_empty() {
      return composed_path
    }

    let current_target = self.current_target().unwrap();

    composed_path.push(current_target);

    let mut current_target_index = 0;
    let mut current_target_hidden_subtree_level = 0;
    let mut index = (path.len() as i32) - 1;

    while index >= 0 {
      let path_target = &path[index as usize];

      if path_target.root_closed_tree {
        current_target_hidden_subtree_level += 1;
      }

      if *path_target.invocation_target == **current_target {
        current_target_index = index;
        break;
      }

      if path_target.slot_closed_tree {
        current_target_hidden_subtree_level -= 1;
      }

      index -= 1;
    }

    let mut current_hidden_level = current_target_hidden_subtree_level;
    let mut max_hidden_level = current_target_hidden_subtree_level;

    index = current_target_index - 1;

    while index >= 0 {
      let path_target = &path[index as usize];

      if path_target.root_closed_tree {
        current_hidden_level += 1;
      }

      if current_hidden_level <= max_hidden_level {
        composed_path.insert(0, &path_target.invocation_target)
      }

      if path_target.slot_closed_tree {
        current_hidden_level -= 1;

        if current_hidden_level < max_hidden_level {
          max_hidden_level = current_hidden_level;
        }
      }

      index -= 1;
    }

    current_hidden_level = current_target_hidden_subtree_level;
    max_hidden_level = current_target_hidden_subtree_level;
    index = current_target_index + 1;

    while index < path.len() as i32 {
      let path_target = &path[index as usize];

      if path_target.slot_closed_tree {
        current_hidden_level += 1;
      }

      if current_hidden_level <= max_hidden_level {
        composed_path.push(&path_target.invocation_target)
      }

      if path_target.root_closed_tree {
        current_hidden_level -= 1;
        
        if current_hidden_level < max_hidden_level {
          max_hidden_level = current_hidden_level;
        }
      }

      index += 1;
    }

    composed_path
  }
 
  fn event_phase(&self) -> UnsignedShort {
    self._event_phase
  }

  fn stop_propagation(&mut self) {
    self._flags.set(EventFlags::STOP_PROPAGATION);
  }

  fn cancel_bubble(&mut self, value: Option<Boolean>) -> Option<Boolean> {
    match value {
      // Getter()
      None => Some(self._flags.is_set(EventFlags::STOP_PROPAGATION)),

      // Setter()
      Some(v) => {
        if v {
          self._flags.set(EventFlags::STOP_PROPAGATION);
        }

        None
      },
    }
  }

  fn stop_immediate_propagation(&mut self) {
    self._flags.set(
      EventFlags::STOP_PROPAGATION |
      EventFlags::STOP_IMMEDIATE_PROPAGATION
    );
  }

  fn bubbles(&self) -> Boolean {
    self._bubbles
  }

  fn cancelable(&self) -> Boolean {
    self._cancelable
  }

  fn return_value(&mut self, value: Option<Boolean>) -> Option<Boolean> {
    match value {
      // Getter()
      None => Some(self._flags.is_unset(EventFlags::CANCELED)),

      // Setter()
      // https://dom.spec.whatwg.org/#set-the-canceled-flag
      Some(v) => {
        if !v && self._cancelable && self._flags.is_unset(EventFlags::IN_PASSIVE_LISTENER) {
          self._flags.set(EventFlags::CANCELED);
        }

        None
      },
    }
  }

  fn prevent_default(&mut self) {
    self._flags.set(EventFlags::CANCELED);
  }

  fn default_prevented(&self) -> Boolean {
    self._flags.is_set(EventFlags::CANCELED)
  }

  fn composed(&self) -> Boolean {
    self._flags.is_set(EventFlags::COMPOSED)
  }

  fn time_stamp(&self) -> DOMHighResTimeStamp {
    self._time_stamp
  }

  // https://dom.spec.whatwg.org/#ref-for-dom-event-initevent
  fn init_event(&mut self, type_: &DOMString, bubbles: Boolean, cancelable: Boolean) {
    if self._flags.is_set(EventFlags::DISPATCH) {
      return
    }

    // https://dom.spec.whatwg.org/#concept-event-initialize
    self._flags.set(EventFlags::INITIALIZED);
    self._flags.remove(
      EventFlags::STOP_PROPAGATION |
      EventFlags::STOP_IMMEDIATE_PROPAGATION |
      EventFlags::CANCELED
    );

    self._is_trusted = false;
    self._target = None;
    self._type = type_.into();
    self._bubbles = bubbles;
    self._cancelable = cancelable;
  }
}
