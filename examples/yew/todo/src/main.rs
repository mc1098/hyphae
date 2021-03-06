use gloo_storage::{LocalStorage, Storage};
use state::{Entry, Filter, State};
use strum::IntoEnumIterator;
use web_sys::HtmlInputElement;
use yew::{html::Scope, prelude::*};

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
    state: State,
    focus_ref: NodeRef,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        let entries = LocalStorage::get(KEY).unwrap_or_default();

        let state = State {
            entries,
            filter: Filter::All,
            value: "".into(),
            edit_value: "".into(),
        };
        let focus_ref = NodeRef::default();

        Self { state, focus_ref }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
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
        LocalStorage::set(KEY, &self.state.entries).expect("failed to set");
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

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
                        { self.view_input(link) }
                    </header>
                    <section class={classes!("main", hidden_class)}>
                        <input
                            type="checkbox"
                            class="toggle-all"
                            id="toggle-all"
                            aria-label="toggle all todo items"
                            checked={self.state.is_all_completed()}
                            onclick={link.callback(|_| Msg::ToggleAll)}
                        />
                        <label for="toggle-all" />
                        <ul class="todo-list">
                            { for self.state.entries.iter().filter(|e| self.state.filter.fits(e)).enumerate().map(|e| self.view_entry(link, e)) }
                        </ul>
                    </section>
                    <footer class={classes!("footer", hidden_class)}>
                        <span class="todo-count">
                            <strong>{ self.state.total() }</strong>
                            { " item(s) left" }
                        </span>
                        <ul class="filters">
                            { for Filter::iter().map(|flt| self.view_filter(link, flt)) }
                        </ul>
                        <button class="clear-completed" onclick={link.callback(|_| Msg::ClearCompleted)}>
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

macro_rules! value_from_input_target {
    ($event:expr) => {
        $event.target_unchecked_into::<HtmlInputElement>().value()
    };
}

impl Model {
    fn view_filter(&self, link: &Scope<Self>, filter: Filter) -> Html {
        let cls = if self.state.filter == filter {
            "selected"
        } else {
            "not-selected"
        };
        html! {
            <li>
                <a class={cls}
                   href={filter.as_href()}
                   onclick={link.callback(move |_| Msg::SetFilter(filter))}
                >
                    { filter }
                </a>
            </li>
        }
    }

    fn view_input(&self, link: &Scope<Self>) -> Html {
        let oninput = link.callback(|e: InputEvent| Msg::Update(value_from_input_target!(e)));

        html! {
            // You can use standard Rust comments. One line:
            // <li></li>
            <input
                class="new-todo"
                placeholder="What needs to be done?"
                value={self.state.value.clone()}
                {oninput}
                onkeypress={link.batch_callback(|e: KeyboardEvent| {
                    if e.key() == "Enter" { Some(Msg::Add) } else { None }
                })}
            />
            /* Or multiline:
            <ul>
                <li></li>
            </ul>
            */
        }
    }

