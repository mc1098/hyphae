use state::{Entry, Filter, State};
use strum::IntoEnumIterator;
use yew::{format::Json, prelude::*, web_sys::HtmlInputElement};
use yew_services::storage::{Area, StorageService};

mod state;

const KEY: &str = "yew.todomvc.self";

pub enum Msg {
    Add,
    Edit(usize),
    Update(String),
    UpdateEdit(String),
    Remove(usize),
    SetFilter(Filter),
    ToggleAll,
    ToggleEdit(usize),
    Toggle(usize),
    ClearCompleted,
    Focus,
}

pub struct Model {
    link: ComponentLink<Self>,
    storage: StorageService,
    state: State,
    focus_ref: NodeRef,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).expect("storage was diabled by the user");
        let entries = {
            if let Json(Ok(restored_model)) = storage.restore(KEY) {
                restored_model
            } else {
                vec![]
            }
        };

        let state = State {
            entries,
            filter: Filter::All,
            value: "".into(),
            edit_value: "".into(),
        };
        let focus_ref = NodeRef::default();

        Self {
            link,
            storage,
            state,
            focus_ref,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => {
                let description = self.state.value.trim();
                if !description.is_empty() {
                    let entry = Entry {
                        description: description.to_string(),
                        completed: false,
                        editing: false,
                    };
                    self.state.entries.push(entry);
                }
                self.state.value = "".to_string();
            }
            Msg::Edit(idx) => {
                let edit_value = self.state.edit_value.trim().to_string();
                self.state.complete_edit(idx, edit_value);
                self.state.edit_value = "".to_string();
            }
            Msg::Update(val) => {
                println!("Input: {}", val);
                self.state.value = val;
            }
            Msg::UpdateEdit(val) => {
                println!("Input: {}", val);
                self.state.edit_value = val;
            }
            Msg::Remove(idx) => {
                self.state.remove(idx);
            }
            Msg::SetFilter(filter) => {
                self.state.filter = filter;
            }
            Msg::ToggleEdit(idx) => {
                self.state.edit_value = self.state.entries[idx].description.clone();
                self.state.clear_all_edit();
                self.state.toggle_edit(idx);
            }
            Msg::ToggleAll => {
                let status = !self.state.is_all_completed();
                self.state.toggle_all(status);
            }
            Msg::Toggle(idx) => {
                self.state.toggle(idx);
            }
            Msg::ClearCompleted => {
                self.state.clear_completed();
            }
            Msg::Focus => {
                if let Some(input) = self.focus_ref.cast::<HtmlInputElement>() {
                    input.focus().unwrap();
                }
            }
        }
        self.storage.store(KEY, Json(&self.state.entries));
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let hidden_class = if self.state.entries.is_empty() {
            "hidden"
        } else {
            ""
        };
        html! {
            <div class="todomvc-wrapper">
                <section class="todoapp">
                    <header class="header">
                        <h1>{ "todos" }</h1>
                        { self.view_input() }
                    </header>
                    <section class=classes!("main", hidden_class)>
                        <input
                            type="checkbox"
                            class="toggle-all"
                            id="toggle-all"
                            aria-label="toggle all todo items"
                            checked=self.state.is_all_completed()
                            onclick=self.link.callback(|_| Msg::ToggleAll)
                        />
                        <label for="toggle-all" />
                        <ul class="todo-list">
                            { for self.state.entries.iter().filter(|e| self.state.filter.fits(e)).enumerate().map(|e| self.view_entry(e)) }
                        </ul>
                    </section>
                    <footer class=classes!("footer", hidden_class)>
                        <span class="todo-count">
                            <strong>{ self.state.total() }</strong>
                            { " item(s) left" }
                        </span>
                        <ul class="filters">
                            { for Filter::iter().map(|flt| self.view_filter(flt)) }
                        </ul>
                        <button class="clear-completed" onclick=self.link.callback(|_| Msg::ClearCompleted)>
                            { format!("Clear completed ({})", self.state.total_completed()) }
                        </button>
                    </footer>
                </section>
                <footer class="info">
                    <p>{ "Double-click to edit a todo" }</p>
                    <p>{ "Written by " }<a href="https://github.com/DenisKolodin/" target="_blank">{ "Denis Kolodin" }</a></p>
                    <p>{ "Part of " }<a href="http://todomvc.com/" target="_blank">{ "TodoMVC" }</a></p>
                </footer>
            </div>
        }
    }
}

impl Model {
    fn view_filter(&self, filter: Filter) -> Html {
        let cls = if self.state.filter == filter {
            "selected"
        } else {
            "not-selected"
        };
        html! {
            <li>
                <a class=cls
                   href=filter.as_href()
                   onclick=self.link.callback(move |_| Msg::SetFilter(filter))
                >
                    { filter }
                </a>
            </li>
        }
    }

