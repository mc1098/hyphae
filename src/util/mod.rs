mod html;
mod lev_distance;

pub(crate) use html::{
    format_html, format_html_with_closest, get_element_value, set_element_value,
};
pub(crate) use lev_distance::closest;