    fn view_entry(&self, link: &Scope<Self>, (idx, entry): (usize, &Entry)) -> Html {
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
            <li id={id.clone()} class={class}>
                <div class="view">
                    <input
                        id={check_id.clone()}
                        type="checkbox"
                        class="toggle"
                        checked={entry.completed}
                        onclick={link.callback(move |_| Msg::Toggle(idx))}
                    />
                    <label for={check_id} ondblclick={link.callback(move |_| Msg::ToggleEdit(idx))}>{ &entry.description }</label>
                    <button aria-controls={id} class="destroy" onclick={link.callback(move |_| Msg::Remove(idx))} />
                </div>
                { self.view_entry_edit_input(link, (idx, entry)) }
            </li>
        }
    }

    fn view_entry_edit_input(&self, link: &Scope<Self>, (idx, entry): (usize, &Entry)) -> Html {
        if entry.editing {
            html! {
                <input
                    class="edit"
                    type="text"
                    ref={self.focus_ref.clone()}
                    value={self.state.edit_value.clone()}
                    onmouseover={link.callback(|_| Msg::Focus)}
                    oninput={link.callback(|e: InputEvent| Msg::UpdateEdit(value_from_input_target!(e)))}
                    onblur={link.callback(move |_| Msg::Edit(idx))}
                    onkeypress={link.batch_callback(move |e: KeyboardEvent| {
                        if e.key() == "Enter" { Some(Msg::Edit(idx)) } else { None }
                    })}
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
    use hyphae::{event::*, prelude::*, type_to};
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
    use web_sys::{HtmlButtonElement, HtmlElement};

    #[wasm_bindgen_test]
    fn make_new_todo_item_complete_it_then_clear_completed() {
        let rendered = QueryElement::default();
        let _ = yew::start_app_in_element::<Model>(rendered.clone().into());

        // get todo input
        let input: HtmlInputElement = rendered.assert_by_placeholder_text("What needs to be done?");

        // type 'Gardening' into input and confirm
        type_to!(input, "Gardening", Key::Enter);

        // 'Gardening' todo item has been rendered - lets just get the completed checkbox
        let checkbox: HtmlInputElement = rendered.assert_by_label_text("Gardening");
        // confirm that our new todo items is not completed
        assert!(!checkbox.checked());

        // get clear completed button
        let clear_completed_btn: HtmlButtonElement =
            rendered.assert_by_aria_role(AriaRole::Button, "Clear completed (0)");

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
        let rendered = QueryElement::default();
        let _ = yew::start_app_in_element::<Model>(rendered.clone().into());

        // get todo input
        let input: HtmlInputElement = rendered.assert_by_placeholder_text("What needs to be done?");

        // enter a new todo - oops pressed enter before finishing to type 'car'!
        type_to!(input, "Wash the c", Key::Enter);

        // get the label for the new todo item
        let label: HtmlElement = rendered.assert_by_text("Wash the c");
        // double click label to edit todo
        label.dbl_click();

        // edit todo input is different - so go find it by the current value
        let edit_input: HtmlInputElement = rendered.assert_by_display_value("Wash the c");
        // finish typing 'car'
        type_to!(edit_input, "ar");
        // confirm edit
        dispatch_key_event(&edit_input, KeyEventType::KeyPress, Key::Enter);

        // confirm label has been updated with the correct text
        assert_text_content!("Wash the car", label);

        // get todo checkbox to complete it
        let checkbox: HtmlInputElement =
            rendered.assert_by_aria_role(AriaRole::Checkbox, "Wash the car");
        // complete todo
        checkbox.click();

        // get clear completed button
        let clear_completed_btn: HtmlButtonElement =
            rendered.assert_by_aria_role(AriaRole::Button, "Clear completed (1)");
        clear_completed_btn.click();
    }

    #[wasm_bindgen_test]
    fn make_multiple_todo_items_and_complete_them_all_at_once() {
        let rendered = QueryElement::default();
        let _ = yew::start_app_in_element::<Model>(rendered.clone().into());

        // get todo input
        let input: HtmlInputElement = rendered.assert_by_placeholder_text("What needs to be done?");

        // enter first todo
        type_to!(input, "A", Key::Enter);

        // enter second todo
        type_to!(input, "B", Key::Enter);

        // get clear completed button
        let clear_completed_btn: HtmlButtonElement =
            rendered.assert_by_aria_role(AriaRole::Button, "Clear completed (0)");

        // get toggle all checkbox
        let toggle_all_checkbox: HtmlInputElement =
            rendered.assert_by_aria_role(AriaRole::Checkbox, "toggle all todo items");

        // set all items to completed
        toggle_all_checkbox.click();

        // confirm that a todo item was marked completed
        assert_text_content!("Clear completed (2)", clear_completed_btn);

        // must clear completed because the storage is a side effect and can spill into other tests
        clear_completed_btn.click();
    }

    #[wasm_bindgen_test]
    fn make_new_todo_item_and_remove_it() {
        let rendered = QueryElement::default();
        let _ = yew::start_app_in_element::<Model>(rendered.clone().into());

        // get todo input
        let input: HtmlInputElement = rendered.assert_by_placeholder_text("What needs to be done?");

        // enter todo item
        type_to!(input, "Some todo item", Key::Enter);

        // get todo item
        let todo_item: HtmlElement =
            rendered.assert_by_aria_role(AriaRole::ListItem, "Some todo item");

        // get single controlling element of checkbox using it's id
        let remove_button: HtmlButtonElement =
            rendered.assert_by_aria_prop(AriaProperty::Controls([todo_item.id()].into()), None);

        // click and remove todo item
        remove_button.click();

        assert!(!rendered.contains(Some(&todo_item)));
    }

    #[wasm_bindgen_test]
    fn check_active_completed_tabs() {
        let rendered = QueryElement::default();
        let _ = yew::start_app_in_element::<Model>(rendered.clone().into());

        // get todo input
        let input: HtmlInputElement = rendered.assert_by_placeholder_text("What needs to be done?");

        // enter todo item
        type_to!(input, "A", Key::Enter);

        // get todo item input
        let checkbox_a: HtmlInputElement = rendered.assert_by_label_text("A");

        // enter todo item
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
        still 'live' in the DOM but disconnected. So we must try and find them again.
        */

        // time to clean up!

        // change back to 'All' filter to find checkbox_b
        rendered
            .assert_by_aria_role::<HtmlElement>(AriaRole::Link, "All")
            .click();

        // set b as completed -
        // Note: even if we had a previous reference to the 'B' input that would be invalid after
        // changing the filter so getting a new reference to 'B' would always be required!
        rendered
            .assert_by_label_text::<HtmlInputElement>("B")
            .click();

        // get clear completed button
        let clear_completed_btn: HtmlButtonElement =
            rendered.assert_by_aria_role(AriaRole::Button, "Clear completed (2)");

        clear_completed_btn.click();
    }
}
