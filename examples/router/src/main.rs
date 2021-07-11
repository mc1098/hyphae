use yew::prelude::*;
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
    link: ComponentLink<Self>,
    navbar_active: bool,
}
impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            navbar_active: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                { self.view_nav() }

                <main>
                    <Router<Route> render=Router::render(switch) />
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
    fn view_nav(&self) -> Html {
        let Self {
            ref link,
            navbar_active,
            ..
        } = *self;

        let active_class = if navbar_active { "is-active" } else { "" };

        html! {
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <h1 class="navbar-item is-size-3">{ "Yew Blog" }</h1>

                    <a role="button"
                        class=classes!("navbar-burger", "burger", active_class)
                        aria-label="menu" aria-expanded="false"
                        onclick=link.callback(|_| Msg::ToggleNavbar)
                    >
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </a>
                </div>
                <div class=classes!("navbar-menu", active_class)>
                    <div class="navbar-start">
                        <Link<Route> classes=classes!("navbar-item") route=Route::Home>
                            { "Home" }
                        </Link<Route>>
                        <Link<Route> classes=classes!("navbar-item") route=Route::Posts>
                            { "Posts" }
                        </Link<Route>>

                        <div class="navbar-item has-dropdown is-hoverable">
                            <a class="navbar-link">
                                { "More" }
                            </a>
                            <div class="navbar-dropdown">
                                <a class="navbar-item">
                                    <Link<Route> classes=classes!("navbar-item") route=Route::Authors>
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

                fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
                    Self
                }

                fn update(&mut self, _: Self::Message) -> ShouldRender {
                    false
                }

                fn change(&mut self, _: Self::Properties) -> ShouldRender {
                    false
                }

                fn view(&self) -> Html {
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
    use sap_yew::test_render;

    #[wasm_bindgen_test]
    fn route_test() {
        let rendered: TestRender = test_render! { <Model /> };

        let _: HtmlElement = rendered
            .get_by_aria_role(AriaRole::Heading, "Home")
            .expect("default route should be 'home' with the 'Home' heading");

        let posts_link: HtmlElement = rendered
            .get_by_aria_role(AriaRole::Link, "Posts")
            .expect("`Posts` link should always be available");

        posts_link.click();

        let _: HtmlElement = rendered
            .get_by_aria_role(AriaRole::Heading, "Post List")
            .expect("should be on the posts page");

        let authors_link: HtmlElement = rendered
            .get_by_aria_role(AriaRole::Link, "Meet the authors")
            .expect("More menu should be open so should be able to find authors link");

        authors_link.click();

        let _: HtmlElement = rendered
            .get_by_aria_role(AriaRole::Heading, "Author List")
            .expect("should be on the authors page");
    }
}
