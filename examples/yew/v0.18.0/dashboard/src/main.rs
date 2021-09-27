use anyhow::Error;
use serde_derive::{Deserialize, Serialize};
use yew::{
    format::{Json, Nothing, Toml},
    prelude::*,
    services::{
        fetch::{FetchService, FetchTask, Request, Response},
        websocket::{WebSocketService, WebSocketStatus, WebSocketTask},
    },
};

type AsBinary = bool;

pub enum Format {
    Json,
    Toml,
}

pub enum WsAction {
    Connect,
    SendData(AsBinary),
    Disconnect,
    Lost,
}

pub enum Msg {
    FetchData(Format, AsBinary),
    WsAction(WsAction),
    FetchReady(Result<DataFromFile, Error>),
    WsReady(Result<WsResponse, Error>),
}

impl From<WsAction> for Msg {
    fn from(action: WsAction) -> Self {
        Msg::WsAction(action)
    }
}

/// This type is used to parse data from `./static/data.json` file and
/// have to correspond the data layout from that file.
#[derive(Deserialize, Debug)]
#[cfg_attr(test, derive(Serialize))]
pub struct DataFromFile {
    value: u32,
}

/// This type is used as a request which sent to websocket connection.
#[derive(Serialize, Debug)]
struct WsRequest {
    value: u32,
}

/// This type is an expected response from a websocket connection.
#[derive(Deserialize, Debug)]
pub struct WsResponse {
    value: u32,
}

pub struct Model {
    link: ComponentLink<Model>,
    data: Option<u32>,
    _ft: Option<FetchTask>,
    ws: Option<WebSocketTask>,
}

impl Model {
    fn view_data(&self) -> Html {
        if let Some(value) = self.data {
            html! {
                <p>{ value }</p>
            }
        } else {
            html! {
                <p>{ "Data hasn't fetched yet." }</p>
            }
        }
    }

