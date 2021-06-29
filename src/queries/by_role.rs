use crate::utils::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlImageElement, NodeList};

use crate::{ElementIter, TestRender};

/// Enables querying by ARIA roles, properties and state.
pub trait ByAria {
    /**
    Get an [`Element`](web_sys::Element) by ARIA role that has text content that matches the
    search term.

    This includes getting elements by their implicit aria roles - for a list of elements with
    their implicit roles see [here on w3](https://www.w3.org/TR/html-aria/#docconformance).

    The [`AriaRole`] enum does document each element that has that role implicitly.

    # Examples

    Effective html rendered:
    ```html
    <div>
        <div id="not-mybtn">
            click me!
            <button id="mybtn">click me!</button>
        </div>
    </div>
    ```
    ```no_run
    let rendered: TestRender = //..
    // ignores the text node in the div with the same search term
    let button: HtmlButtonElement = rendered
        .get_by_aria_role(AriaRole::Button, "click me!")
        .unwrap();

    assert_eq!("mybtn", button.id());
    ```
    */
    fn get_by_aria_role<'a, S, T>(&self, role: AriaRole, name: S) -> Option<T>
    where
        S: Into<Option<&'a str>>,
        T: JsCast;

    /**
    Get all [`Element`] by ARIA role

    This includes getting elements by their implicit aria roles - for a list of elements with
    their implicit roles see [here on w3](https://www.w3.org/TR/html-aria/#docconformance).

    The [`AriaRole`] enum does document each element that has that role implicitly.

    # Examples
    ```no_run
    /*
    rendered effective html:
        <div>
            <ol>
                <li id="item-1"></li>
                <li id="item-2"></li>
                <li id="item-3"></li>
                <li id="item-4"></li>
                <li id="item-5"></li>
            </ol>
        </div>
    */
    let rendered: TestRender = //..
    let list_items = rendered.get_all_by_aria_role::<HtmlElement>(AriaRole::ListItem);

    assert_eq!(5, list_items.size_hint().1.unwrap())
    ```
     */
    fn get_all_by_aria_role<T>(&self, role: AriaRole) -> ElementIter<T>
    where
        T: JsCast;

    /**
    Get an [`Element`](web_sys::Element) by ARIA property.

    Some [`AriaProperty`] are so descriptive that the accessible name is not required, such is the case
    with [`AriaProperty::Label`] - the content of this property is the accessible name and
    **should be** unique in the DOM. When this is the case the accessible name can be [`None`].

    # Examples

    ## Accessible name required to find a single element
    Effective html rendered:
    ```html
    <form>
        <label for="user-email">Email:</label>
        <input type="email" id="user-email" required aria-required="true" />
        <label for="user-password">Password:</label>
        <input type="password" id="user-password" required aria-required="true" />
    </form>
    ```
    ```no_run
    let rendered: TestRender = //..

    let email_input: HtmlInputElement = rendered
        .get_by_aria_prop(AriaProperty::Required(true), "Email:".into())
        .unwrap();

    assert_eq!("user-email", email_input.id());
    ```
    ## When the accessible name is not required
    Effective html rendered:
    ```html
    <div>
        <div id="not-mybtn">
            <button id="mybtn" aria-label="ok" /> <!-- No text on button -->
        </div>
    </div>
    ```
    ```no_run
    let rendered: TestRender = // ..

    let button: HtmlButtonElement = rendered
        .get_by_aria_prop(AriaProperty::Label("ok"), None)
        .unwrap();

    assert_eq!("mybtn", button.id());
    ```
    */
    fn get_by_aria_prop<'a, S, T>(&self, property: AriaProperty, name: S) -> Option<T>
    where
        S: Into<Option<&'a str>>,
        T: JsCast;

    /**
    Get all [`Element`]s by ARIA property.

    # Examples
    effective html rendered:
    ```html
    <form>
        <input type="text" aria-required="true" placeholder="Username" />
        <input type="password" aria-required="true" placeholder="Password" />
        <label for="spam-email-signup">Want loads of emails about us?</label>
        <input id="spam-email-signup" type="checkbox" aria-required="false" />
    </form>
    ```
    ```no_run
    let rendered: TestRender = //..

