/*!
Convenience module for firing events to [`EventTarget`].

The goal of this module is to remove the boilerplate from firing [`web_sys`] events by providing
helper functions and traits for medium/high level actions.
*/
mod key;

pub use key::*;

use web_sys::{
    Event, EventInit, EventTarget, InputEvent, InputEventInit, KeyboardEvent, KeyboardEventInit,
    MouseEvent, MouseEventInit,
};

/**
Dispatches a single [`KeyboardEvent`] with the type and key provided to the event target.

Uses the [`KeyEventType`] and [`Key`] enum to provide type safe options - this avoids typos causing
tests to fail.

The [list of keys available](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key/Key_Values).

# Examples
## Control Key
[`Key`] has a variant for most of the keys listed above (see next example for typed chars).
```
use hyphae::event::*;
use web_sys::HtmlButtonElement;

# fn control_key_example(btn: HtmlButtonElement) {
let btn: HtmlButtonElement = // function to get button element
    # btn;
// can confirm by pressing enter on button
dispatch_key_event(&btn, KeyEventType::KeyPress, Key::Enter);
# }
```

## Char literals
A [`char`] can be accepted and will be the equivalent to using [`Key::Lit`] variant with the [`char`]
value.
```
use hyphae::event::*;
use web_sys::HtmlInputElement;

# fn char_literal_example(input: HtmlInputElement) {
let input: HtmlInputElement = // get input
    # input;
dispatch_key_event(&input, KeyEventType::KeyPress, 'a');
# }
```
*/
pub fn dispatch_key_event<K>(element: &EventTarget, event_type: KeyEventType, key: K)
where
    K: Into<Key>,
{
    let mut event_init = KeyboardEventInit::new();
    event_init.bubbles(true);
    event_init.key(&key.into().to_string());
    let key_event =
        KeyboardEvent::new_with_keyboard_event_init_dict(event_type.into(), &event_init).unwrap();

    element.dispatch_event(&key_event).unwrap();
}

/**
A simple simulation of typing a single key to the [`EventTarget`].

This will fire the following events, in this order, on the target:
- `keydown` [`KeyboardEvent`]
- `keypress` [`KeyboardEvent`]
- `keyup` [`KeyboardEvent`]
- `input` [`InputEvent`]

# Examples
```
use hyphae::event::*;
use web_sys::HtmlInputElement;

# fn type_key_example(input: HtmlInputElement) {
let input: HtmlInputElement = // some function to get input element;
    # input;
type_key(&input, 'A');
assert_eq!("A", input.value());
# }
```
*/
pub fn type_key<K>(element: &EventTarget, key: K)
where
    K: Into<Key>,
{
    let key = key.into();
    type_key_only(element, key);
    if key.is_visible() {
        let mut init = InputEventInit::new();
        init.data(Some(&key.to_string()));
        init.bubbles(true);
        init.input_type("insertText");
        dispatch_input_event(element, init);
    }
}

/**
A simple simulation of typing a multiple keys to the [`EventTarget`].

This will fire the following events, in this order, on the target for each key:
- `keydown` [`KeyboardEvent`]
- `keypress` [`KeyboardEvent`]
- `keyup` [`KeyboardEvent`]
- `input` [`InputEvent`] if the key is visible

# Examples
```
use hyphae::event::*;
use web_sys::HtmlInputElement;

# fn type_key_example(input: HtmlInputElement) {
let input: HtmlInputElement = // some function to get input element;
    # input;
type_keys(&input, "abc");
assert_eq!("abc", input.value());
# }
```
*/
pub fn type_keys<K>(element: &EventTarget, keys: K)
where
    K: Into<Keys>,
{
    let keys = keys.into();
    for key in keys.iter().copied() {
        type_key(element, key);
    }
}

fn type_key_only(element: &EventTarget, key: Key) {
    for &key_event_type in [
        KeyEventType::KeyDown,
        KeyEventType::KeyPress,
        KeyEventType::KeyUp,
    ]
    .iter()
    {
        dispatch_key_event(element, key_event_type, key);
    }
}

