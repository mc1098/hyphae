mod event_bus;
mod producer;
mod subscriber;

use producer::Producer;
use subscriber::Subscriber;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Model;

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn change(&mut self, _msg: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _props: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Producer />
                <Subscriber />
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}

#[cfg(test)]
mod tests {

    use super::*;
    use sap::prelude::*;
    use sap_yew::test_render;
    use wasm_bindgen_test::*;
    use yew::web_sys::HtmlButtonElement;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn click_producer_and_view_subscriber_message() {
        let rendered = test_render! { <Model /> };

        // get subscriber heading message
        let sub_message: HtmlElement =
            rendered.assert_by_aria_role(AriaRole::Heading, "No message yet.");

        // get producer button
        let button: HtmlButtonElement = rendered.assert_by_aria_role(AriaRole::Button, "PRESS ME");

        // click the producer button which will send a message to the subscriber through an agent.
        button.click();

        // assert that the subscriber received the message and updated the heading.
        assert_text_content!("Message received", sub_message);
    }
}
