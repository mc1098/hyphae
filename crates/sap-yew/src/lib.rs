/*!
A bridge crate between Sap and Yew, which exposes a single macro to help rendering a component
for testing.
*/

/**
Convenience macro for test rendering of Yew components or raw html blocks.

Note: A big limitation to this macro is that it cannot capture dynamic values - if
you run into this problem then you may need to create a Wrapper component to
provide the desired values.

_This API requires the following crate features to be activated: `Yew`_

# Examples

## Components
```no_run
use sap::prelude::*;
use sap_yew::test_render;
use yew::prelude::*;
// Counter component impl
# struct Counter;
# impl Component for Counter {
#   type Message = ();
#   type Properties = ();
#
#   fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
#       Self
#   }
#
#   fn update(&mut self, _: Self::Message) -> ShouldRender {
#       false
#   }
#
#   fn change(&mut self, _: Self::Properties) -> ShouldRender {
#       false
#   }
#
#   fn view(&self) -> Html {
#       Html::default()
#   }
# }
let rendered = test_render! { <Counter /> };
// use rendered to perform queries.
```

## Component with props
```no_run
use sap::prelude::*;
use sap_yew::test_render;
use yew::prelude::*;

// Counter component impl
# #[derive(Clone, Properties)]
# struct CounterProps {
#   start: usize,
}
# struct Counter {
#   props: CounterProps,
# }
# impl Component for Counter {
#   type Message = ();
#   type Properties = CounterProps;
#
#   fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
#       Self { props }
#   }
#
#   fn update(&mut self, _: Self::Message) -> ShouldRender {
#       false
#   }
#
#   fn change(&mut self, _: Self::Properties) -> ShouldRender {
#       false
#   }
#
#   fn view(&self) -> Html {
#       Html::default()
#   }
# }
let props = CounterProps { start: 0 };
let rendered = test_render!{ <Counter with props /> };
// use rendered to perform queries.

```

## Raw `html!` blocks
This macro contains an arm that accepts the same input as Yew's `html!` macro:
```no_run
use sap::prelude::*;
use sap_yew::test_render;
use yew::prelude::*;

let rendered = test_render! {
    <div>
        <h1>{ "Hello, World!" }</h1>
    </div>
};
// use rendered to perform queries.
```
This macro uses the version of the `html!` that is currently in your project
so will be in sync with your project.
*/
#[macro_export]
macro_rules! test_render {
    (<$comp:ident />) => {{
        let props = Default::default();
        test_render!(<$comp with props />)
    }};
    (<$comp:ident with $props:ident />) => {{
        let div = yew::utils::document().create_element("div").unwrap();
        div.set_id("test-app");
        yew::utils::document()
            .body()
            .unwrap()
            .append_child(&div)
            .unwrap();
        yew::start_app_with_props_in_element::<$comp>(div.clone(), $props);
        TestRender::new(div)
    }};
    ($($html:tt)+) => {{
        pub struct TestComp;
        impl yew::html::Component for TestComp {
            type Properties = ();
            type Message = ();

            fn create(_: Self::Properties, _: yew::html::ComponentLink<Self>) -> Self {
                Self
            }

            fn update(&mut self, _: Self::Message) -> yew::html::ShouldRender {
                false
            }
            fn change(&mut self, _: Self::Properties) -> yew::html::ShouldRender {
                false
            }
            fn view(&self) -> yew::html::Html {
                yew::html! { $($html)+ }

            }
        }
        test_render!(<TestComp />)
    }};
}