/// A simple simulation of typing multiple [`Key`]s to the [`EventTarget`].
///
/// This will fire the following events, in this order, for each [`Key`]:
/// - `keydown` [`KeyboardEvent`]
/// - `keypress` [`KeyboardEvent`]
/// - `keyup` [`KeyboardEvent`]
/// - `input` [`InputEvent`]
///
/// ```
/// use hyphae::{event::*, type_to};
/// use web_sys::HtmlInputElement;
///
/// # fn type_to_example(input: HtmlInputElement) {
/// let input: HtmlInputElement = // some query to get input element
///     # input;
/// type_to!(input, "Hello,", " World!");
/// assert_eq!("Hello, World!", input.value());
/// # }
///
/// ```
#[macro_export]
macro_rules! type_to {
    ($element: ident, $($into_keys:expr),+) => {
        let mut keys: Vec<hyphae::event::Key> = vec![];
        $(
            let mut ks: hyphae::event::Keys = $into_keys.into();
            keys.append(&mut ks);
        )+
        hyphae::event::type_keys(&$element, keys);
    };
}

/// Enables firing a `dblclick` [`MouseEvent`].
pub trait DblClick {
    /**
    Fires a `dblclick` [`MouseEvent`] on this [`EventTarget`].

    # Examples
    ```
    use hyphae::event::DblClick;
    use web_sys::HtmlButtonElement;

    # fn dbl_click_example(btn: HtmlButtonElement) {
    let btn: HtmlButtonElement = // get button from query
        # btn;
    btn.dbl_click();
    # }
    ```
    */
    fn dbl_click(&self)
    where
        Self: AsRef<EventTarget>;
}

impl DblClick for EventTarget {
    fn dbl_click(&self) {
        let mut event_init = MouseEventInit::new();
        event_init.bubbles(true);
        let dbl_click_event = MouseEvent::new("dblclick").unwrap();
        assert!(
            self.dispatch_event(&dbl_click_event).unwrap(),
            "expected dblclick event to be fired."
        );
    }
}

/**
Dispatches a [`InputEvent`] with the `data` given, to the event target.

Input events can only be fired on the following:
- [`HtmlInputElement`](web_sys::HtmlInputElement)
- [`HtmlSelectElement`](web_sys::HtmlSelectElement)
- [`HtmlTextAreaElement`](web_sys::HtmlTextAreaElement)

Using the function on other elements will do nothing!

Only use this if you need to trigger an `oninput` event listener - if you want to change the value
of the [`EventTarget`] you can just use the relative set value method.

# Examples
```
use hyphae::event::dispatch_input_event;
use web_sys::{HtmlInputElement, InputEventInit};

# fn dispatch_input_event_example(input: HtmlInputElement) {
let input: HtmlInputElement = // function to get input element
    # input;
// enter value into input
let mut init = InputEventInit::new();
init.data(Some("Hello World!"));
init.input_type("insertText");

dispatch_input_event(&input, init);
assert_eq!("Hello, World!", input.value());
# }
```
*/
pub fn dispatch_input_event(element: &EventTarget, data: InputEventInit) {
    let input_event = InputEvent::new_with_event_init_dict("input", &data).unwrap();
    let data = input_event.data();
    // if let Some(data) = data {
    //     let mut value = hyphae_utils::get_element_value(element).unwrap();
    //     value.push_str(&data);
    //     hyphae_utils::set_element_value(element, value);
    // }
    if let Some(data) = data.as_ref() {
        hyphae_utils::map_element_value(element, |mut value| {
            value.push_str(data);
            value
        });
    }
    assert!(element.dispatch_event(&input_event).unwrap());
}

/// Enables dispatching a bubbling `change` event from an EventTarget
pub trait EventTargetChanged {
    /**
    Dispatches a change [`Event`] on this [`EventTarget`]

    # Examples
    ```
    use hyphae::event::EventTargetChanged;
    use web_sys::HtmlInputElement;

    # fn dispatch_input_event_example(input: HtmlInputElement) {
    let input: HtmlInputElement = // function to get input element
        # input;
    // dispatch "change" event
    input.changed();
    # }
    ```
    */
    fn changed(&self);
}

