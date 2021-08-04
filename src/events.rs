/*!
Convenience module for firing events to [`EventTarget`].

The goal of this module is to remove the boilerplate from firing [`web_sys`] events by providing
helper functions and traits for medium/high level actions.
*/

use std::ops::{Deref, DerefMut};

use web_sys::{
    Event, EventInit, EventTarget, InputEvent, InputEventInit, KeyboardEvent, KeyboardEventInit,
    MouseEvent, MouseEventInit,
};

/**
Dispatches a single [`KeyboardEvent`] with the type and key provided to the event target.

Uses the [`KeyEventType`] and [`Key`] enum to provide type safe options - this avoids typos causing
tests to fail.

The [list of keys available](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key/Key_Values).

# Examples
## Control Key
[`Key`] has a variant for most of the keys listed above (see next example for typed chars).
```
use sap::events::*;
use web_sys::HtmlButtonElement;

# fn control_key_example(btn: HtmlButtonElement) {
let btn: HtmlButtonElement = // function to get button element
    # btn;
// can confirm by pressing enter on button
dispatch_key_event(&btn, KeyEventType::KeyPress, Key::Enter);
# }
```

## Char literals
A [`char`] can be accepted and will be the equivalent to using [`Key::Lit`] variant with the [`char`]
value.
```
use sap::events::*;
use web_sys::HtmlInputElement;

# fn char_literal_example(input: HtmlInputElement) {
let input: HtmlInputElement = // get input
    # input;
dispatch_key_event(&input, KeyEventType::KeyPress, 'a');
# }
```
*/
pub fn dispatch_key_event<K>(element: &EventTarget, event_type: KeyEventType, key: K)
where
    K: Into<Key>,
{
    let mut event_init = KeyboardEventInit::new();
    event_init.bubbles(true);
    event_init.key(&key.into().to_string());
    let key_event =
        KeyboardEvent::new_with_keyboard_event_init_dict(event_type.into(), &event_init).unwrap();

    element.dispatch_event(&key_event).unwrap();
}

/**
A simple simulation of typing a single key to the [`EventTarget`].

This will fire the following events, in this order, on the target:
- `keydown` [`KeyboardEvent`]
- `keypress` [`KeyboardEvent`]
- `keyup` [`KeyboardEvent`]
- `input` [`InputEvent`]

# Examples
```
use sap::events::*;
use web_sys::HtmlInputElement;

# fn type_key_example(input: HtmlInputElement) {
let input: HtmlInputElement = // some function to get input element;
    # input;
type_key(&input, 'A');
assert_eq!("A", input.value());
# }
```
*/
pub fn type_key<K>(element: &EventTarget, key: K)
where
    K: Into<Key>,
{
    let key = key.into();
    for &key_event_type in [
        KeyEventType::KeyDown,
        KeyEventType::KeyPress,
        KeyEventType::KeyUp,
    ]
    .iter()
    {
        dispatch_key_event(element, key_event_type, key);
    }
    if key.is_visible() {
        dispatch_input_event(element, &key.to_string());
    }
}

/// A simple simulation of typing multiple [`Key`]s to the [`EventTarget`].
///
/// This will fire the following events, in this order, for each [`Key`]:
/// - `keydown` [`KeyboardEvent`]
/// - `keypress` [`KeyboardEvent`]
/// - `keyup` [`KeyboardEvent`]
/// - `input` [`InputEvent`]
///
/// ```
/// use sap::{events::*, type_to};
/// use web_sys::HtmlInputElement;
///
/// # fn type_to_example(input: HtmlInputElement) {
/// let input: HtmlInputElement = // some query to get input element
///     # input;
/// type_to!(input, "Hello,", " World!");
/// assert_eq!("Hello, World!", input.value());
/// # }
///
/// ```
#[macro_export]
macro_rules! type_to {
    ($element: ident, $($into_keys:expr),+) => {
        let mut keys: Vec<Key> = vec![];
        $(
            let mut ks: Keys = $into_keys.into();
            keys.append(&mut ks);
        )+
        for key in keys {
            type_key(&$element, key);
        }
    };
}

/// A newtype around a [`Vec<Key>`] for use with [`type_to!`] macro.
pub struct Keys(Vec<Key>);

