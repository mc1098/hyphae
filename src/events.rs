/*!
Convenience module for firing events to [`EventTarget`].

The goal of this module is to remove the boilerplate from firing web_sys events by providing
helper functions and traits for medium/high level actions.
*/
use wasm_bindgen::JsCast;
use web_sys::{
    EventTarget, HtmlInputElement, InputEvent, InputEventInit, KeyboardEvent, KeyboardEventInit,
    MouseEvent,
};

/// An enum for the possible event types for [`KeyboardEvent`]s.
#[derive(Clone, Copy)]
pub enum KeyEventType {
    /// The `keydown` event type.
    KeyDown,
    /// The `keyup` event type.
    KeyUp,
    /// The `keypress` event type
    KeyPress,
}

#[allow(clippy::from_over_into)]
impl Into<&str> for KeyEventType {
    fn into(self) -> &'static str {
        match self {
            KeyEventType::KeyDown => "keydown",
            KeyEventType::KeyUp => "keyup",
            KeyEventType::KeyPress => "keypress",
        }
    }
}

/**
Dispatches a single [`KeyboardEvent`] with the type and key provided to the event target.

See the list of keys available [here](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key/Key_Values).

# Examples
```no_run
let btn: HtmlButtonElement = // function to get button element
// can confirm by pressing enter on button
dispatch_key_event(&btn, KeyEventType::KeyPress, "Enter");
```
*/
pub fn dispatch_key_event(element: &EventTarget, event_type: KeyEventType, key: &str) {
    let mut event_init = KeyboardEventInit::new();
    event_init.key(key);
    let key_event =
        KeyboardEvent::new_with_keyboard_event_init_dict(event_type.into(), &event_init).unwrap();

    element.dispatch_event(&key_event).unwrap();
}

/**
Simulates typing a single key to the [`EventTarget`].

This will fire the following events on the target:
- `keydown` [`KeyboardEvent`]
- `keypress` [`KeyboardEvent`]
- `keyup` [`KeyboardEvent`]
- `input` [`InputEvent`]

# Examples
```no_run
let input: HtmlInputElement = // some function to get input element;
type_key(&input, "A");
assert_eq!("A", input.value());
```
*/
pub fn type_key(element: &EventTarget, key: &str) {
    for &key_event_type in [
        KeyEventType::KeyDown,
        KeyEventType::KeyPress,
        KeyEventType::KeyUp,
    ]
    .iter()
    {
        dispatch_key_event(element, key_event_type, key);
    }
    dispatch_input_event(element, &key);
}

/// Simulate typing a String to an implementation of this type.
pub trait SimTyping {
    /**
    Simulates typing multiple characters to Self.

    This will fire the following events for each character:
    - `keydown` [`KeyboardEvent`]
    - `keypress` [`KeyboardEvent`]
    - `keyup` [`KeyboardEvent`]
    - `input` [`InputEvent`]

    # Examples
    ```no_run
    let input: HtmlInputElement = // ..
    input.sim_typing("Hello, World!");
    assert_eq!("Hello, World!", input.value());
    ```
    */
    fn sim_typing(&self, value: &str)
    where
        Self: AsRef<EventTarget>;
}

impl SimTyping for EventTarget {
    fn sim_typing(&self, value: &str) {
        type_to(self, value);
    }
}

/**
Simulates typing multiple characters to the [`EventTarget`].

This will fire the following events for each character:
- `keydown` [`KeyboardEvent`]
- `keypress` [`KeyboardEvent`]
- `keyup` [`KeyboardEvent`]
- `input` [`InputEvent`]
*/
pub fn type_to(element: &EventTarget, value: &str) {
    for char in value.chars() {
        let ch = char.to_string();
        for &key_event_type in [
            KeyEventType::KeyDown,
            KeyEventType::KeyPress,
            KeyEventType::KeyUp,
        ]
        .iter()
        {
            dispatch_key_event(element, key_event_type, &ch);
        }
        dispatch_input_event(element, &ch);
    }
}

/// Enables firing a `dblclick` [`MouseEvent`].
pub trait DblClick {
    /**
    Fires a `dblclick` [`MouseEvent`] on this [`EventTarget`].

    # Examples
    ```no_run
    let btn: HtmlButtonElement = //..
    btn.dbl_click();
    ```
    */
    fn dbl_click(&self)
    where
        Self: AsRef<EventTarget>;
}

