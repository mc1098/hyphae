use sycamore::prelude::*;

#[component(App<G>)]
fn app() -> View<G> {
    let counter = Signal::new(0);

    create_effect(cloned!((counter) => move || {
        log::info!("Counter value: {}", *counter.get());
    }));

    let increment = cloned!((counter) => move |_| counter.set(*counter.get() + 1));

    let reset = cloned!((counter) => move |_| counter.set(0));

    view! {
        div {
            "Counter demo"
            p(class="value") {
                "Value: "
                (counter.get())
            }
            button(class="increment", on:click=increment) {
                "Increment"
            }
            button(class="reset", on:click=reset) {
                "Reset"
            }
        }
    }
}

fn main() {
    sycamore::render(|| view! { App() });
}

#[cfg(test)]
mod tests {

    use hyphae::prelude::*;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    use super::*;
    use web_sys::{HtmlButtonElement, HtmlElement};

    #[wasm_bindgen_test]
    fn can_count_and_reset() {
        let rendered = QueryElement::new();
        sycamore::render_to(|| view! { App() }, &rendered);

        let inc_btn: HtmlButtonElement = rendered.assert_by_text("Increment");
        let counter: HtmlElement = rendered.assert_by_text("Value: 0");

        inc_btn.click();
        assert_text_content!("Value: 1", counter);

        inc_btn.click();
        inc_btn.click();
        assert_text_content!("Value: 3", counter);

        let reset_btn: HtmlButtonElement = rendered.assert_by_text("Reset");
        reset_btn.click();
        assert_text_content!("Value: 0", counter);
    }
}
