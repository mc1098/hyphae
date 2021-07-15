#![warn(missing_docs)]
/*!
# Sap Mock

Provides simple mocks for JS APIs.

_Work in Progress_
*/

use js_sys::{Function, Uint8Array};
use serde::Serialize;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen(module = "/js/mock.js")]
extern "C" {
    fn mock_fetch_resolve(value: JsValue) -> JsValue;
    fn mock_fetch_error(code: JsValue, reason: JsValue) -> JsValue;
    fn restore_fetch(original_fetch: &JsValue);
    fn wait_promise(ms: JsValue) -> js_sys::Promise;
    fn until_mutation(element: &JsValue, action: &Function, timeout: JsValue) -> js_sys::Promise;

    fn mock_websocket(conn_delay: JsValue) -> RawWebSocketController;

    type RawWebSocketController;
    #[wasm_bindgen(method, getter = is_opened)]
    fn is_opened(this: &RawWebSocketController) -> bool;
    #[wasm_bindgen(method, getter = last_message)]
    fn last_message(this: &RawWebSocketController) -> JsValue;
    #[wasm_bindgen(method, getter = last_message_type)]
    fn last_message_type(this: &RawWebSocketController) -> JsValue;
    #[wasm_bindgen(method, getter = original_ws)]
    fn original_ws(this: &RawWebSocketController) -> JsValue;

    #[wasm_bindgen(method)]
    fn send(this: &RawWebSocketController, data: &JsValue);
    #[wasm_bindgen(method)]
    fn error(this: &RawWebSocketController, message: &JsValue);
    #[wasm_bindgen(method)]
    fn close(this: &RawWebSocketController, code: JsValue, reason: JsValue);
    #[wasm_bindgen(method)]
    fn restore(this: &RawWebSocketController);

}

// @TODO: Provide a typed interface to avoid users having to deal with JsValue

/// Controller for a mock WebSocket
///
/// Use this controller to send messages to the mock WebSocket or assert the last message sent by
/// the mock WebSocket
///
/// Note: When this is dropped the mock WebSocket will receive an onclose event, if the close function
/// hasn't already been called, and this will restore the normal WebSocket definition.
#[must_use]
pub struct WebSocketController(RawWebSocketController);

impl WebSocketController {
    /// Send a string message to the mock WebSocket.
    pub fn send_with_str(&self, data: &str) {
        self.0.send(&data.into());
    }

    /// Send a binary message to the mock WebSocket.
    pub fn send_with_u8_array(&self, data: &[u8]) {
        self.0.send(&Uint8Array::from(data));
    }

    /// Get last message sent by the mock WebSocket as a [`String`].
    pub fn get_last_message_as_string(&self) -> Option<String> {
        self.0.last_message().as_string()
    }

    /// Get last message sent by the mock WebSocket as a [`Vec<u8>`].
    pub fn get_last_message_as_vec(&self) -> Option<Vec<u8>> {
        Some(Uint8Array::new(&self.0.last_message()).to_vec())
    }

    /// True, when the mock WebSocket is connected.
    pub fn is_opened(&self) -> bool {
        self.0.is_opened()
    }

    /// Close mock WebSocket with default code (1005) and no reason.
    pub fn close(&self) {
        self.close_with_code_and_reason(1005, "");
    }

    /// Close mock WebSocket with code and no reason.
    pub fn close_with_code(&self, code: u16) {
        self.close_with_code_and_reason(code, "");
    }

    /// Close mock WebSocket with code and reason.
    pub fn close_with_code_and_reason(&self, code: u16, reason: &str) {
        self.0.close(code.into(), reason.into());
    }
}

impl Drop for WebSocketController {
    fn drop(&mut self) {
        self.0.restore();
    }
}

/**
Replaces the JS WebSocket with a mocked version and returns a controller for the mocked version.

The parameter is used to simulate connection time - using 0 will make the mock WebSocket connect
immediately.

# Examples

Mock a WebSocket that connects immediately:
```no_run
use sap_mock::WebSocketController;

let controller: WebSocketController = sap_mock::mock_ws(0);
// `ws` is a mocked WebSocket
let ws = web_sys::WebSocket::new("anyurl").unwrap();
// No wait required
assert!(controller.is_opened());
```

Mock a WebSocket that takes 500ms to connect:
```no_run
# async fn wait_for_ws() {
use sap_mock::WebSocketController;

// Mock a WebSocket that takes 500ms to connect
let controller: WebSocketController = sap_mock::mock_ws(500);
// `ws` is a mocked WebSocket
let ws = web_sys::WebSocket::new("anyurl").unwrap();

// Won't be open yet.
assert!(!controller.is_opened());

// Need to be in an async fn here to await
sap_mock::wait_ms(500).await.unwrap();
// After 500ms mock WebSocket will be opened
assert!(controller.is_opened());
# }
```
*/
pub fn mock_ws(conn_delay: u32) -> WebSocketController {
    WebSocketController(mock_websocket(conn_delay.into()))
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
    sap_mock::wait_ms(500).await.expect("Underlying JS not to throw exception");
    // some asserts..
}
```
*/
pub async fn wait_ms(ms: u32) -> Result<(), JsValue> {
    let _ = JsFuture::from(wait_promise(ms.into())).await?;
    Ok(())
}

/// A handle that keeps the current fetch mock living.
///
/// When this handle is dropped the original fetch API will be restored.
#[must_use]
pub struct FetchMockHandle(JsValue);

impl Drop for FetchMockHandle {
    fn drop(&mut self) {
        restore_fetch(&self.0);
    }
}

/**
Mocks the Fetch API to return either a value or an error depending on the mock input.

When used with [`Ok`] any calls to the fetch api will return a Response with the body of `T`,
however, when [`Err`] is used the fetch API will return a error Response with the status of
the u32 provided and will contain the string as the reason for this error.

# Examples
```
use wasm_bindgen_test::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use serde::{Deserialize, Serialize};
use web_sys::{window, Response};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Model {
    value: usize,
}

