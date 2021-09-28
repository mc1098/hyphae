mod copyright;
mod footer;
mod header;
mod item;
mod list;

use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Todo {
    title: String,
    completed: bool,
    id: Uuid,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub todos: Signal<Vec<Signal<Todo>>>,
    pub filter: Signal<Filter>,
}

impl AppState {
    fn add_todo(&self, title: String) {
        self.todos.set(
            self.todos
                .get()
                .as_ref()
                .clone()
                .into_iter()
                .chain(Some(Signal::new(Todo {
                    title,
                    completed: false,
                    id: Uuid::new_v4(),
                })))
                .collect(),
        )
    }

    fn remove_todo(&self, id: Uuid) {
        self.todos.set(
            self.todos
                .get()
                .iter()
                .filter(|todo| todo.get().id != id)
                .cloned()
                .collect(),
        );
    }

    fn todos_left(&self) -> usize {
        self.todos.get().iter().fold(
            0,
            |acc, todo| if todo.get().completed { acc } else { acc + 1 },
        )
    }

    fn toggle_complete_all(&self) {
        if self.todos_left() == 0 {
            // make all todos active
            for todo in self.todos.get().iter() {
                if todo.get().completed {
                    todo.set(Todo {
                        completed: false,
                        ..todo.get().as_ref().clone()
                    })
                }
            }
        } else {
            // make all todos completed
            for todo in self.todos.get().iter() {
                if !todo.get().completed {
                    todo.set(Todo {
                        completed: true,
                        ..todo.get().as_ref().clone()
                    })
                }
            }
        }
    }

    fn clear_completed(&self) {
        self.todos.set(
            self.todos
                .get()
                .iter()
                .filter(|todo| !todo.get().completed)
                .cloned()
                .collect(),
        );
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            todos: Signal::new(Vec::new()),
            filter: Signal::new(Filter::All),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Filter {
    fn url(self) -> &'static str {
        match self {
            Filter::All => "#",
            Filter::Active => "#/active",
            Filter::Completed => "#/completed",
        }
    }

    fn get_filter_from_hash() -> Self {
        let hash = web_sys::window().unwrap().location().hash().unwrap();

        match hash.as_str() {
            "#/active" => Filter::Active,
            "#/completed" => Filter::Completed,
            _ => Filter::All,
        }
    }
}

const KEY: &str = "todos-sycamore";

#[component(App<G>)]
fn app() -> Template<G> {
    let local_storage = web_sys::window()
        .unwrap()
        .local_storage()
        .unwrap()
        .expect("user has not enabled localStorage");

    let todos = if let Ok(Some(app_state)) = local_storage.get_item(KEY) {
        serde_json::from_str(&app_state).unwrap_or_else(|_| Signal::new(Vec::new()))
    } else {
        Signal::new(Vec::new())
    };

    let app_state = AppState {
        todos,
        filter: Signal::new(Filter::get_filter_from_hash()),
    };

    create_effect(cloned!((local_storage, app_state) => move || {
        for todo in app_state.todos.get().iter() {
            todo.get(); // subscribe to changes in all todos
        }

        local_storage.set_item(KEY, &serde_json::to_string(app_state.todos.get().as_ref()).unwrap()).unwrap();
    }));

    let todos_is_empty =
        create_selector(cloned!((app_state) => move || app_state.todos.get().len() == 0));

    template! {
        div(class="todomvc-wrapper") {
            section(class="todoapp") {
                header::Header(app_state.clone())

                (if !*todos_is_empty.get() {
                    template! {
                        list::List(app_state.clone())
                        footer::Footer(app_state.clone())
                    }
                } else {
                    Template::empty()
                })
            }

            copyright::Copyright()
        }
    }
}

fn main() {
    sycamore::render(|| template! { App() });
}

#[cfg(test)]
mod tests {

    use super::*;
    use sap::{events::*, prelude::*, type_to};
    use wasm_bindgen_test::*;
    use web_sys::{HtmlButtonElement, HtmlElement, HtmlInputElement};
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn make_new_todo_item_complete_it_then_clear_completed() {
        let rendered = QueryElement::new();
        sycamore::render_to(|| template! { App() }, &rendered);

        // get todo input
        let input: HtmlInputElement = rendered.assert_by_placeholder_text("What needs to be done?");

        // type 'Gardening' into input and confirm
        type_to!(input, "Gardening", Key::Enter);

        // // expecting the item left count to be 1
        // let todo_left: HtmlElement = rendered.assert_by_text("1 item left");

        // 'Gardening' todo item has been rendered - lets just get the completed checkbox
        let checkbox: HtmlInputElement = rendered.assert_by_label_text("Gardening");
        // confirm that our new todo items is not completed
        assert!(!checkbox.checked());

        // click the todo checkbox - marking it complete
        checkbox.click();

        // get clear completed button - it only appears after a todo has been marked as completed
        let clear_completed_btn: HtmlButtonElement =
            rendered.assert_by_aria_role(AriaRole::Button, "Clear completed");

        // click to clear all completed todo items
        clear_completed_btn.click();
        // confirm that the todo item has been removed
        assert!(!rendered.contains(Some(&checkbox)));
    }
}