    fn view_input(&self) -> Html {
        html! {
            // You can use standard Rust comments. One line:
            // <li></li>
            <input
                class="new-todo"
                placeholder="What needs to be done?"
                value=self.state.value.clone()
                oninput=self.link.callback(|e: InputData| Msg::Update(e.value))
                onkeypress=self.link.batch_callback(|e: KeyboardEvent| {
                    if e.key() == "Enter" { Some(Msg::Add) } else { None }
                })
            />
            /* Or multiline:
            <ul>
                <li></li>
            </ul>
            */
        }
    }

    fn view_entry(&self, (idx, entry): (usize, &Entry)) -> Html {
        let mut class = Classes::from("todo");
        if entry.editing {
            class.push(" editing");
        }
        if entry.completed {
            class.push(" completed");
        }
        let id = format!("todo-item-{}", idx);
        let check_id = format!("todo-item-{}-check", idx);
        html! {
            <li id=id.clone() class=class>
                <div class="view">
                    <input
                        id=check_id.clone()
                        type="checkbox"
                        class="toggle"
                        checked=entry.completed
                        onclick=self.link.callback(move |_| Msg::Toggle(idx))
                    />
                    <label for=check_id ondblclick=self.link.callback(move |_| Msg::ToggleEdit(idx))>{ &entry.description }</label>
                    <button aria-controls=id class="destroy" onclick=self.link.callback(move |_| Msg::Remove(idx)) />
                </div>
                { self.view_entry_edit_input((idx, &entry)) }
            </li>
        }
    }

