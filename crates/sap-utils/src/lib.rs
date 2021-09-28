mod html;
mod lev_distance;

pub use html::{format_html, format_html_with_closest, get_element_value, set_element_value};
pub use lev_distance::{closest, is_close};

use js_sys::Function;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen(module = "/js/sap-utils.js")]
extern "C" {
    fn wait_promise(ms: JsValue) -> js_sys::Promise;
    fn until_mutation(element: &JsValue, action: &Function, timeout: JsValue) -> js_sys::Promise;
}

/**
Perform an action and await a DOM change with an optional timeout.

This function uses the MutationObserver in JS to track whether a change in the DOM has occurred
for the element given or it's subtree, this includes attribute changes.

When a timeout is given, the Future will wait until the allotted time for a change in the DOM
to occur. If no DOM change occurs then this function will panic.

*/
pub fn effect_dom<F>(element: &JsValue, action: F, timeout_ms: Option<u32>) -> JsFuture
where
    F: Fn() + 'static,
{
    let timeout = match timeout_ms {
        Some(ms) => ms.into(),
        None => JsValue::UNDEFINED,
    };
    let function = Closure::wrap(Box::new(action) as Box<dyn Fn()>);
    JsFuture::from(until_mutation(
        element,
        function.as_ref().unchecked_ref(),
        timeout,
    ))
}

/**
Asynchronous wait for a given amount of ms.

This is a Rust Future which uses an underlying JS Promise and Timeout.
This can be useful to assert something has occurred, or not, after a given amount of time -
especially as you cannot use [sleep](std::thread::sleep) in a test using
[`wasm_bindgen_test`](wasm_bindgen_testhttps://crates.io/crates/wasm-bindgen-test/).

# Examples
```no_run

use wasm_bindgen_test::*;

#[wasm_bindgen_test]
async fn some_test_that_requires_waiting() {
    // setup..
    // wait 500ms - unwrap required
    sap_utils::wait_ms(500).await.expect("Underlying JS not to throw exception");
    // some asserts..
}
```
*/
pub async fn wait_ms(ms: u32) -> Result<(), JsValue> {
    let _ = JsFuture::from(wait_promise(ms.into())).await?;
    Ok(())
}