impl Deref for Keys {
    type Target = Vec<Key>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Keys {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<&str> for Keys {
    fn from(value: &str) -> Self {
        Self(value.chars().map(Key::Lit).collect())
    }
}

impl From<String> for Keys {
    fn from(value: String) -> Self {
        let value: &str = &value;
        value.into()
    }
}

impl From<Key> for Keys {
    fn from(key: Key) -> Self {
        Self(vec![key])
    }
}

/// Enables firing a `dblclick` [`MouseEvent`].
pub trait DblClick {
    /**
    Fires a `dblclick` [`MouseEvent`] on this [`EventTarget`].

    # Examples
    ```
    use sap::events::DblClick;
    use web_sys::HtmlButtonElement;

    # fn dbl_click_example(btn: HtmlButtonElement) {
    let btn: HtmlButtonElement = // get button from query
        # btn;
    btn.dbl_click();
    # }
    ```
    */
    fn dbl_click(&self)
    where
        Self: AsRef<EventTarget>;
}

impl DblClick for EventTarget {
    fn dbl_click(&self) {
        let mut event_init = MouseEventInit::new();
        event_init.bubbles(true);
        let dbl_click_event = MouseEvent::new("dblclick").unwrap();
        assert!(
            self.dispatch_event(&dbl_click_event).unwrap(),
            "expected dblclick event to be fired."
        );
    }
}

/**
Dispatches a [`InputEvent`] with the `data` given, to the event target.

Input events can only be fired on the following:
- [`HtmlInputElement`]
- [`HtmlSelectElement`]
- [`HtmlTextAreaElement`]

Using the function on other elements will do nothing!

Only use this if you need to trigger an `oninput` event listener - if you want to change the value
of the [`EventTarget`] you can just use the relative set value method.

# Examples
```
use sap::events::dispatch_input_event;
use web_sys::HtmlInputElement;

# fn dispatch_input_event_example(input: HtmlInputElement) {
let input: HtmlInputElement = // function to get input element
    # input;
// enter value into input
dispatch_input_event(&input, "Hello, World!");
assert_eq!("Hello, World!", input.value());
# }
```
*/
pub fn dispatch_input_event(element: &EventTarget, data: &str) {
    let value_updated = sap_utils::get_element_value(element)
        .map(|mut value| {
            value.push_str(data);
            sap_utils::set_element_value(element, &value)
        })
        .unwrap_or_default();

    if value_updated {
        let mut event_init = InputEventInit::new();
        event_init.data(Some(data));
        event_init.bubbles(true);
        let input_event = InputEvent::new_with_event_init_dict("input", &event_init).unwrap();
        assert!(element.dispatch_event(&input_event).unwrap());
    }
}

/// Enables dispatching a bubbling `change` event from an EventTarget
pub trait EventTargetChanged {
    /**
    Dispatches a change [`Event`] on this [`EventTarget`]

    # Examples
    ```
    use sap::events::EventTargetChanged;
    use web_sys::HtmlInputElement;

    # fn dispatch_input_event_example(input: HtmlInputElement) {
    let input: HtmlInputElement = // function to get input element
        # input;
    // dispatch "change" event
    input.changed();
    # }
    ```
    */
    fn changed(&self);
}

impl EventTargetChanged for EventTarget {
    fn changed(&self) {
        let mut event_init = EventInit::new();
        event_init.bubbles(true);
        let change_event = Event::new_with_event_init_dict("change", &event_init).unwrap();
        assert!(self.dispatch_event(&change_event).unwrap());
    }
}

macro_rules! key_impl {
    (
        #[$($key_doc:meta)+]
        pub enum Key {$(
            $(#[$($variant_doc:meta)+])?
            $variant:ident
        ),*$(,)*}
    ) => {
        #[$($key_doc)+]
        #[derive(Copy, Clone)]
        pub enum Key {
            /**
            A literal key such as an alphanumeric or even the single space ' '.
            This also allows for special characters such as '🎉'.
            */
            Lit(char),
            $(
                #[allow(missing_docs)] // might have to keep this or document every key...
                $(#[$($variant_doc)+])?
                $variant,
            )*
        }

        impl Key {
            fn is_visible(&self) -> bool {
                match self {
                    Key::Lit(_) => true,
                    _ => false,
                }
            }
        }

        impl std::fmt::Display for Key {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use std::fmt::Write;

                match self {
                    Key::Lit(c) => f.write_char(*c),
                    $(
                        Key::$variant => f.write_str(stringify!($variant)),
                    )*
                }
            }
        }
    }
}

impl From<char> for Key {
    fn from(c: char) -> Self {
        Self::Lit(c)
    }
}

key_impl! {
    /**
    Standard key value to be used to represent the [`KeyboardEvent::key()`].
    The [list of keys](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key/Key_Values)
    used can be found on MDN.
    */
    pub enum Key {
        // Modifier keys
        Alt,
        AltGraph,
        CapsLock,
        Control,
        Fn,
        FnLock,
        Hyper,
        Meta,
        NumLock,
        ScrollLock,
        Shift,
        Super,
        Symbol,
        SymbolLock,
        // Whitespace keys
        Enter,
        Tab,
        // Navigation keys
        ArrowDown,
        ArrowLeft,
        ArrowRight,
        ArrowUp,
        End,
        Home,
        PageDown,
        PageUp,
        // Editing keys
        Backspace,
        Clear,
        Copy,
        CrSel,
        Cut,
        Delete,
        EraseEof,
        ExSel,
        Insert,
        Paste,
        Redo,
        Undo,
        // UI keys
        Accept,
        Again,
        Attn,
        Cancel,
        ContextMenu,
        Escape,
        Execute,
        Find,
        Finish,
        Help,
        Pause,
        Play,
        Props,
        Select,
        ZoomIn,
        ZoomOut,
        // Device keys
        BrightnessDown,
        BrightnessUp,
        Eject,
        LogOff,
        Power,
        PowerOff,
        PrintScreen,
        Hibernate,
        Standby,
        WakeUp,
        // Common IME keys
        AllCandidates,
        Alphanumeric,
        CodeInput,
        Compose,
        Convert,
        Dead,
        FinalMode,
        GroupFirst,
        GroupLast,
        GroupNext,
        GroupPrevious,
        ModeChange,
        NextCandidate,
        NonConvert,
        PreviousCandidate,
        Process,
        SingleCandidate,
        /// Korean keyboards only
        HangulMode,
        /// Korean keyboards only
        HanjaMode,
        /// Korean keyboards only
        JunjaMode,
        /// Japanese keyboards only
        Eisu,
        /// Japanese keyboards only
        Hankaku,
        /// Japanese keyboards only
        Hiragana,
        /// Japanese keyboards only
        HiraganaKatakana,
        /// Japanese keyboards only
        KanaMode,
        /// Japanese keyboards only
        KanjiMode,
        /// Japanese keyboards only
        Katakana,
        /// Japanese keyboards only
        Romaji,
        /// Japanese keyboards only
        Zenkaku,
        /// Japanese keyboards only
        ZenkakuHanaku,
        // Function keys
        F1,
        F2,
        F3,
        F4,
        F5,
        F6,
        F7,
        F8,
        F9,
        F10,
        F11,
        F12,
        F13,
        F14,
        F15,
        F16,
        F17,
        F18,
        F19,
        F20,
        Soft1,
        Soft2,
        Soft3,
        Soft4,
        // Phone keys
        AppSwitch,
        Call,
        Camera,
        CameraFocus,
        EndCall,
        GoBack,
        GoHome,
        HeadsetHook,
        LastNumberRedial,
        Notification,
        MannerMode,
        VoiceDial,
        // Multimedia keys
        ChannelDown,
        ChannelUp,
        MediaFastForward,
        MediaPause,
        MediaPlay,
        MediaPlayPause,
        MediaRecord,
        MediaRewind,
        MediaStop,
        MediaTrackNext,
        MediaTrackPrevious,
        // Audio control keys
        AudioBalanceLeft,
        AudioBalanceRight,
        AudioBassDown,
        AudioBassBoostDown,
        AudioBassBoostToggle,
        AudioBassBoostUp,
        AudioBassUp,
        AudioFaderFront,
        AudioFaderRear,
        AudioSurroundModeNext,
        AudioTrebleDown,
        AudioTrebleUp,
        AudioVolumeDown,
        AudioVolumeMute,
        AudioVolumeUp,
        MicrophoneToggle,
        MicrophoneVolumeDown,
        MicrophoneVolumeMute,
        MicrophoneVolumeUp,
        // TV control keys
        TV,
        TV3DMode,
        TVAntennaCable,
        TVAudioDescription,
        TVAudioDescriptionMixDown,
        TVAudioDescriptionMixUp,
        TVContentMenu,
        TVDataService,
        TVInput,
        TVInputComponent1,
        TVInputComponent2,
        TVInputComposite1,
        TVInputComposite2,
        TVInputHDMI1,
        TVInputHDMI2,
        TVInputHDMI3,
        TVInputHDMI4,
        TVInputVGA1,
        TVMediaContext,
        TVNetwork,
        TVNumberEntry,
        TVPower,
        TVRadioService,
        TVSatellite,
        TVSatelliteBS,
        TVSatelliteCS,
        TVSatelliteToggle,
        TVTerrestrialAnalog,
        TVTerrestrialDigital,
        TVTimer,
        // Media controller keys
        AVRInput,
        AVRPower,
        ColorF0Red,
        ColorF1Green,
        ColorF2Yellow,
        ColorF3Blue,
        ColorF4Grey,
        ColorF5Brown,
        ClosedCaptionToggle,
        Dimmer,
        DisplaySwag,
        #[allow(clippy::clippy::upper_case_acronyms)]
        DVR,
        Exit,
        FavoriteClear0,
        FavoriteClear1,
        FavoriteClear2,
        FavoriteClear3,
        FavoriteRecall0,
        FavoriteRecall1,
        FavoriteRecall2,
        FavoriteRecall3,
        FavoriteStore0,
        FavoriteStore1,
        FavoriteStore2,
        FavoriteStore3,
        Guide,
        GuideNextDay,
        GuidePreviousDay,
        Info,
        InstantReplay,
        Link,
        ListProgram,
        LiveContent,
        Lock,
        MediaApps,
        MediaAudioTrack,
        MediaLast,
        MediaSkipBackward,
        MediaSkipForward,
        MediaStepBackward,
        MediaStepForward,
        MediaTopMenu,
        NavigateIn,
        NavigateNext,
        NavigateOut,
        NavigatePrevious,
        NextFavoriteChannel,
        NextUserProfile,
        OnDemand,
        Pairing,
        PinPDown,
        PinPMove,
        PinPToggle,
        PinPUp,
        PlaySpeedDown,
        PlaySpeedReset,
        PlaySpeedUp,
        RandomToggle,
        RcLowBattery,
        RecordSpeedNext,
        RfBypass,
        ScanChannelsToggle,
        ScreenModeNext,
        Settings,
        SplitScreenToggle,
        STBInput,
        STBPower,
        Subtitle,
        Teletext,
        VideoModeNext,
        Wink,
        ZoomToggle,
        // Speech recognition keys
        SpeechCorrectionList,
        SpeechInputToggle,
        // Document keys
        Close,
        New,
        Open,
        Print,
        Save,
        SpellCheck,
        MailForward,
        MailReply,
        MailSend,
        // Application selector keys
        LaunchCalculator,
        LaunchCalendar,
        LaunchContacts,
        LaunchMail,
        LaunchMediaPlayer,
        LaunchMusicPlayer,
        LaunchMyComputer,
        LaunchPhone,
        LaunchScreenSaver,
        LaunchSpreadsheet,
        LaunchWebBrowser,
        LaunchWebCam,
        LaunchWordProcessor,
        LaunchApplication1,
        LaunchApplication2,
        LaunchApplication3,
        LaunchApplication4,
        LaunchApplication5,
        LaunchApplication6,
        LaunchApplication7,
        LaunchApplication8,
        LaunchApplication9,
        LaunchApplication10,
        LaunchApplication11,
        LaunchApplication12,
        LaunchApplication13,
        LaunchApplication14,
        LaunchApplication15,
        LaunchApplication16,
        // Browser control keys
        BrowserBack,
        BrowserFavorites,
        BrowserForward,
        BrowserHome,
        BrowserRefresh,
        BrowserSearch,
        BrowserStop,
        Decimal,
        Key11,
        Key12,
        Multiply,
        Add,
        Divide,
        Subtract,
        Separator,
    }
}

/// An enum for the possible event types for [`KeyboardEvent`]s.
#[derive(Clone, Copy)]
pub enum KeyEventType {
    /// The `keydown` event type.
    KeyDown,
    /// The `keyup` event type.
    KeyUp,
    /// The `keypress` event type
    KeyPress,
}

#[allow(clippy::from_over_into)]
impl Into<&str> for KeyEventType {
    fn into(self) -> &'static str {
        match self {
            KeyEventType::KeyDown => "keydown",
            KeyEventType::KeyUp => "keyup",
            KeyEventType::KeyPress => "keypress",
        }
    }
}

#[cfg(test)]
mod tests {

    use std::cell::Cell;

    use wasm_bindgen::{prelude::Closure, JsCast};
    use web_sys::{HtmlElement, HtmlInputElement, KeyboardEvent};
    use yew::prelude::*;

    struct KeyDemo {
        link: ComponentLink<Self>,
        last_key_pressed: Option<String>,
    }

    impl Component for KeyDemo {
        type Message = KeyboardEvent;
        type Properties = ();

        fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
            Self {
                link,
                last_key_pressed: None,
            }
        }

        fn update(&mut self, msg: Self::Message) -> ShouldRender {
            self.last_key_pressed = Some(msg.key());
            true
        }

        fn change(&mut self, _props: Self::Properties) -> ShouldRender {
            false
        }

        fn view(&self) -> Html {
            let default = "None".to_owned();
            let last_key_value = self.last_key_pressed.as_ref().unwrap_or(&default);
            html! {
                <>
                    <p>{ last_key_value }</p>
                    <label for="input">{ "input label" }</label>
                    <input id="input" placeholder="key" type="text" onkeydown={self.link.callback(|e| e)} />
                </>
            }
        }
    }

    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use crate::{assert_text_content, TestRender};
    use sap_yew::test_render;

    use super::*;
    use crate::prelude::*;

    #[wasm_bindgen_test]
    fn sim_typing_to_input_and_enter_to_confirm() {
        let rendered = test_render! {
            <KeyDemo />
        };

        let last_key_value: HtmlElement = rendered.get_by_text("None").unwrap();
        let input: HtmlInputElement = rendered.get_by_placeholder_text("key").unwrap();

        assert_text_content!("None", last_key_value);

        dispatch_key_event(&input, KeyEventType::KeyDown, Key::Enter);

        assert_text_content!("Enter", last_key_value);

        type_to!(input, "hello");

        assert_text_content!("o", last_key_value);
        assert_eq!("hello", input.value());

        dispatch_key_event(&input, KeyEventType::KeyDown, '🎉');

        assert_text_content!('🎉', last_key_value);
    }

    struct InputDemo;

    impl Component for InputDemo {
        type Message = InputData;
        type Properties = ();

        fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
            Self
        }

        fn update(&mut self, _: Self::Message) -> ShouldRender {
            false
        }

        fn change(&mut self, _props: Self::Properties) -> ShouldRender {
            false
        }

        fn view(&self) -> Html {
            html! {
                <>
                    <input placeholder="key" type="text" />
                </>
            }
        }
    }

    #[wasm_bindgen_test]
    fn type_to_input() {
        let rendered = test_render! {
            <InputDemo />
        };

        let input: HtmlInputElement = rendered.get_by_placeholder_text("key").unwrap();

        type_to!(input, "hello");

        assert_eq!("hello", input.value());
    }

    #[wasm_bindgen_test]
    fn trigger_on_change_event() {
        thread_local! {
            static FLAG: Cell<bool> = Default::default();
        }

        let rendered = test_render! {
            <InputDemo />
        };

        let input: HtmlInputElement = rendered.get_by_placeholder_text("key").unwrap();

        let listener =
            Closure::wrap(Box::new(move |_| FLAG.with(|v| v.set(true))) as Box<dyn Fn(Event)>);

        rendered
            .add_event_listener_with_callback("change", listener.as_ref().unchecked_ref())
            .unwrap();

        input.changed();

        assert!(FLAG.with(|v| v.get()));

        // clean up
        rendered
            .remove_event_listener_with_callback("change", listener.as_ref().unchecked_ref())
            .unwrap();
    }
}