    fn fetch_json(&mut self, binary: AsBinary) -> FetchTask {
        let callback = self.link.batch_callback(
            move |response: Response<Json<Result<DataFromFile, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                if meta.status.is_success() {
                    Some(Msg::FetchReady(data))
                } else {
                    None // FIXME: Handle this error accordingly.
                }
            },
        );
        let request = Request::get("/data.json").body(Nothing).unwrap();
        if binary {
            FetchService::fetch_binary(request, callback).unwrap()
        } else {
            FetchService::fetch(request, callback).unwrap()
        }
    }

    pub fn fetch_toml(&mut self, binary: AsBinary) -> FetchTask {
        let callback = self.link.batch_callback(
            move |response: Response<Toml<Result<DataFromFile, Error>>>| {
                let (meta, Toml(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                if meta.status.is_success() {
                    Some(Msg::FetchReady(data))
                } else {
                    None // FIXME: Handle this error accordingly.
                }
            },
        );
        let request = Request::get("/data.toml").body(Nothing).unwrap();
        if binary {
            FetchService::fetch_binary(request, callback).unwrap()
        } else {
            FetchService::fetch(request, callback).unwrap()
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            data: None,
            _ft: None,
            ws: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchData(format, binary) => {
                let task = match format {
                    Format::Json => self.fetch_json(binary),
                    Format::Toml => self.fetch_toml(binary),
                };
                self._ft = Some(task);
                true
            }
            Msg::WsAction(action) => match action {
                WsAction::Connect => {
                    let callback = self.link.callback(|Json(data)| Msg::WsReady(data));
                    let notification = self.link.batch_callback(|status| match status {
                        WebSocketStatus::Opened => None,
                        WebSocketStatus::Closed | WebSocketStatus::Error => {
                            Some(WsAction::Lost.into())
                        }
                    });
                    let task =
                        WebSocketService::connect("ws://localhost:9001/", callback, notification)
                            .unwrap();
                    self.ws = Some(task);
                    true
                }
                WsAction::SendData(binary) => {
                    let request = WsRequest { value: 321 };
                    if binary {
                        self.ws.as_mut().unwrap().send_binary(Json(&request));
                    } else {
                        self.ws.as_mut().unwrap().send(Json(&request));
                    }
                    false
                }
                WsAction::Disconnect => {
                    self.ws.take();
                    true
                }
                WsAction::Lost => {
                    self.ws = None;
                    true
                }
            },
            Msg::FetchReady(response) => {
                self.data = response.map(|data| data.value).ok();
                true
            }
            Msg::WsReady(response) => {
                self.data = response.map(|data| data.value).ok();
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <nav class="menu">
                    <button onclick=self.link.callback(|_| Msg::FetchData(Format::Json, false))>
                        { "Fetch Data" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::FetchData(Format::Json, true))>
                        { "Fetch Data [binary]" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::FetchData(Format::Toml, false))>
                        { "Fetch Data [toml]" }
                    </button>
                    { self.view_data() }
                    <button disabled=self.ws.is_some()
                            onclick=self.link.callback(|_| WsAction::Connect)>
                        { "Connect To WebSocket" }
                    </button>
                    <button disabled=self.ws.is_none()
                            onclick=self.link.callback(|_| WsAction::SendData(false))>
                        { "Send To WebSocket" }
                    </button>
                    <button disabled=self.ws.is_none()
                            onclick=self.link.callback(|_| WsAction::SendData(true))>
                        { "Send To WebSocket [binary]" }
                    </button>
                    <button disabled=self.ws.is_none()
                            onclick=self.link.callback(|_| WsAction::Disconnect)>
                        { "Close WebSocket connection" }
                    </button>
                </nav>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}

#[cfg(test)]
mod tests {

    use super::*;

    use anyhow::anyhow;

    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    use sap::prelude::*;
    use yew::{
        format::{Binary, Text},
        web_sys::{HtmlButtonElement, HtmlElement},
    };

    #[wasm_bindgen_test]
    async fn get_data() {
        let rendered = QueryElement::new();
        App::<Model>::new().mount(rendered.clone().into());

        let mock = DataFromFile { value: 20 };
        // mock fetch to return mock value
        let _handle = sap_mock::mock_fetch(Ok(&mock));

        let button: HtmlButtonElement =
            rendered.assert_by_aria_role(AriaRole::Button, "Fetch Data");

        // We need to wait for a bit here because fetch is async
        // even with a Promise that resolves immediately it will be delayed
        // Use effect_dom to add a future that won't complete until the dom changes or gets timed out.
        sap_utils::effect_dom(&rendered, move || button.click(), Some(100))
            .await
            .unwrap();

        // check that mock value has been added to the DOM.
        rendered.assert_by_text::<HtmlElement>("20");
    }

    #[wasm_bindgen_test]
    async fn get_binary_data() {
        let rendered = QueryElement::new();
        App::<Model>::new().mount(rendered.clone().into());

        let mock = DataFromFile { value: 50 };
        let _handle = sap_mock::mock_fetch(Ok(&mock));

        let button = rendered
            .assert_by_aria_role::<HtmlButtonElement>(AriaRole::Button, "Fetch Data [binary]");

        sap_utils::effect_dom(&rendered, move || button.click(), Some(100))
            .await
            .unwrap();

        rendered.assert_by_text::<HtmlElement>("50");
    }

    #[wasm_bindgen_test]
    async fn get_binary_toml() {
        let rendered = QueryElement::new();
        App::<Model>::new().mount(rendered.clone().into());

        let mock: Text = Toml(&DataFromFile { value: 230 }).into();
        let _handle = sap_mock::mock_fetch(Ok(&mock.unwrap()));

        let button = rendered
            .assert_by_aria_role::<HtmlButtonElement>(AriaRole::Button, "Fetch Data [toml]");

        sap_utils::effect_dom(&rendered, move || button.click(), Some(100))
            .await
            .unwrap();

        rendered.assert_by_text::<HtmlElement>("230");
    }

    #[wasm_bindgen_test]
    fn connect_to_ws_send_and_recieve() {
        let rendered = QueryElement::new();
        App::<Model>::new().mount(rendered.clone().into());

        let controller = sap_mock::mock_ws(0);

        let connect_to_ws_btn = rendered
            .assert_by_aria_role::<HtmlButtonElement>(AriaRole::Button, "Connect To WebSocket");

        let send_to_ws_btn: HtmlButtonElement =
            rendered.assert_by_aria_state(AriaState::Disabled(true), "Send To WebSocket");

        // connect to ws
        connect_to_ws_btn.click();

        // Should now be enabled
        assert!(!send_to_ws_btn.disabled());

        // Send WsRequest
        send_to_ws_btn.click();

        // map json string to Json format - use WsResponse as it has the same structure as request
        // but can be deserialized by Json::from.
        let request = controller
            .get_last_message_as_string()
            .ok_or_else(|| anyhow!("No message found!"));
        let Json(data): Json<Result<WsResponse, _>> = Json::from(request);

        // last message should match the request
        assert_eq!(321, data.unwrap().value);

        // send mocked response through controller to component
        let response: Text = Json(&WsRequest { value: 444 }).into();
        controller.send_with_str(&response.unwrap());

        // Confirm that the app actually received our mocked response and updated the value.
        rendered.assert_by_text::<HtmlElement>("444");

        // Drop controller which causes the WebSocket to close
        drop(controller);
        // Send To button should be disabled again now the WebSocket is closed.
        assert!(send_to_ws_btn.disabled());
    }

    #[wasm_bindgen_test]
    fn connect_to_ws_send_and_recieve_binary() {
        let rendered = QueryElement::new();
        App::<Model>::new().mount(rendered.clone().into());
        let controller = sap_mock::mock_ws(0);

        let connect_to_ws_btn = rendered
            .assert_by_aria_role::<HtmlButtonElement>(AriaRole::Button, "Connect To WebSocket");

        let send_to_ws_btn: HtmlButtonElement =
            rendered.assert_by_aria_state(AriaState::Disabled(true), "Send To WebSocket [binary]");

        // connect to ws
        connect_to_ws_btn.click();

        // Should now be enabled
        assert!(!send_to_ws_btn.disabled());

        // Send WsRequest
        send_to_ws_btn.click();

        // map binary to Json format with WsResponse
        let data = controller.get_last_message_as_vec().unwrap();
        // let mut data = vec![0; 13];
        // controller.copy_last_message_to_array(&mut data);
        let Json(data): Json<Result<WsResponse, _>> = Json::from(Ok(data));
        assert_eq!(321, data.unwrap().value);

        let response: Binary = Json(&WsRequest { value: 987 }).into();
        controller.send_with_u8_array(&response.unwrap());

        // Confirm that the app received our mock response and updated the value
        rendered.assert_by_text::<HtmlElement>("987");
    }
}
