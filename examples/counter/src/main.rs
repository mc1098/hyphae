use js_sys::Date;
use yew::{prelude::*, web_sys::console};

pub enum Msg {
    Increment,
    Decrement,
}

pub struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.value += 1;
                console::log_1(&"plus one".into());
            }
            Msg::Decrement => {
                self.value -= 1;
                console::log_1(&"minus one".into());
            }
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div class="panel">
                    // A button to send the Increment message
                    <button class="button" onclick=self.link.callback(|_| Msg::Increment)>
                        { "+1" }
                    </button>
                    // A button to send the Decrement message
                    <button class="button" onclick=self.link.callback(|_| Msg::Decrement)>
                        { "-1" }
                    </button>
                    // A button to send the two Increment messages
                    <button class="button" onclick=self.link.batch_callback(|_| vec![Msg::Increment, Msg::Increment])>
                        { "+1, +1" }
                    </button>

                </div>

                // Display the current value of the counter
                <p class="counter">
                    { self.value }
                </p>

                // Display the current date and time the page was rendered
                <p class="footer">
                    { "Rendered: " }
                    { String::from(Date::new_0().to_string()) }
                </p>

            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}

#[cfg(test)]
mod tests {

    use sap::prelude::*;
    use wasm_bindgen_test::*;
    use yew::web_sys::{HtmlButtonElement, HtmlElement};
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
    use super::*;

    #[wasm_bindgen_test]
    fn test_counter() {
        let rendered = test_render! { <Model /> };

        let inc_btn: HtmlButtonElement = rendered.get_by_text("+1").unwrap();
        let dec_btn: HtmlButtonElement = rendered.get_by_text("-1").unwrap();
        let dbl_inc_btn: HtmlButtonElement = rendered.get_by_text("+1, +1").unwrap();

        // Keep counter as it will be updated
        let counter: HtmlElement = rendered.get_by_text("0").unwrap();

        // Confirm that the counter is at 0
        assert_text_content!(0, counter);

        inc_btn.click();
        assert_text_content!(1, counter);

        dbl_inc_btn.click();
        assert_text_content!(3, counter);

        dec_btn.click();
        dec_btn.click();
        dec_btn.click();
        assert_text_content!(0, counter);

        dec_btn.click();
        dec_btn.click();
        // To confirm that the counter in the DOM is in sync with the 'counter' variable
        let counter_copy: HtmlElement = rendered.get_by_text("-2").unwrap();
        assert_eq!(counter, counter_copy);
    }
}
