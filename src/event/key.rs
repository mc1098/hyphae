///! Key

/// A newtype around a [`Vec<Key>`] for use with [`type_to!`] macro.
pub struct Keys(Vec<Key>);

impl std::ops::Deref for Keys {
    type Target = Vec<Key>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Keys {
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

impl From<Vec<Key>> for Keys {
    fn from(keys: Vec<Key>) -> Self {
        Self(keys)
    }
}

/// An enum for the possible event types for [`web_sys::KeyboardEvent`]s.
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

impl From<char> for Key {
    fn from(c: char) -> Self {
        Self::Lit(c)
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
            /// A literal key such as an alphanumeric or even the single space ' '.
            /// This also allows for special characters such as '🎉'.
            Lit(char),
            $(
                #[allow(missing_docs)] // might have to keep this or document every key...
                $(#[$($variant_doc)+])?
                $variant,
            )*
        }

        impl Key {
            /// Returns whether this key is visible
            pub fn is_visible(&self) -> bool {
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

key_impl! {
    /**
    Standard key value to be used to represent the [`web_sys::KeyboardEvent::key()`].
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
