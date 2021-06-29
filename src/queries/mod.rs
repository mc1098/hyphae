/*!
Queries for finding [`Element`](web_sys::Element)s.

This module helps to query the DOM of a rendered root element. The goal is to use high/medium level
APIs so that the DOM can be queried in a manner similar to how a user might navigate the UI.

This module includes the following traits:
- [`ByDisplayValue`]
- [`ByLabelText`]
- [`ByPlaceholderText`]
- [`ByAria`]
- [`ByText`]
*/
mod by_display_value;
mod by_label_text;
mod by_placeholder_text;
mod by_role;
mod by_text;

pub use self::{
    by_display_value::*, by_label_text::*, by_placeholder_text::*, by_role::*, by_text::*,
};