impl DblClick for EventTarget {
    fn dbl_click(&self) {
        let dbl_click_event = MouseEvent::new("dblclick").unwrap();
        assert!(self.dispatch_event(&dbl_click_event).unwrap());
    }
}

/**
Dispatches a [`InputEvent`] with the `data` given, to the event target.

Currently only supports [`HtmlInputElement`]!

Only use this if you need to trigger an `oninput` event listener - if you want to change the value
of the [`EventTarget`] you can just use the relative set method. For example the
HtmlInputElement::set_value function

_**Note:** Yew ignores [`InputEvent::data()`]_

# Examples
```no_run
let input: HtmlInputElement = // function to get input element
// enter value into input
dispatch_input_event(&input, "Hello, World!");
assert_eq!("Hello, World!", input.value());
```
*/
pub fn dispatch_input_event(element: &EventTarget, data: &str) {
    let input: &HtmlInputElement = element.unchecked_ref::<HtmlInputElement>();
    let mut value = input.value();
    value.push_str(data);
    // Force value update as Yew uses HtmlInputElement::value instead of InputEvent::data for oninput
    input.set_value(&value);
    let mut event_init = InputEventInit::new();
    event_init.data(Some(data));
    let input_event = InputEvent::new_with_event_init_dict("input", &event_init).unwrap();
    assert!(element.dispatch_event(&input_event).unwrap());
}

#[cfg(test)]
mod tests {

    use web_sys::{HtmlElement, HtmlInputElement, KeyboardEvent};
    use yew::{prelude::*, virtual_dom::test_render};

    struct KeyDemo {
        link: ComponentLink<Self>,
        last_key_pressed: Option<String>,
    }

    impl Component for KeyDemo {
        type Message = KeyboardEvent;
        type Properties = ();

        fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
            Self {
                link,
                last_key_pressed: None,
            }
        }

        fn update(&mut self, msg: Self::Message) -> ShouldRender {
            self.last_key_pressed = Some(msg.key());
            true
        }

        fn change(&mut self, _props: Self::Properties) -> ShouldRender {
            false
        }

        fn view(&self) -> Html {
            let default = "None".to_owned();
            let last_key_value = self.last_key_pressed.as_ref().unwrap_or(&default);
            html! {
                <>
                    <p>{ last_key_value }</p>
                    <label for="input">{ "input label" }</label>
                    <input id="input" placeholder="key" type="text" onkeydown=self.link.callback(|e| e) />
                </>
            }
        }
    }

    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use crate::{assert_text_content, TestRender};

    use super::*;
    use crate::queries::*;

    #[wasm_bindgen_test]
    fn sim_typing_to_input_and_enter_to_confirm() {
        let rendered = TestRender::new(test_render(html! {
            <KeyDemo />
        }));

        let last_key_value: HtmlElement = rendered.get_by_text("None").unwrap();
        let input: HtmlInputElement = rendered.get_by_placeholder_text("key").unwrap();

        assert_text_content!("None", last_key_value);

        dispatch_key_event(&input, KeyEventType::KeyDown, "Enter");

        assert_text_content!("Enter", last_key_value);

        input.sim_typing("hello");

        assert_text_content!("o", last_key_value);
        assert_eq!("hello", input.value());
    }

    struct InputDemo {
        link: ComponentLink<Self>,
        value: String,
    }

    impl Component for InputDemo {
        type Message = InputData;
        type Properties = ();

        fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
            Self {
                link,
                value: String::new(),
            }
        }

        fn update(&mut self, msg: Self::Message) -> ShouldRender {
            self.value = msg.value;
            true
        }

        fn change(&mut self, _props: Self::Properties) -> ShouldRender {
            false
        }

        fn view(&self) -> Html {
            let cb = |e| e;
            html! {
                <>
                    <input value=self.value.clone() placeholder="key" type="text" oninput=self.link.callback(cb) />
                </>
            }
        }
    }

    #[wasm_bindgen_test]
    fn type_to_input() {
        let rendered = TestRender::new(test_render(html! {
            <InputDemo />
        }));

        let input: HtmlInputElement = rendered.get_by_placeholder_text("key").unwrap();
        type_to(&input, "hello");

        assert_eq!("hello", input.value());
    }
}