    fn view_entry_edit_input(&self, (idx, entry): (usize, &Entry)) -> Html {
        if entry.editing {
            html! {
                <input
                    class="edit"
                    type="text"
                    ref=self.focus_ref.clone()
                    value=self.state.edit_value.clone()
                    onmouseover=self.link.callback(|_| Msg::Focus)
                    oninput=self.link.callback(|e: InputData| Msg::UpdateEdit(e.value))
                    onblur=self.link.callback(move |_| Msg::Edit(idx))
                    onkeypress=self.link.batch_callback(move |e: KeyboardEvent| {
                        if e.key() == "Enter" { Some(Msg::Edit(idx)) } else { None }
                    })
                />
            }
        } else {
            html! { <input type="hidden" /> }
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
    use yew::{virtual_dom::test_render, web_sys::HtmlButtonElement};
    use yew_test::{events::*, prelude::*};
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn make_new_todo_item_complete_it_then_clear_completed() {
        let rendered = TestRender::new(test_render(html! {<Model />}));

        // get todo input
        let input: HtmlInputElement = rendered
            .get_by_placeholder_text("What needs to be done?")
            .unwrap();

        // type 'Gardening' into input
        type_to(&input, "Gardening");

        // press enter to confirm
        dispatch_key_event(&input, KeyEventType::KeyPress, Key::Enter);

        // 'Gardening' todo item has been rendered - lets just get the completed checkbox
        let checkbox: HtmlInputElement = rendered
            .get_by_label_text("Gardening")
            .expect("Input should be found by it's label text");
        // confirm that our new todo items is not completed
        assert!(!checkbox.checked());

        // get clear completed button
        let clear_completed_btn: HtmlButtonElement = rendered
            .get_by_aria_role(AriaRole::Button, "Clear completed (0)")
            .expect("clear completed button should be found by text_content");

        // click the todo checkbox - marking it complete
        checkbox.click();
        // confirm that a todo item was marked completed
        assert_text_content!("Clear completed (1)", clear_completed_btn);

        // click to clear all completed todo items
        clear_completed_btn.click();
        // confirm that the todo item has been removed
        assert!(!rendered.contains(Some(&checkbox)));
    }

    #[wasm_bindgen_test]
    fn make_new_todo_item_and_edit_it_and_complete() {
        use yew_test::prelude::*;

        let rendered = TestRender::new(test_render(html! { <Model /> }));

        // get todo input
        let input: HtmlInputElement = rendered
            .get_by_placeholder_text("What needs to be done?")
            .unwrap();

        // enter a new todo - oops pressed enter before finishing to type 'car'!
        type_to(&input, "Wash the c");
        dispatch_key_event(&input, KeyEventType::KeyPress, Key::Enter);

        // get the label for the new todo item
        let label: HtmlElement = rendered.get_by_text("Wash the c").unwrap();
        // double click label to edit todo
        label.dbl_click();

        // edit todo input is different - so go find it by the current value
        let edit_input: HtmlInputElement = rendered.get_by_display_value("Wash the c").unwrap();
        // finish typing 'car'
        type_to(&edit_input, "ar");
        // confirm edit
        dispatch_key_event(&edit_input, KeyEventType::KeyPress, Key::Enter);

        // confirm label has been updated with the correct text
        assert_text_content!("Wash the car", label);

        // get todo checkbox to complete it
        let checkbox: HtmlInputElement = rendered
            .get_by_aria_role(AriaRole::Checkbox, "Wash the car")
            .unwrap();
        // complete todo
        checkbox.click();

        // get clear completed button
        let clear_completed_btn: HtmlButtonElement = rendered
            .get_by_aria_role(AriaRole::Button, "Clear completed (1)")
            .unwrap();
        clear_completed_btn.click();
    }

    #[wasm_bindgen_test]
    fn make_multiple_todo_items_and_complete_them_all_at_once() {
        let rendered: TestRender = test_render(html! { <Model />}).into();

        // get todo input
        let input: HtmlInputElement = rendered
            .get_by_placeholder_text("What needs to be done?")
            .unwrap();

        // enter first todo
        type_to(&input, "A");
        dispatch_key_event(&input, KeyEventType::KeyPress, Key::Enter);

        // enter second todo
        type_to(&input, "B");
        dispatch_key_event(&input, KeyEventType::KeyPress, Key::Enter);

        // get clear completed button
        let clear_completed_btn: HtmlButtonElement = rendered
            .get_by_aria_role(AriaRole::Button, "Clear completed (0)")
            .unwrap();

        // get toggle all checkbox
        let toggle_all_checkbox: HtmlInputElement = rendered
            .get_by_aria_role(AriaRole::Checkbox, "toggle all todo items")
            .expect("toggle all checkbox should be found by it's aria-label value");

        // set all items to completed
        toggle_all_checkbox.click();

        // confirm that a todo item was marked completed
        assert_text_content!("Clear completed (2)", clear_completed_btn);

        // must clear completed because the storage is a side effect and can spill into other tests
        clear_completed_btn.click();
    }

    #[wasm_bindgen_test]
    fn make_new_todo_item_and_remove_it() {
        let rendered: TestRender = test_render(html! { <Model /> }).into();

        // get todo input
        let input: HtmlInputElement = rendered
            .get_by_placeholder_text("What needs to be done?")
            .unwrap();

        // enter todo item
        type_to(&input, "Some todo item");
        dispatch_key_event(&input, KeyEventType::KeyPress, Key::Enter);

        // get todo item
        let todo_item: HtmlElement = rendered
            .get_by_aria_role(AriaRole::ListItem, "Some todo item")
            .unwrap();

        // get single controlling element of checkbox using it's id
        let remove_button: HtmlButtonElement = rendered
            .get_by_aria_prop(AriaProperty::Controls([todo_item.id()].into()), None)
            .unwrap();

        // click and remove todo item
        remove_button.click();

        assert!(!rendered.contains(Some(&todo_item)));
    }

    fn render_model() -> TestRender {
        test_render(html! { <Model /> }).into()
    }

    #[wasm_bindgen_test]
    fn check_active_completed_tabs() {
        let rendered = render_model();

        // get todo input
        let input: HtmlInputElement = rendered
            .get_by_placeholder_text("What needs to be done?")
            .unwrap();

        // enter todo item
        type_to(&input, "A");
        dispatch_key_event(&input, KeyEventType::KeyPress, Key::Enter);

        // get todo item input
        let checkbox_a: HtmlInputElement = rendered
            .get_by_label_text("A")
            .expect("Input should be found by it's label text");

        // enter todo item
        type_to(&input, "B");
        dispatch_key_event(&input, KeyEventType::KeyPress, Key::Enter);

        // complete todo 'A'
        checkbox_a.click();

        // find active filter link
        let active_filter: HtmlElement = rendered
            .get_by_aria_role(AriaRole::Link, "Active")
            .expect("couldn't find 'Active' link");
        active_filter.click();

        // 'A' is completed so is not active
        assert!(rendered.get_by_label_text::<HtmlInputElement>("A").is_err());

        // find completed filter link
        let completed_filter: HtmlElement = rendered
            .get_by_aria_role(AriaRole::Link, "Completed")
            .expect("couldn't find 'Completed' link");

        completed_filter.click();

        // 'A' is completed so should now be able to find it!
        assert!(rendered.get_by_label_text::<HtmlInputElement>("A").is_ok());
        // 'B' is active so can not be found in completed filter
        assert!(rendered.get_by_label_text::<HtmlInputElement>("B").is_err());

        /*
        rendered.contains does not work here - this will always return true as these elements
        still 'live' in the DOM but disconnected. So we must try and find them again.
        */

        // time to clean up!

        // change back to 'All' filter to find checkbox_b
        rendered
            .get_by_aria_role::<_, HtmlElement>(AriaRole::Link, "All")
            .expect("all filter")
            .click();

        // set b as completed -
        // Note: even if we had a previous reference to the 'B' input that would be invalid after
        // changing the filter so getting a new reference to 'B' would always be required!
        rendered
            .get_by_label_text::<HtmlInputElement>("B")
            .unwrap()
            .click();

        // get clear completed button
        let clear_completed_btn: HtmlButtonElement = rendered
            .get_by_aria_role(AriaRole::Button, "Clear completed (2)")
            .expect("clear button");

        clear_completed_btn.click();
    }
}
