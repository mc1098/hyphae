use yew::{html::Scope, prelude::*};
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/posts")]
    Posts,
    #[at("/authors")]
    Authors,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub enum Msg {
    ToggleNavbar,
}

pub struct Model {
    navbar_active: bool,
}
impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            navbar_active: false,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                { self.view_nav(ctx.link()) }

                <main>
                    <Router<Route> render={Router::render(switch)} />
                </main>
                <footer class="footer">
                    <div class="content has-text-centered">
                        { "Powered by " }
                        <a href="https://yew.rs">{ "Yew" }</a>
                        { " using " }
                        <a href="https://bulma.io">{ "Bulma" }</a>
                        { " and images from " }
                        <a href="https://unsplash.com">{ "Unsplash" }</a>
                    </div>
                </footer>
            </>
        }
    }
}
impl Model {
    fn view_nav(&self, link: &Scope<Self>) -> Html {
        let navbar_active = self.navbar_active;

        let active_class = if navbar_active { "is-active" } else { "" };

        html! {
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <h1 class="navbar-item is-size-3">{ "Yew Blog" }</h1>

                    <a role="button"
                        class={classes!("navbar-burger", "burger", active_class)}
                        aria-label="menu" aria-expanded="false"
                        onclick={link.callback(|_| Msg::ToggleNavbar)}
                    >
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </a>
                </div>
                <div class={classes!("navbar-menu", active_class)}>
                    <div class="navbar-start">
                        <Link<Route> classes={classes!("navbar-item")} route={Route::Home}>
                            { "Home" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} route={Route::Posts}>
                            { "Posts" }
                        </Link<Route>>

                        <div class="navbar-item has-dropdown is-hoverable">
                            <a class="navbar-link">
                                { "More" }
                            </a>
                            <div class="navbar-dropdown">
                                <a class="navbar-item">
                                    <Link<Route> classes={classes!("navbar-item")} route={Route::Authors}>
                                        { "Meet the authors" }
                                    </Link<Route>>
                                </a>
                            </div>
                        </div>
                    </div>
                </div>
            </nav>
        }
    }
}

macro_rules! static_comp {
    ($($comp_name:ident => $content:expr),*) => {
        $(
            struct $comp_name;

            impl Component for $comp_name {
                type Message = ();
                type Properties = ();

                fn create(_: &Context<Self>) -> Self {
                    Self
                }

                fn view(&self, _: &Context<Self>) -> Html {
                    $content
                }
            }
        )*
    };
}

static_comp! {
    PostList => html! (<h1>{ "Post List" }</h1>),
    AuthorList => html! (<h1>{ "Author List" }</h1>),
    Home => html! (<h1>{ "Home" }</h1>),
    PageNotFound => html! (<h1>{ "Page Not Found" }</h1>)
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Posts => {
            html! { <PostList /> }
        }
        Route::Authors => {
            html! { <AuthorList /> }
        }
        Route::Home => {
            html! { <Home /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}

#[cfg(test)]
mod tests {

    use super::*;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    use sap::prelude::*;
    use web_sys::HtmlElement;

    #[wasm_bindgen_test]
    fn route_test() {
        let rendered = QueryElement::new();
        yew::start_app_in_element::<Model>(rendered.clone().into());

        // Confirm that the Home component is loaded by checking for the "Home" heading.
        rendered.assert_by_aria_role::<HtmlElement>(AriaRole::Heading, "Home");

        // Get the link to the post page.
        let posts_link: HtmlElement = rendered.assert_by_aria_role(AriaRole::Link, "Posts");
        // Click link to change to post page
        posts_link.click();

        // Confirm that the Post component is loaded by checking for the "Post List" heading.
        rendered.assert_by_aria_role::<HtmlElement>(AriaRole::Heading, "Post List");

        // Confirm that the Home component is no longer loaded and the "Home" heading cannot be found!
        assert!(rendered
            .get_by_aria_role::<HtmlElement>(AriaRole::Heading, "Home")
            .is_err());

        // Same as above with the link to the authors page and confirming that it loads

        let authors_link: HtmlElement =
            rendered.assert_by_aria_role(AriaRole::Link, "Meet the authors");

        authors_link.click();

        rendered.assert_by_aria_role::<HtmlElement>(AriaRole::Heading, "Author List");
    }
}
