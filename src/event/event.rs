use super::{
  event_target::EventTarget,
  event_init::EventInit,
};

pub enum Flags {
  // https://dom.spec.whatwg.org/#stop-propagation-flag
  StopPropagation = 1,
  // https://dom.spec.whatwg.org/#stop-immediate-propagation-flag
  StopImmediatePropagation = 1 << 1,
  // https://dom.spec.whatwg.org/#canceled-flag
  Canceled = 1 << 2,
  // https://dom.spec.whatwg.org/#in-passive-listener-flag
  InPassiveListener = 1 << 3,
  // https://dom.spec.whatwg.org/#composed-flag
  Composed = 1 << 4,
  // https://dom.spec.whatwg.org/#initialized-flag
  Initialized = 1 << 5,
  // https://dom.spec.whatwg.org/#dispatch-flag
  Dispatch = 1 << 6,
}

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
  pub _flags: u8,

  pub _type: String,
  pub _current_target: Option<Box<dyn EventTarget>>,

  pub _event_phase: u8,

  pub _bubbles: bool,
  pub _cancelable: bool,

  pub _is_trusted: bool,
  pub _time_stamp: f64,
}

impl Event {
  pub const NONE: u8 = 0;
  pub const CAPTURING_PHASE: u8 = 1;
  pub const AT_TARGET: u8 = 2;
  pub const BUBBLING_PHASE: u8 = 3;
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
      _flags: 0,

      _current_target: None,

      _event_phase: Self::NONE,

      _bubbles: event_init_dict.bubbles,
      _cancelable: event_init_dict.cancelable,
      _is_trusted: false,
      _time_stamp: 0.0,
    };

    if event_init_dict.composed {
      event._flags |= Flags::Composed as u8;
    }

    event
  }
}

// https://dom.spec.whatwg.org/#dom-event-type
// readonly attribute DOMString type
impl Event {
  pub fn type_(&self) -> &str {
    &self._type
  }
}

// https://dom.spec.whatwg.org/#dom-event-target
// readonly attribute EventTarget? target
impl Event {
  pub fn target(&self) -> Option<&Box<dyn EventTarget>> {
    self._target.as_ref()
  }
}

// https://dom.spec.whatwg.org/#dom-event-srcelement
// readonly attribute EventTarget? srcElement (legacy)
impl Event {
  pub fn src_element(&self) -> Option<&Box<dyn EventTarget>> {
    self.target()
  }
}

// https://dom.spec.whatwg.org/#dom-event-currenttarget
// readonly attribute EventTarget? currentTarget
impl Event {
  pub fn current_target(&self) -> Option<&Box<dyn EventTarget>> {
    self._current_target.as_ref()
  }
}

// https://dom.spec.whatwg.org/#dom-event-composedpath
// sequence<EventTarget> composedPath()
impl Event {
  pub fn composed_path(&self) -> Vec<&Box<dyn EventTarget>> {
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
}

// https://dom.spec.whatwg.org/#dom-event-eventphase
// readonly attribute unsigned short eventPhase
impl Event {
  pub fn event_phase(&self) -> u8 {
    self._event_phase
  }
}

// https://dom.spec.whatwg.org/#dom-event-stoppropagation
// undefined stopPropagation()
impl Event {
  pub fn stop_propagation(&mut self) {
    self._flags |= Flags::StopPropagation as u8;
  }
}

// https://dom.spec.whatwg.org/#dom-event-cancelbubble
// attribute boolean cancelBubble
impl Event {
  pub fn cancel_bubble(&mut self, value: Option<bool>) -> Option<bool> {
    match value {
      // Getter()
      None => Some(self._flags & Flags::StopPropagation as u8 != 0),

      // Setter()
      Some(v) => {
        if v {
          self._flags |= Flags::StopPropagation as u8;
        }

        None
      },
    }
  }
}

// https://dom.spec.whatwg.org/#dom-event-stopimmediatepropagation
// undefined stopImmediatePropagation()
impl Event {
  pub fn stop_immediate_propagation(&mut self) {
    self._flags |= Flags::StopPropagation as u8 | Flags::StopImmediatePropagation as u8;
  }
}

// https://dom.spec.whatwg.org/#dom-event-bubbles
// readonly attribute boolean bubbles
impl Event {
  pub fn bubbles(&self) -> bool {
    self._bubbles
  }
}

// https://dom.spec.whatwg.org/#dom-event-cancelable
// readonly attribute boolean cancelable
impl Event {
  pub fn cancelable(&self) -> bool {
    self._cancelable
  }
}

// https://dom.spec.whatwg.org/#dom-event-returnvalue
// attribute boolean returnValue
impl Event {
  pub fn return_value(&mut self, value: Option<bool>) -> Option<bool> {
    match value {
      // Getter()
      None => Some(self._flags & Flags::Canceled as u8 == 0),

      // Setter()
      // https://dom.spec.whatwg.org/#set-the-canceled-flag
      Some(v) => {
        if !v && self._cancelable && self._flags & Flags::InPassiveListener as u8 == 0 {
          self._flags |= Flags::Canceled as u8;
        }

        None
      },
    }
  }
}

// https://dom.spec.whatwg.org/#dom-event-preventdefault
// undefined preventDefault()
impl Event {
  pub fn prevent_default(&mut self) {
    self._flags |= Flags::Canceled as u8;
  }
}

// https://dom.spec.whatwg.org/#dom-event-defaultprevented
// readonly attribute boolean defaultPrevented
impl Event {
  pub fn default_prevented(&self) -> bool {
    self._flags & Flags::Canceled as u8 != 0
  }
}

// https://dom.spec.whatwg.org/#dom-event-composed
// readonly attribute boolean composed
impl Event {
  pub fn composed(&self) -> bool {
    self._flags & Flags::Composed as u8 != 0
  }
}

// https://dom.spec.whatwg.org/#dom-event-istrusted
// [LegacyUnforgeable] readonly attribute boolean isTrusted
impl Event {
  pub fn is_trusted(&self) -> bool {
    self._is_trusted
  }
}

// https://dom.spec.whatwg.org/#dom-event-timestamp
// readonly attribute DOMHighResTimeStamp timeStamp
impl Event {
  pub fn time_stamp(&self) -> f64 {
    self._time_stamp
  }
}

// https://dom.spec.whatwg.org/#dom-event-initevent
// undefined initEvent(DOMString type, optional boolean bubbles = false, optional boolean cancelable = false) (legacy)
impl Event {
  pub fn init_event(&mut self, type_: &str, bubbles: bool, cancelable: bool) {
    if self._flags & Flags::Dispatch as u8 != 0 {
      return
    }

    // https://dom.spec.whatwg.org/#concept-event-initialize
    self._flags |= Flags::Initialized as u8;
    self._flags ^=
      Flags::StopPropagation as u8 |
      Flags::StopImmediatePropagation as u8 |
      Flags::Canceled as u8;

    self._is_trusted = false;
    self._target = None;
    self._type = type_.into();
    self._bubbles = bubbles;
    self._cancelable = cancelable;
  }
}
