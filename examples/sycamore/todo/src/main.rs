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
    use sap::{assert_text_content, events::*, prelude::*, type_to};
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

        // expecting the item left count to be 1
        let todo_left: HtmlElement = rendered.assert_by_text("1 item left");

        // 'Gardening' todo item has been rendered - lets just get the completed checkbox
        let checkbox: HtmlInputElement = rendered.assert_by_label_text("Gardening");
        // confirm that our new todo items is not completed
        assert!(!checkbox.checked());

        // click the todo checkbox - marking it complete
        checkbox.click();

        // todo_left contains the number text node but the other text is in an
        // element so won't be found.
        assert_text_content!("0", todo_left);

        // get clear completed button - it only appears after a todo has been marked as completed
        let clear_completed_btn: HtmlButtonElement =
            rendered.assert_by_aria_role(AriaRole::Button, "Clear completed");

        // click to clear all completed todo items
        clear_completed_btn.click();
        // confirm that the todo item has been removed
        assert!(!rendered.contains(Some(&checkbox)));
    }

    #[wasm_bindgen_test]
    fn make_new_todo_item_and_edit_it_and_complete() {
        let rendered = QueryElement::new();
        sycamore::render_to(|| template! { App() }, &rendered);

        // get todo input
        let input: HtmlInputElement = rendered.assert_by_placeholder_text("What needs to be done?");

        // enter a new todo - oops pressed enter before finishing to type 'car!'
        type_to!(input, "Wash the c", Key::Enter);

        // get the label for the new todo item
        let label: HtmlElement = rendered.assert_by_text("Wash the c");
        // double click label to edit todo
        label.dbl_click();

        // edit todo input is different - so go find it by the current value
        let edit_input: HtmlInputElement = rendered.assert_by_display_value("Wash the c");
        // finish typing 'car' with 'Enter' to confirm
        type_to!(edit_input, "ar", Key::Enter);

        // confirm label has been updated with the correct text
        assert_text_content!("Wash the car", label);

        // get todo checkbox to complete it
        let checkbox: HtmlInputElement =
            rendered.assert_by_aria_role(AriaRole::Checkbox, "Wash the car");
        // complete todo
        checkbox.click();

        // get clear completed button
        let clear_completed_btn: HtmlButtonElement =
            rendered.assert_by_aria_role(AriaRole::Button, "Clear completed");
        clear_completed_btn.click();
    }

    #[wasm_bindgen_test]
    fn make_multiple_todo_items_and_complete_them_all_at_once() {
        let rendered = QueryElement::new();
        sycamore::render_to(|| template! { App() }, &rendered);

        // get todo input
        let input: HtmlInputElement = rendered.assert_by_placeholder_text("What needs to be done?");

        // enter first todo
        type_to!(input, "A", Key::Enter);

        // enter second todo
        type_to!(input, "B", Key::Enter);

        // get toggle all checkbox
        let toggle_all_checkbox: HtmlInputElement =
            rendered.assert_by_aria_role(AriaRole::Checkbox, "toggle all todo items");

        // expecting the item left count to be 2
        let todo_left: HtmlElement = rendered.assert_by_text("2 items left");

        // set all items to completed
        toggle_all_checkbox.click();

        // get clear completed button
        let clear_completed_btn: HtmlButtonElement =
            rendered.assert_by_aria_role(AriaRole::Button, "Clear completed");

        // confirm that no items are left todo
        assert_text_content!("0", todo_left);

        // must clear completed because the storage is a side effect and can spill into other tests
        clear_completed_btn.click();
    }

    #[wasm_bindgen_test]
    fn make_new_todo_item_and_remove_it() {
        let rendered = QueryElement::new();
        sycamore::render_to(|| template! { App() }, &rendered);

        // get todo input
        let input: HtmlInputElement = rendered.assert_by_placeholder_text("What needs to be done?");

        // enter todo item
        type_to!(input, "Some todo item", Key::Enter);

        // get todo item
        let todo_item: HtmlElement =
            rendered.assert_by_aria_role(AriaRole::ListItem, "Some todo item");

        // get single controlling element of checkbox using its id
        let remove_button: HtmlButtonElement =
            rendered.assert_by_aria_prop(AriaProperty::Controls([todo_item.id()].into()), None);

        // click and remove todo item
        remove_button.click();

        assert!(!rendered.contains(Some(&todo_item)));
    }

    #[wasm_bindgen_test]
    fn check_active_completed_tabs() {
        let rendered = QueryElement::new();
        sycamore::render_to(|| template! { App() }, &rendered);

        // get todo input
        let input: HtmlInputElement = rendered.assert_by_placeholder_text("What needs to be done?");

        // enter todo item
        type_to!(input, "A", Key::Enter);

        // get todo item checkbox
        let checkbox_a: HtmlInputElement = rendered.assert_by_label_text("A");

        // enter another todo item
        type_to!(input, "B", Key::Enter);

        // complete todo 'A'
        checkbox_a.click();

        // find active filter link
        let active_filter: HtmlElement = rendered.assert_by_aria_role(AriaRole::Link, "Active");
        active_filter.click();

        // 'A' is completed so is not active
        assert!(rendered.get_by_label_text::<HtmlInputElement>("A").is_err());

        // find completed filter link
        let completed_filter: HtmlElement =
            rendered.assert_by_aria_role(AriaRole::Link, "Completed");
        completed_filter.click();

        // 'A' is completed so should now be able to find it!
        rendered.assert_by_label_text::<HtmlInputElement>("A");

        // 'B' is active so can not be found in completed filter
        assert!(rendered.get_by_label_text::<HtmlInputElement>("B").is_err());

        /*
        rendered.contains does not work here - this will always return true as these elements
        still are in the DOM but disconnected. So we must try and find them again.
        */

        // time to clean up!

        // change back to 'All' filter to find checkbox_b
        rendered
            .assert_by_aria_role::<HtmlElement>(AriaRole::Link, "All")
            .click();

        // set 'B' as completed
        // Note: even if we had a previous reference to 'B' that would be invalid after
        // changing the filter so getting a new reference to 'B' is always required.
        rendered
            .assert_by_label_text::<HtmlInputElement>("B")
            .click();

        // get clear completed button
        rendered
            .assert_by_aria_role::<HtmlElement>(AriaRole::Button, "Clear completed")
            .click();
    }
}