    let required_fields =
        rendered.get_all_by_aria_prop::<HtmlInputElement>(AriaProperty::Required(true));

    assert_eq!(2, required_fields.size_hint().1.unwrap());
    ```
    */
    fn get_all_by_aria_prop<T>(&self, prop: AriaProperty) -> ElementIter<T>
    where
        T: JsCast;

    /**
    Get an [`Element`] by ARIA state.

    [`AriaState`] can be descriptive in a subsection of HTML that the accessible name is not
    required, such is the case with [`AriaState::Selected`] on a subsection which can only have one
    selected item. When this is the case the accessible name can be [`None`].

    # Examples
    Effective rendered html:
    ```html
    <form>
        <input id="best-pet" aria-invalid="spelling" value="doge" aria-label="best pet" />
        <input id="second-best-pet" aria-invalid="false" value="cat" aria-label="second best pet" />
    </form>
    ```
    ```no_run
    let rendered: TestRender = //..
    let spelling_error_input: HtmlInputElement = rendered
        .get_by_aria_state(AriaState::Invalid(InvalidToken::Spelling), "best pet")
        .unwrap();

    assert_eq!("best-pet", spelling_error_input.id());
    ```
    */
    fn get_by_aria_state<'a, S, T>(&self, state: AriaState, name: S) -> Option<T>
    where
        S: Into<Option<&'a str>>,
        T: JsCast;

    /**
    Get all [`Element`] by ARIA state.

    # Examples
    Effective rendered html:
    ```html
    <div>
        <button disabled />
        <button disabled />
        <button disabled />
        <button disabled />
    </div>
    ```
    ```no_run
    let rendered: TestRender = //..
    let disabled_buttons =
            rendered.get_all_by_aria_state::<HtmlButtonElement>(AriaState::Disabled(true));

