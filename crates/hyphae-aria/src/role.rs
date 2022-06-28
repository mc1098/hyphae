use crate::utils::ToQueryString;

macro_rules! roles_impl {
        ($(#[$role_comment:meta])+ pub enum AriaRole {$(
            $(#[$var_comment:meta])*
            $var:ident, $name:literal, [$(
                $implicit:literal$(,)?
            )*]$(,)?
        )*}) => {
            $(#[$role_comment])+
            #[non_exhaustive]
            pub enum AriaRole {
                $(
                    $(#[$var_comment])*
                    $var,
                )*
            }

            impl ToQueryString for AriaRole {
                fn to_query_string(&self) -> String {
                    match self {
                        $(
                            AriaRole::$var => {
                                let queries: &[&str] = &[$($implicit,)?];
                                if queries.is_empty() {
                                    format!("[role={}]", $name)
                                } else {
                                    format!("[role={}],{}", $name, queries.join(","))
                                }
                            }
                        )*
                    }
                }
            }
        };
    }

roles_impl! {
    /// Main indicator of type. This semantic association allows tools to present and support
    /// interaction with the object in a manner that is consistent with user expectations about
    /// other objects of that type.
    pub enum AriaRole {
    /// `alert` role - no implicit elements with these semantics
    Alert, "alert", [],
    /// `alertdialog` role - no implicit elements with these semantics
    AlertDialog, "alertdialog", [],
    /// `application` role - no implicit elements with these semantics
    Application, "application", [],
    ///
    AriaLabel, "aria-label", [],
    /// `article` role - implicit elements with these semantics:
    /// - `article`
    Article, "article", ["article"],
    /** `button` role - implicit elements with these semantics:
    - `button`
    - `input` with types of:
        - `button`
        - `img`
        - `reset`
        - `submit`
    - `summary`
    */
    Button, "button", ["button", "input[type=button], input[type=img], input[type=reset], input[type=submit], summary"],
    /// `checkbox` role - implicit elements with these semantics:
    /// - `input` with `type=checkbox`
    Checkbox, "checkbox", ["input[type=checkbox]"],
    /** `combobox` role - implicit elements with these semantics:
    - `input` with `list` attribute and types:
        - `text`
        - `search`
        - `tel`
        - `url`
        - `email`
    - `select`
    */
    Combobox, "combobox", [
        "input:not([type])",
        "input[type=text][list]",
        "input[type=search][list]",
        "input[type=tel][list]",
        "input[type=url][list]",
        "input[type=email][list]",
        "select"
        ],
    /** `complementary` role - implicit elements with these semantics:
    - `aside`
    */
    Complementary, "complementary", ["aside"],
    /** `dialog` role - implicit elements with these semantics:
    - `dialog`
    */
    Dialog, "dialog", ["dialog"],
    /** `figure` role - implicit elements with these semantics:
    - `figure`
    */
    Figure, "figure", ["figure"],
    /** `form` role - implicit elements with these semantics:
    - `form` - regardless of accessible name (differs from w3)
    */
    Form, "form", ["form"],
    /** `heading` role - implicit elements with these semantics:
    - `h1`
    - `h2`
    - `h3`
    - `h4`
    - `h5`
    - `h6`
    */
    Heading, "heading", ["h1", "h2", "h3", "h4", "h5", "h6"],
    /** `img` role - implicit elements with these semantics:
    - `img`
    */
    Image, "img", ["img"],
    /** `link` role - implicit elements with these semantics:
    - `a` with `href`
    - `area` with `href`
    */
    Link, "link", ["a[href]", "area[href]"],
    /** `list` role - implicit elements with these semantics:
    - `menu`
    - `ol`
    - `ul`
    */
    List, "list", ["menu", "ol", "ul"],
    /** `listbox` role - implicit elements with these semantics:
    - `datalist`
    - `select`
    */
    ListBox, "listbox", ["datalist", "select"],
    /** `listitem` role - implicit elements with these semantics:
    - `li`
    */
    ListItem, "listitem", ["li"],
    /// `log` role - no implicit elements with these semantics
    Log, "log", [],
    /** `main` role - implicit elements with these semantics:
    - `main`
    */
    Main, "main", ["main"],
    /** `math` role - implicit elements with these semantics:
    - `math`
    */
    Math, "math", ["math"],
    /// `menu` role - no implicit elements with these semantics
    Menu, "menu", [],
    /// `menuitem` role - no implicit elements with these semantics
    MenuItem, "menuitem", [],
    /// `menuitemcheckbox` role - no implicit elements with these semantics
    MenuItemCheckbox, "menuitemcheckbox", [],
    /// `menuitemcheckbox` role - no implicit elements with these semantics
    MenuItemRadio, "menuitemradio", [],
    /** `navigation` role - implicit elements with these semantics:
     - `nav`
    */
    Navigation, "navigation", ["nav"],
    /// `none` role - no implicit elements with these semantics
    None, "none", [],
    /// `note` role - no implicit elements with these semantics
    Note, "note", [],
    /** `option` role - implicit elements with these semantics:
     - `option`
    */
    Option, "option", ["option"],
    /** `status` role - implicit elements with these semantics:
     - `output`
    */
    Output, "status", ["output"],
    /** `presentation` role - implicit elements with these semantics:
     - `img` with alt="" (empty string)
    */
    Presentation, "presentation", ["img[alt=``]"],
    /** `progressbar` role - implicit elements with these semantics:
     - `progress`
    */
    Progressbar, "progressbar", ["progress"],
    /** `radio` role - implicit elements with these semantics:
     - `input` with `type=radio`
    */
    Radio, "radio", ["input[type=radio]"],
    /** `region` role - implicit elements with these semantics:
     - `section`
    */
    Region, "region", ["section"],
    /** `row` role - implicit elements with these semantics:
     - `tr`
    */
    Row, "row", ["tr"],
    /** `rowgroup` role - implicit elements with these semantics:
     - `tbody`
    - `tfoot`
    - `thead`
    */
    RowGroup, "rowgroup", ["tbody", "tfoot", "thead"],
    /** `rowheader` role - implicit elements with these semantics:
     - `th` within a `table` element
    */
    RowHeader, "rowheader", ["table>th"],
    /// `scrollbar` role - no implicit elements with these semantics
    Scrollbar, "scrollbar", [],
    /// `search` role - no implicit elements with these semantics
    Search, "search", [],
    /** `searchbox` role - implicit elements with these semantics:
     - `input` with `type=search`
    */
    Searchbox, "searchbox", ["input[type=search]"],
    /** `slider` role - implicit elements with these semantics:
     - `input` with `type=range`
    */
    Slider, "slider", ["input[type=range]"],
    /** `spinbutton` role - implicit elements with these semantics:
     - `input` with `type=number`
    */
    SpinButton, "spinbutton", ["input[type=number]"],
    /// `switch` role - no implicit elements with these semantics
    Switch, "switch", [],
    /// `tab` role - no implicit elements with these semantics
    Tab, "tab", [],
    /** `table` role - implicit elements with these semantics:
    - `table`
    */
    Table, "table", ["table"],
    /// `tabpanel` role - no implicit elements with these semantics
    TabPanel, "tabpanel", [],
    /** `term` role - implicit elements with these semantics:
    - `dfn`
    - `dt`
    */
    Term, "term", ["dfn", "dt"],
    /** `textbox` role - implicit elements with these semantics:
    - `input` with the types:
        - `email`
        - `tel`
        - `text` - this includes input without a type set
        - `url`
    - `textarea`
    */
    TextBox, "textbox", ["input:not([type])", "input[type=email]", "input[type=tel]", "input[type=text]", "input[type=url]", "textarea"],
    /// `toolbar` role - no implicit elements with these semantics
    Toolbar, "toolbar", [],
    /// `tooltip` role - no implicit elements with these semantics
    Tooltip, "tooltip", [],
    /// `treeitem` role - no implicit elements with these semantics
    TreeItem, "treeitem", [],
    }
}
