mod event_bus;
mod producer;
mod subscriber;

use producer::Producer;
use subscriber::Subscriber;
use yew::{html, Component, Context, Html};

pub struct Model;

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _: &Context<Self>) -> Html {
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
    use wasm_bindgen_test::*;
    use web_sys::{HtmlButtonElement, HtmlElement};

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn click_producer_and_view_subscriber_message() {
        let rendered = QueryElement::new();
        yew::start_app_in_element::<Model>(rendered.clone().into());

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