#[wasm_bindgen_test]
async fn mock_fetch_usize() {
    let mock = Model { value: 32 };

    // Hold handle to keep mock alive
    let _handle = sap_mock::mock_fetch(Ok(&mock));
    let window = window().expect("No global window");
    // Wrap fetch call into a Future to await it
    let resp: Response = JsFuture::from(window.fetch_with_str("someurl"))
        .await
        .unwrap()
        .unchecked_into();
    let json = JsFuture::from(resp.json().unwrap()).await.unwrap();
    let value = json.into_serde::<Model>().unwrap();

    assert_eq!(mock, value);

    // _handle goes out of scope and restores fetch for other tests
}
```
*/
pub fn mock_fetch<T>(mock: Result<&T, (u32, String)>) -> FetchMockHandle
where
    T: Serialize,
{
    let fetch = match mock {
        Ok(value) => mock_fetch_resolve(
            JsValue::from_serde(&value).expect("Mocked value failed to be serialized to a JsValue"),
        ),
        Err((code, reason)) => mock_fetch_error(code.into(), reason.into()),
    };

    FetchMockHandle(fetch)
}

#[cfg(test)]
mod tests {

    use super::*;

    use serde::Deserialize;
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    use wasm_bindgen_test::*;
    use web_sys::{window, MessageEvent, Response, WebSocket};
    wasm_bindgen_test_configure!(run_in_browser);

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct SomeObject {
        value: usize,
    }

    #[wasm_bindgen_test]
    async fn mock_fetch_usize() {
        let mock = SomeObject { value: 32 };

        // Hold handle to keep mock alive
        let _handle = mock_fetch(Ok(&mock));
        let window = window().expect("No global window");
        let resp: Response = JsFuture::from(window.fetch_with_str("someurl"))
            .await
            .unwrap()
            .unchecked_into();
        let json = JsFuture::from(resp.json().unwrap()).await.unwrap();
        let value = json.into_serde::<SomeObject>().unwrap();

        assert_eq!(mock, value);

        // _handle goes out of scope and restores fetch for other tests
    }

    #[wasm_bindgen_test]
    async fn mock_fetch_err() {
        let reason = "Server error!";
        let code = 500;

        let _handle = mock_fetch::<usize>(Err((code, reason.to_owned())));
        let window = window().expect("No global window");
        let resp: Response = JsFuture::from(window.fetch_with_str("url_with_server_error"))
            .await
            .unwrap()
            .unchecked_into();

        assert!(!resp.ok());

        let err = JsFuture::from(resp.json().unwrap()).await;

        assert!(err.is_err());

        match err {
            Ok(_) => panic!("Should be an error!"),
            Err(resp_reason) => {
                let resp_reason = resp_reason.as_string().unwrap();

                assert_eq!(reason, resp_reason);
            }
        };
    }

    #[wasm_bindgen_test]
    async fn send_str_to_mock_ws() {
        let controller = mock_ws(100);
        let ws = WebSocket::new("someurl").unwrap();

        // connection is not open yet!
        assert!(!controller.is_opened());
        // wait for connection
        wait_ms(100).await.unwrap();

        assert!(controller.is_opened());

        ws.send_with_str("Hello, World!").unwrap();

        assert_eq!(
            "Hello, World!",
            controller.get_last_message_as_string().unwrap()
        );

        let cb = Closure::wrap(Box::new(move |e: MessageEvent| {
            assert_eq!("hi", e.data().as_string().unwrap())
        }) as Box<dyn Fn(MessageEvent)>);

        ws.add_event_listener_with_callback("message", &cb.as_ref().unchecked_ref())
            .unwrap();

        controller.send_with_str("hi");
    }

    #[wasm_bindgen_test]
    async fn send_u8_array_to_mock_ws() {
        // no connection delay
        let controller = mock_ws(0);
        let ws = WebSocket::new("fakeurl").unwrap();

        let array = &[5, 4, 3, 2, 1];
        ws.send_with_u8_array(array).unwrap();
        let last_message = controller.get_last_message_as_vec();

        assert_eq!(array, &last_message.unwrap()[..]);
    }
}
