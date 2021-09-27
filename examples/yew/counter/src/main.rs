use js_sys::Date;
use web_sys::console;
use yew::prelude::*;

pub enum Msg {
    Increment,
    Decrement,
}

pub struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <div class="panel">
                    // A button to send the Increment message
                    <button class="button" onclick={ctx.link().callback(|_| Msg::Increment)}>
                        { "+1" }
                    </button>
                    // A button to send the Decrement message
                    <button class="button" onclick={ctx.link().callback(|_| Msg::Decrement)}>
                        { "-1" }
                    </button>
                    // A button to send the two Increment messages
                    <button class="button" onclick={ctx.link().batch_callback(|_| vec![Msg::Increment, Msg::Increment])}>
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
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
    use super::*;
    use web_sys::{HtmlButtonElement, HtmlElement};

    #[wasm_bindgen_test]
    fn test_counter() {
        let rendered = QueryElement::new();
        yew::start_app_in_element::<Model>(rendered.clone().into());

        let inc_btn: HtmlButtonElement = rendered.assert_by_text("+1");
        let dec_btn: HtmlButtonElement = rendered.assert_by_text("-1");
        let dbl_inc_btn: HtmlButtonElement = rendered.assert_by_text("+1, +1");

        // Keep counter as it will be updated
        let counter: HtmlElement = rendered.assert_by_text("0");

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
