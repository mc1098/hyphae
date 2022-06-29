mod html;
mod lev_distance;

use std::time::Duration;

pub use html::{
    format_html, format_html_with_closest, get_element_value, make_element_with_html_string,
    map_element_value, set_element_value,
};

pub use lev_distance::{closest, is_close};

use js_sys::Function;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen(module = "/js/hyphae-utils.js")]
extern "C" {
    fn wait_promise(ms: JsValue) -> js_sys::Promise;
    fn until_mutation(element: &JsValue, action: &Function, timeout: JsValue) -> js_sys::Promise;
}

/// Perform an action and await a DOM change with a timeout duration.
///
/// This function uses the MutationObserver in JS to track whether a change in the DOM has occurred
/// for the element given or it's subtree, this includes attribute changes.
///
/// The Future will wait until the allotted time for a change in the DOM
/// to occur. If no DOM change occurs then this function will panic.
pub async fn effect_dom<F>(element: &JsValue, action: F, timeout: Duration)
where
    F: Fn() + 'static,
{
    let timeout = timeout.as_millis().into();
    let function = Closure::wrap(Box::new(action) as Box<dyn Fn()>);
    JsFuture::from(until_mutation(
        element,
        function.as_ref().unchecked_ref(),
        timeout,
    ))
    .await
    .unwrap_throw();
}

/// Asynchronous wait for a given amount of ms.
///
/// This is a Rust Future which uses an underlying JS Promise and Timeout.
/// This can be useful to assert something has occurred, or not, after a given amount of time -
/// especially as you cannot use [sleep](std::thread::sleep) in a test using
/// [`wasm_bindgen_test`](wasm_bindgen_testhttps://crates.io/crates/wasm-bindgen-test/).
///
/// # Examples
/// ```no_run
///
/// use wasm_bindgen_test::*;
///
/// #[wasm_bindgen_test]
/// async fn some_test_that_requires_waiting() {
///     // setup..
///     // wait 500ms
///     hyphae_utils::wait_ms(500);
///     // some asserts..
/// }
/// ```
pub async fn wait_ms(ms: u32) {
    JsFuture::from(wait_promise(ms.into())).await.unwrap_throw();
}