impl EventTargetChanged for EventTarget {
    fn changed(&self) {
        let mut event_init = EventInit::new();
        event_init.bubbles(true);
        let change_event = Event::new_with_event_init_dict("change", &event_init).unwrap();
        assert!(self.dispatch_event(&change_event).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    use std::cell::Cell;

    use wasm_bindgen::{prelude::Closure, JsCast};
    use web_sys::{Document, HtmlElement, HtmlInputElement, KeyboardEvent};

    use hyphae::{prelude::*, QueryElement};
    use hyphae_utils::make_element_with_html_string;

    macro_rules! wasm_closure {
        (|_: $t:ty| $expr:expr) => {
            FunctionClosure(Closure::<dyn Fn($t)>::wrap(Box::new(|_: $t| $expr)))
        };
        (move |_: $t:ty| $expr:expr) => {
            FunctionClosure(Closure::<dyn Fn($t)>::wrap(Box::new(move |_: $t| $expr)))
        };
        (| $($v:ident: $t:ty),* | $expr:expr) => {
            FunctionClosure(Closure::<dyn Fn($($t),*)>::wrap(Box::new(|$($v: $t),*| $expr)))
        };
        (move | $($v:ident: $t:ty),* | $expr:expr) => {
            FunctionClosure(Closure::<dyn Fn($($t),*)>::wrap(Box::new(move |$($v: $t),*| $expr)))
        };
    }

    struct FunctionClosure<T: ?Sized>(Closure<T>);

    impl<T: ?Sized> std::ops::Deref for FunctionClosure<T> {
        type Target = js_sys::Function;

        fn deref(&self) -> &Self::Target {
            self.0.as_ref().unchecked_ref()
        }
    }

    fn global_document() -> Document {
        web_sys::window()
            .expect("No global window object")
            .document()
            .expect("No global document object")
    }

    #[wasm_bindgen_test]
    fn sim_typing_to_input_and_enter_to_confirm() {
        // setup

        let rendered: QueryElement = make_element_with_html_string(
            r#"
            <p id="key-value">None</p>
            <label for="input">input label</label>
            <input id="input" placeholder="key" type="text" />
        "#,
        )
        .into();

        let document = global_document();

        let key_value_output = document
            .get_element_by_id("key-value")
            .expect("no element with 'key-value' id found")
            .unchecked_into::<HtmlElement>();

        let input = document
            .get_element_by_id("input")
            .expect("no element with `input` id found");

        let closure = wasm_closure!(move |e: KeyboardEvent| {
            key_value_output.set_inner_text(&e.key());
        });

        input
            .add_event_listener_with_callback("keydown", &closure)
            .unwrap();

        // asserts

        let last_key_value: HtmlElement = rendered.get_by_text("None").unwrap();
        let input: HtmlInputElement = rendered.get_by_placeholder_text("key").unwrap();

        assert_text_content!("None", last_key_value);

        dispatch_key_event(&input, KeyEventType::KeyDown, Key::Enter);

        assert_text_content!("Enter", last_key_value);

        type_to!(input, "hello");

        assert_text_content!("o", last_key_value);
        assert_eq!("hello", input.value());

        dispatch_key_event(&input, KeyEventType::KeyDown, '🎉');

        assert_text_content!('🎉', last_key_value);
    }

    #[wasm_bindgen_test]
    fn type_to_input() {
        let rendered: QueryElement = make_element_with_html_string(
            r#"
            <input placeholder="key" type="text" />
        "#,
        )
        .into();

        let input: HtmlInputElement = rendered.get_by_placeholder_text("key").unwrap();
        type_to!(input, "hello");

        assert_eq!("hello", input.value());
    }

    #[wasm_bindgen_test]
    fn trigger_on_change_event() {
        thread_local! {
            static FLAG: Cell<bool> = Default::default();
        }

        let rendered: QueryElement = make_element_with_html_string(
            r#"
            <input placeholder="key" type="text" />
        "#,
        )
        .into();

        let input: HtmlInputElement = rendered.get_by_placeholder_text("key").unwrap();

        let listener = wasm_closure!(move |_: Event| {
            FLAG.with(|v| v.set(true));
        });

        rendered
            .add_event_listener_with_callback("change", &listener)
            .unwrap();

        input.changed();

        assert!(FLAG.with(|v| v.get()));

        // clean up
        rendered
            .remove_event_listener_with_callback("change", &listener)
            .unwrap();
    }
}