    assert_eq!(4, disabled_buttons.size_hint().1.unwrap());
    ```
    */
    fn get_all_by_aria_state<T>(&self, state: AriaState) -> ElementIter<T>
    where
        T: JsCast;
}

/// Reference to the ID of another element in the same document
pub type IdReference = String;

/// A list of one or more [`IdReference`]s.
pub struct IdReferenceList(Vec<String>);

impl std::fmt::Display for IdReferenceList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0.join(","))
    }
}

impl<S> From<S> for IdReferenceList
where
    S: AsRef<[String]>,
{
    fn from(slice: S) -> Self {
        IdReferenceList(slice.as_ref().to_owned())
    }
}

/// A list of one or more tokens.
pub struct TokenList<'a, T>(&'a [T])
where
    T: std::fmt::Display;

impl<'a, S, T> From<&'a S> for TokenList<'a, T>
where
    S: AsRef<[T]>,
    T: std::fmt::Display,
{
    fn from(slice: &'a S) -> Self {
        TokenList(slice.as_ref())
    }
}

impl<T> std::fmt::Display for TokenList<'_, T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0.iter().fold(String::new(), |mut acc, t| {
                acc.push(',');
                acc.push_str(&t.to_string());
                acc
            })
        )
    }
}

macro_rules! enum_to_lowercase_string_impl {
        ( $(#[$enum_comment:meta])+ $enum_name:ident {$( $(#[$var_comment:meta])+ $variant:ident,)*$(,)?}) => {
            #[derive(Debug, PartialEq)]
            $(#[$enum_comment])+
            pub enum $enum_name {
                $(
                    $(#[$var_comment])+
                    $variant,
                )*
            }

            #[allow(deprecated)]
            impl std::fmt::Display for $enum_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                    match self {
                        $(
                            $enum_name::$variant => write!(f, "{}", stringify!($variant).to_lowercase()),
                        )*
                    }
                }
            }
        };
    }

enum_to_lowercase_string_impl! {
    #[deprecated(note = "Deprecated in ARIA 1.1")]
    /// Indicates what functions can be performed when a dragged object is released on the drop target.
    DropEffectToken {
        /// A duplicate of the source object will be dropped into the target.
        Copy,
        /// A function supported by the drop target is executed, using the drag source as an input.
        Execute,
        /// A reference or shortcut to the dragged object will be created in the target object.
        Link,
        /// The source object will be removed from its current location and dropped into the target.
        Move,
        /// No operation can be performed; effectively cancels the drag operation if an attempt
        /// is made to drop on this object. Ignored if combined with any other token value.
        /// e.g., 'none copy' is equivalent to a 'copy' value.
        None,
        /// There is a popup menu or dialog that allows the user to choose one of the drag
        /// operations (copy, move, link, execute) and any other drag functionality,
        /// such as cancel.
        Popup,
    }
}

enum_to_lowercase_string_impl! {
    /// Indicates whether inputting text could trigger display of one or more predictions of
    /// the user's intended value for an input and specifies how predictions would be presented
    /// if they are made.
    AutoCompleteToken {
        /// When a user is providing input, text suggesting one way to complete the provided
        /// input may be dynamically inserted after the caret.
        Inline,
        /// When a user is providing input, an element containing a collection of values that
        /// could complete the provided input may be displayed.
        List,
        /// When a user is providing input, an element containing a collection of values that
        /// could complete the provided input may be displayed. If displayed, one value in the
        /// collection is automatically selected, and the text needed to complete the
        /// automatically selected value appears after the caret in the input.
        Both,
        /// When a user is providing input, an automatic suggestion that attempts to predict
        /// how the user intends to complete the input is not displayed.
        None,
    }
}

enum_to_lowercase_string_impl! {
    /// Indicates the availability and type of interactive popup element, such as menu or
    /// dialog, that can be triggered by an element.
    HasPopupToken {
        /// Indicates the element does not have a popup.
        False,
        /// Indicates the popup is a menu.
        True,
        /// Indicates the popup is a menu.
        Menu,
        /// Indicates the popup is a listbox.
        ListBox,
        /// Indicates the popup is a tree.
        Tree,
        /// Indicates the popup is a grid.
        Grid,
        /// Indicates the popup is a dialog.
        Dialog,
    }
}

enum_to_lowercase_string_impl! {
    /// Indicates that an element will be updated, and describes the types of updates the user
    /// agents, assistive technologies, and user can expect from the live region.
    LiveToken {
        /// Indicates that updates to the region have the highest priority and should be
        /// presented the user immediately.
        Assertive,
        /// Indicates that updates to the region should not be presented to the user unless
        /// the used is currently focused on that region.
        Off,
        /// Indicates that updates to the region should be presented at the next graceful
        /// opportunity, such as at the end of speaking the current sentence or when the user
        /// pauses typing.
        Polite,
    }
}

enum_to_lowercase_string_impl! {
    /// Indicates whether the element's orientation is horizontal, vertical, or
    /// unknown/ambiguous.
    OrientationToken {
        /// The element is oriented horizontally.
        Horizontal,
        /// The element's orientation is unknown/ambiguous.
        Undefined,
        /// The element is oriented vertically.
        Vertical,
    }
}

enum_to_lowercase_string_impl! {
    /// Indicates what notifications the user agent will trigger when the accessibility tree
    /// within a live region is modified. See related aria-atomic.
    RelevantToken {
        /// Element nodes are added to the accessibility tree within the live region.
        Additions,
        /// Equivalent to the combination of values, "additions text".
        AdditionsText,
        /// Equivalent to the combination of all values, "additions removals text".
        All,
        /// Text content, a text alternative, or an element node within the live region is
        /// removed from the accessibility tree.
        Removals,
        /// Text content or a text alternative is added to any descendant in the accessibility
        /// tree of the live region.
        Text,
    }
}

enum_to_lowercase_string_impl! {
    /// Indicates if items in a table or grid are sorted in ascending or descending order.
    SortToken {
        /// Items are sorted in ascending order by this column.
        Ascending,
        /// Items are sorted in descending order by this column.
        Descending,
        /// There is no defined sort applied to the column.
        None,
        /// A sort algorithm other than ascending or descending has been applied.
        Other,
    }
}

macro_rules! aria_enum {
        ($(#[$enum_comment:meta])+ $enum_name:ident {$(
            $(#[$var_comment:meta])+ $var_name:ident($var_type:ty) => $implicit: expr
        ),*$(,)?}) => {
            $(#[$enum_comment])+
            pub enum $enum_name {
                $(
                    $(#[$var_comment])+
                    #[allow(dead_code)]
                    $var_name($var_type),
                )*
            }

            #[allow(deprecated)]
            impl std::fmt::Display for $enum_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                    match self {
                        $(
                            $enum_name::$var_name(value) => write!(f, "{}[aria-{}={}]",
                                $implicit(value),
                                stringify!($var_name).to_lowercase(),
                                value.to_string(),
                            ),
                        )*
                    }
                }
            }
        };
        ($(#[$enum_comment:meta])+ $enum_name:ident<'a> {$( $(#[$var_comment:meta])+ $var_name:ident($var_type:ty)),*$(,)?}) => {
            $(#[$enum_comment])+
            pub enum $enum_name<'a> {
                $(
                    $(#[$var_comment])+
                    #[allow(dead_code, deprecated)]
                    $var_name($var_type),
                )*
            }

            #[allow(deprecated)]
            impl std::fmt::Display for $enum_name<'_> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                    match self {
                        $(
                            $enum_name::$var_name(value) => write!(f, "[aria-{}={}]",
                                stringify!($var_name).to_lowercase(),
                                value.to_string()
                            ),
                        )*
                    }
                }
            }
        };
    }

aria_enum! {
    /// Attributes that are essential to the nature of a given object, or that represent a
    /// data value associated with the object. A change of a property may significantly
    /// impact the meaning or presentation of an object. Certain properties (for example,
    /// aria-multiline) are less likely to change than states, but note that the frequency of
    /// change difference is not a rule. A few properties, such as aria-activedescendant,
    /// aria-valuenow, and aria-valuetext are expected to change often.
    AriaProperty<'a> {
        /// Indicates whether assistive technologies will present all, or only parts of,
        /// the changed region based on the change notifications defined by the aria-relevant
        /// attribute.
        Atomic(bool),
        /// Identifies the element (or elements) whose contents or presence are controlled by
        /// the current element.
        Controls(IdReferenceList),
        /// Identifies the element (or elements) that describes the object.
        DescribedBy(IdReferenceList),
        /// Identifies the element that provides a detailed, extended description for the object.
        Details(IdReference),
        #[deprecated(note = "Deprecated in ARIA 1.1")]
        /// Indicates what functions can be performed when a dragged object is released on the drop target.
        DropEffect(TokenList<'a, DropEffectToken>), //(&'a [DropEffectToken]),
        /// Identifies the element that provides an error message for the object.
        ErrorMessage(IdReference),
        /// Identifies the currently active element when DOM focus is on a composite widget,
        /// textbox, group, or application.
        ActiveDescendant(IdReference),
        /// Indicates whether inputting text could trigger display of one or more predictions
        /// of the user's intended value for an input and specifies how predictions would be
        /// presented if they are made.
        AutoComplete(AutoCompleteToken),
        /// Defines the total number of columns in a table, grid, or treegrid.
        ColCount(i32),
        /// Defines an element's column index or position with respect to the total number of
        /// columns within a table, grid, or treegrid.
        ColIndex(i32),
        /// Defines the number of columns spanned by a cell or gridcell within a table, grid,
        /// or treegrid.
        ColSpan(i32),
        /// Identifies the next element (or elements) in an alternate reading order of content
        /// which, at the user's discretion, allows assistive technology to override the
        /// general default of reading in document source order.
        FlowTo(IdReferenceList),
        /// Indicates the availability and type of interactive popup element, such as menu or
        /// dialog, that can be triggered by an element.
        HasPopup(HasPopupToken),
        /// Indicates keyboard shortcuts that an author has implemented to activate or give
        /// focus to an element.
        KeyShortcuts(&'a str),
        /// Defines a string value that labels the current element.
        Label(&'a str),
        /// Identifies the element (or elements) that labels the current element.
        LabelledBy(IdReferenceList),
        /// Defines the hierarchical level of an element within a structure.
        Level(i32),
        /// Indicates that an element will be updated, and describes the types of updates the
        /// user agents, assistive technologies, and user can expect from the live region.
        Live(LiveToken),
        /// Indicates whether an element is modal when displayed.
        Modal(bool),
        /// Indicates whether a text box accepts multiple lines of input or only a single line.
        MultiLine(bool),
        /// Indicates that the user may select more than one item from the current selectable
        /// descendants.
        MultiSelectable(bool),
        /// Indicates whether the element's orientation is horizontal, vertical, or
        /// unknown/ambiguous.
        Orientation(OrientationToken),
        /// Identifies an element (or elements) in order to define a visual, functional, or
        /// contextual parent/child relationship between DOM elements where the DOM hierarchy
        /// cannot be used to represent the relationship.
        Owns(IdReferenceList),
        /// Defines a short hint (a word or short phrase) intended to aid the user with data
        /// entry when the control has no value. A hint could be a sample value or a brief
        /// description of the expected format.
        Placeholder(&'a str),
        /// Defines an element's number or position in the current set of listitems or
        /// treeitems. Not required if all elements in the set are present in the DOM.
        PosInSet(i32),
        /// Indicates that the element is not editable, but is otherwise operable.
        ReadOnly(bool),
        /// Indicates what notifications the user agent will trigger when the accessibility
        /// tree within a live region is modified.
        Relevant(TokenList<'a, RelevantToken>),//(&'a [RelevantToken]),
        /// Indicates that user input is required on the element before a form may be submitted.
        Required(bool),
        /// Defines a human-readable, author-localized description for the role of an element.
        RoleDescription(&'a str),
        /// Defines the total number of rows in a table, grid, or treegrid.
        RowCount(i32),
        /// Defines an element's row index or position with respect to the total number of rows
        /// within a table, grid, or treegrid.
        RowIndex(i32),
        /// Defines the number of rows spanned by a cell or gridcell within a table, grid, or
        /// treegrid.
        RowSpan(i32),
        /// Defines the number of items in the current set of listitems or treeitems.
        /// Not required if all elements in the set are present in the DOM.
        SetSize(i32),
        /// Indicates if items in a table or grid are sorted in ascending or descending order.
        Sort(SortToken),
        /// Defines the maximum allowed value for a range widget.
        ValueMax(f32),
        /// Defines the minimum allowed value for a range widget.
        ValueMin(f32),
        /// Defines the current value for a range widget.
        ValueNow(f32),
        /// Defines the human readable text alternative of aria-valuenow for a range widget.
        ValueText(&'a str),
    }
}

macro_rules! roles_impl {
        ($(#[$role_comment:meta])+ pub enum AriaRole {$(
            $(#[$var_comment:meta])*
            $var:ident, $name:literal, [$(
                $implicit:literal$(,)?
            )*]$(,)?
        )*}) => {
            $(#[$role_comment])+
            pub enum AriaRole {
                $(
                    $(#[$var_comment])*
                    $var,
                )*
            }

            impl std::fmt::Display for AriaRole {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                    match self {
                        $(
                            AriaRole::$var => {
                                let queries: &[&str] = &[$($implicit,)?];
                                if queries.is_empty() {
                                    write!(f, "[role={}]", $name)
                                } else {
                                    write!(f, "[role={}],{}", $name, queries.join(","))
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
        - `text`
        - `url`
    - `textarea`
    */
    TextBox, "textbox", ["input[type=email]", "input[type=tel]", "input[type=text]", "input[type=url]", "textarea"],
    /// `toolbar` role - no implicit elements with these semantics
    Toolbar, "toolbar", [],
    /// `tooltip` role - no implicit elements with these semantics
    Tooltip, "tooltip", [],
    /// `treeitem` role - no implicit elements with these semantics
    TreeItem, "treeitem", [],
    }
}

enum_to_lowercase_string_impl! {
    /// Value representing true, false, or not applicable.
    DuoState {
        /// Synonymous with boolean true
        True,
        /// Synonymous with boolean false
        False,
        /// Not applicable
        Undefined,
    }
}

enum_to_lowercase_string_impl! {
    /// Value representing true or false, with an intermediate "mixed" value. The default
    /// value for this value type is false unless otherwise specified.
    TriState {
        /// Synonymous with boolean false
        False,
        /// Intermediate value between true and false
        Mixed,
        /// Synonymous with boolean true
        True,
        /// Not defined
        Undefined,
    }
}

enum_to_lowercase_string_impl! {
    /// Indicates the element that represents the current item within a container or set of
    /// related elements.
    CurrentToken {
        /// Represents the current page within a set of pages.
        Page,
        /// Represents the current step within a process.
        Step,
        /// Represents the current location within an environment or context.
        Location,
        /// Represents the current date within a collection of dates.
        Date,
        /// Represents the current time within a set of times.
        Time,
        /// Represents the current item within a set.
        True,
        /// Does not represent the current item within a set.
        False,
    }
}

enum_to_lowercase_string_impl! {
    /// Indicates the entered value does not conform to the format expected by the application.
    InvalidToken {
        /// A grammatical error was detected.
        Grammar,
        /// There are no detected errors in the value.
        False,
        /// A spelling error was detected.
        Spelling,
        /// The value entered by the user has failed validation.
        True,
    }
}

fn state_default<T>(_value: T) -> String {
    String::new()
}

fn state_checked(state: &TriState) -> String {
    if *state == TriState::True {
        "input[type=checkbox]:checked,input[type=radio]:checked,".to_string()
    } else {
        String::new()
    }
}

fn state_disabled(state: &bool) -> String {
    if *state {
        ":disabled,".to_string()
    } else {
        String::new()
    }
}

fn state_hidden(state: &DuoState) -> String {
    if *state == DuoState::True {
        ":hidden,".to_string()
    } else {
        String::new()
    }
}

aria_enum! {
    /// A state is a dynamic property expressing characteristics of an object that may change
    /// in response to user action or automated processes.
    /// States do not affect the essential nature of the object, but represent data
    /// associated with the object or user interaction possibilities.
    AriaState {
        /// Indicates an element is being modified and that assistive technologies MAY want to
        /// wait until the modifications are complete before exposing them to the user.
        Busy(bool) => state_default,
        /**
        Indicates the current "checked" state of checkboxes, radio buttons,
        and other widgets.

        Attribute parity:
        - `input` elements of the following types that are checked:
            - `radio`
            - `checkbox`
        */
        Checked(TriState) => state_checked,
        /// Indicates the element that represents the current item within a container or set
        /// of related elements.
        Current(CurrentToken) => state_default,
        /**
        Indicates that the element is perceivable but disabled, so it is not editable or
        otherwise operable.

        Attribute parity:
        - any element which is disabled will be equivalent to aria-disabled="true"
        */
        Disabled(bool) => state_disabled,
        /// Indicates whether the element, or another grouping element it controls, is
        /// currently expanded or collapsed.
        Expanded(DuoState) => state_default,
        #[deprecated(note = "Deprecated in ARIA 1.1")]
        /// Indicates an element's "grabbed" state in a drag-and-drop operation.
        Grabbed(DuoState) => state_default,
        /**
        Indicates whether the element is exposed to an accessibility API.

        Attribute parity:
        - any element which is hidden will be equivalent to aria-hidden="true"
        */
        Hidden(DuoState) => state_hidden,
        /// Indicates the entered value does not conform to the format expected by the
        /// application.
        Invalid(InvalidToken) => state_default,
        /// Indicates the current "pressed" state of toggle buttons.
        Pressed(TriState) => state_default,
        /// Indicates the current "selected" state of various widgets.
        Selected(DuoState) => state_default,
    }
}

fn is_accessible_name_match(root: &Element, element: &Element, name: &str) -> bool {
    // is an aria-label set with the given name?
    if element
        .get_attribute("aria-label")
        .map(|label| label == name)
        .unwrap_or_default()
    {
        return true;
    }

    // if aria-labelledby is set then look to see if that label's text_content matches name
    if let Some(label_id) = element.get_attribute("aria-labelledby") {
        if is_text_content_from_query_select(root.query_selector(&format!("#{}", label_id)), name) {
            return true;
        }
    }

    // If this element has an id attribute then a label might be associated with this element
    if let Some(this_id) = element.get_attribute("id") {
        if is_text_content_from_query_select(
            root.query_selector(&format!("label[for={}]", this_id)),
            name,
        ) {
            return true;
        }
    }

    // If this element is an img than check alt text
    if element
        .dyn_ref::<HtmlImageElement>()
        .map(|img| img.alt() == name)
        .unwrap_or_default()
    {
        return true;
    }

    // finally check the elements text_content - this is last because it's the most passive
    // approach for accessiblilty naming so acts as a good default last check.
    has_text_content(&element, name)
}

fn find_accessible_name<T>(root: &Element, node_list: NodeList, name: &str) -> Option<T>
where
    T: JsCast,
{
    for i in 0..node_list.length() {
        let node = node_list.get(i)?;

        if is_accessible_name_match(root, node.unchecked_ref(), name) {
            return node.dyn_into().ok();
        }
    }
    None
}

#[inline]
fn get_by_aria_impl<S, T>(root: &Element, aria: S, name: Option<&str>) -> Option<T>
where
    S: ToString,
    T: JsCast,
{
    let node_list = root.query_selector_all(&aria.to_string()).ok()?;
    if let Some(name) = name {
        find_accessible_name(root, node_list, name)
    } else {
        node_list.get(0).and_then(|e| e.dyn_into().ok())
    }
}

#[inline]
fn get_all_by_aria_impl<S, T>(root: &Element, aria: S) -> ElementIter<T>
where
    S: ToString,
    T: JsCast,
{
    let node_list = root.query_selector_all(&aria.to_string()).ok();
    ElementIter::new(node_list)
}

impl ByAria for TestRender {
    fn get_by_aria_role<'a, S, T>(&self, role: AriaRole, name: S) -> Option<T>
    where
        S: Into<Option<&'a str>>,
        T: JsCast,
    {
        get_by_aria_impl(self, role, name.into())
    }

    fn get_all_by_aria_role<T>(&self, role: AriaRole) -> ElementIter<T>
    where
        T: JsCast,
    {
        get_all_by_aria_impl(self, role)
    }

    fn get_by_aria_prop<'a, S, T>(&self, prop: AriaProperty, name: S) -> Option<T>
    where
        S: Into<Option<&'a str>>,
        T: JsCast,
    {
        if let AriaProperty::Label(_) = prop {
            self.query_selector(&prop.to_string())
                .ok()
                .flatten()
                .and_then(|e| e.dyn_into().ok())
        } else {
            get_by_aria_impl(self, prop, name.into())
        }
    }

    fn get_all_by_aria_prop<T>(&self, prop: AriaProperty) -> ElementIter<T>
    where
        T: JsCast,
    {
        get_all_by_aria_impl(self, prop)
    }

    fn get_by_aria_state<'a, S, T>(&self, state: AriaState, name: S) -> Option<T>
    where
        S: Into<Option<&'a str>>,
        T: JsCast,
    {
        get_by_aria_impl(self, state, name.into())
    }

    fn get_all_by_aria_state<T>(&self, state: AriaState) -> ElementIter<T>
    where
        T: JsCast,
    {
        get_all_by_aria_impl(self, state)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use wasm_bindgen_test::*;
    use web_sys::{HtmlButtonElement, HtmlInputElement};
    use yew::virtual_dom::test_render;
    wasm_bindgen_test_configure!(run_in_browser);

    use yew::prelude::*;

    #[wasm_bindgen_test]
    fn get_by_button_role_with_text_content() {
        let rendered: TestRender = test_render(html! {
            <div>
                <div id="not-mybtn">
                    { "click me" }
                    <button id="mybtn">{ "click me!" }</button>
                </div>
            </div>
        })
        .into();
        let button: HtmlButtonElement = rendered
            .get_by_aria_role(AriaRole::Button, "click me!")
            .unwrap();

        assert_eq!("mybtn", button.id());
    }

    #[wasm_bindgen_test]
    fn get_by_aria_label() {
        let rendered: TestRender = test_render(html! {
            <div>
                <div id="not-mybtn">
                    <button id="mybtn" aria-label="ok" /> // No text on button
                </div>
            </div>
        })
        .into();

        let button: HtmlButtonElement = rendered
            .get_by_aria_prop(AriaProperty::Label("ok"), None)
            .unwrap();

        assert_eq!("mybtn", button.id());
    }

    #[wasm_bindgen_test]
    fn get_by_aria_disabled_state() {
        let rendered: TestRender = test_render(html! {
            <div>
                <input type="email" id="my-input" aria-disabled="true" />
            </div>
        })
        .into();

        let input: HtmlInputElement = rendered
            .get_by_aria_state(AriaState::Disabled(true), None)
            .unwrap();

        assert_eq!("my-input", input.id());
    }

    #[wasm_bindgen_test]
    fn get_single_input_with_spelling_error() {
        let rendered: TestRender = test_render(html! {
            <form>
                <input id="best-pet" aria-label="best pet" aria-invalid="spelling" value="doge" />
                <input id="second-best-pet" aria-label="second best pet" aria-invalid="false" value="cat"  />
            </form>
        })
        .into();
        let spelling_error_input: HtmlInputElement = rendered
            .get_by_aria_state(AriaState::Invalid(InvalidToken::Spelling), "best pet")
            .unwrap();

        assert_eq!("best-pet", spelling_error_input.id());
    }

    #[wasm_bindgen_test]
    fn get_input_by_role_with_aria_label() {
        let rendered: TestRender = test_render(html! {
            <div>
                <input id="myinput" type="text" aria-label="username" />
            </div>
        })
        .into();

        let input: HtmlInputElement = rendered
            .get_by_aria_role(AriaRole::TextBox, "username")
            .unwrap();

        assert_eq!("myinput", input.id());
    }

    #[wasm_bindgen_test]
    fn get_button_by_role_with_aria_labelledby() {
        let rendered: TestRender = test_render(html! {
            <>
                <div id="button-label" >
                    { "My custom button label" }
                </div>
                <button aria-labelledby="button-label" />
            </>
        })
        .into();

        let button: HtmlButtonElement = rendered
            .get_by_aria_role(AriaRole::Button, "My custom button label")
            .unwrap();

        assert_eq!(
            "button-label",
            button.get_attribute("aria-labelledby").unwrap()
        );
    }

    #[wasm_bindgen_test]
    fn get_input_by_role_with_label() {
        let rendered: TestRender = test_render(html! {
            <div>
                <div>
                    <label for="my-input">{ "My input label" }</label>
                </div>
                <input id="my-input" type="search" />
            </div>

        })
        .into();

        let input: HtmlInputElement = rendered
            .get_by_aria_role(AriaRole::Searchbox, "My input label")
            .unwrap();

        assert_eq!("my-input", input.id());
    }

    #[wasm_bindgen_test]
    fn get_img_by_role_with_alt() {
        let rendered: TestRender = test_render(html! {
            <div>
                <img id="no" src="first-img.jpg" />
                <img id="yes" src="somg-img.jpg" alt="The best image ever!" />
            </div>
        })
        .into();

        let img: HtmlImageElement = rendered
            .get_by_aria_role(AriaRole::Image, "The best image ever!")
            .unwrap();

        assert_eq!("yes", img.id());
    }

    #[wasm_bindgen_test]
    fn get_all_list_items() {
        let rendered: TestRender = test_render(html! {
            <div>
                <ol>
                    <li id="item-1"></li>
                    <li id="item-2"></li>
                    <li id="item-3"></li>
                    <li id="item-4"></li>
                    <li id="item-5"></li>
                </ol>
            </div>
        })
        .into();
        let list_items = rendered.get_all_by_aria_role::<web_sys::HtmlElement>(AriaRole::ListItem);

        assert_eq!(5, list_items.size_hint().1.unwrap())
    }

    #[wasm_bindgen_test]
    fn get_all_required_input() {
        let rendered: TestRender = test_render(html! {
            <form>
                <input type="text" aria-required="true" placeholder="Username" />
                <input type="password" aria-required="true" placeholder="Password" />
                <label for="spam-email-signup">{ "Want loads of emails about us?" }</label>
                <input id="spam-email-signup" type="checkbox" aria-required="false" />
            </form>
        })
        .into();

        let required_fields =
            rendered.get_all_by_aria_prop::<HtmlInputElement>(AriaProperty::Required(true));

        assert_eq!(2, required_fields.size_hint().1.unwrap());
    }

    #[wasm_bindgen_test]
    fn get_disabled_buttons_implicit() {
        // disabled=true is implicitly == aria-disabled=true
        let rendered: TestRender = test_render(html! {
            <>
                <button disabled=true/>
                <button disabled=true/>
                <button disabled=true/>
                <button disabled=true/>
            </>
        })
        .into();

        let disabled_buttons =
            rendered.get_all_by_aria_state::<HtmlButtonElement>(AriaState::Disabled(true));

        assert_eq!(4, disabled_buttons.size_hint().1.unwrap());
    }
}
