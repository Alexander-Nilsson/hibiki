use gtk4_layer_shell::Edge;
use serde::{Deserialize, Serialize};

const DEFAULT_DISPLAY_TIMEOUT_MS: u64 = 2000;
const DEFAULT_BUBBLE_TIMEOUT_MS: u64 = 10000;
const DEFAULT_MAX_KEYS: usize = 5;
const DEFAULT_MARGIN: i32 = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum DisplayMode {
    #[default]
    Keystroke,
    Bubble,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Position {
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    Center,
    MiddleRight,
    BottomLeft,
    #[default]
    BottomCenter,
    BottomRight,
}

impl Position {
    #[must_use]
    pub fn layer_shell_edges(self) -> Vec<(Edge, bool)> {
        match self {
            Position::TopLeft => vec![
                (Edge::Top, true),
                (Edge::Left, true),
                (Edge::Bottom, false),
                (Edge::Right, false),
            ],
            Position::TopCenter => vec![
                (Edge::Top, true),
                (Edge::Left, false),
                (Edge::Bottom, false),
                (Edge::Right, false),
            ],
            Position::TopRight => vec![
                (Edge::Top, true),
                (Edge::Left, false),
                (Edge::Bottom, false),
                (Edge::Right, true),
            ],
            Position::MiddleLeft => vec![
                (Edge::Top, false),
                (Edge::Left, true),
                (Edge::Bottom, false),
                (Edge::Right, false),
            ],
            Position::Center => vec![
                (Edge::Top, false),
                (Edge::Left, false),
                (Edge::Bottom, false),
                (Edge::Right, false),
            ],
            Position::MiddleRight => vec![
                (Edge::Top, false),
                (Edge::Left, false),
                (Edge::Bottom, false),
                (Edge::Right, true),
            ],
            Position::BottomLeft => vec![
                (Edge::Top, false),
                (Edge::Left, true),
                (Edge::Bottom, true),
                (Edge::Right, false),
            ],
            Position::BottomCenter => vec![
                (Edge::Top, false),
                (Edge::Left, false),
                (Edge::Bottom, true),
                (Edge::Right, false),
            ],
            Position::BottomRight => vec![
                (Edge::Top, false),
                (Edge::Left, false),
                (Edge::Bottom, true),
                (Edge::Right, true),
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct AudioConfig {
    pub enabled: bool,
    pub volume: f32,
    pub sound_pack: String,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            volume: 1.0,
            sound_pack: "cherrymx-blue-abs".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct BubbleConfig {
    pub font_family: String,
    pub font_size: f64,
    pub color: String,
    pub audio: AudioConfig,
    pub position: Position,
    pub draggable: bool,
    pub hotkey: String,
    pub timeout_ms: u64,
}

impl Default for BubbleConfig {
    fn default() -> Self {
        Self {
            font_family: "Sans".to_string(),
            font_size: 1.0,
            color: "#3584e4".to_string(),
            audio: AudioConfig::default(),
            position: Position::TopRight,
            draggable: false,
            hotkey: "<Shift><Control>b".to_string(),
            timeout_ms: DEFAULT_BUBBLE_TIMEOUT_MS,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
#[allow(clippy::struct_excessive_bools)]
pub struct KeystrokeConfig {
    pub display_mode: DisplayMode,

    pub position: Position,

    pub display_timeout_ms: u64,

    pub max_keys: usize,

    pub margin: i32,

    pub show_modifiers: bool,

    pub all_keyboards: bool,

    pub font_scale: f64,

    pub opacity: f64,

    pub font_family: String,

    pub font_size: f64,

    pub keystroke_theme: String,

    pub keystroke_draggable: bool,

    pub keystroke_hotkey: String,

    pub pause_hotkey: String,

    pub toggle_focus_hotkey: String,

    pub auto_detect_layout: bool,

    pub keyboard_layout: Option<String>,

    pub bubble: BubbleConfig,

    pub audio: AudioConfig,
}

impl Default for KeystrokeConfig {
    fn default() -> Self {
        Self {
            display_mode: DisplayMode::Keystroke,
            position: Position::BottomCenter,
            display_timeout_ms: DEFAULT_DISPLAY_TIMEOUT_MS,
            max_keys: DEFAULT_MAX_KEYS,
            margin: DEFAULT_MARGIN,
            show_modifiers: true,
            all_keyboards: true,
            font_scale: 1.0,
            opacity: 0.9,
            font_family: "Sans".to_string(),
            font_size: 1.2,
            keystroke_theme: "system".to_string(),
            keystroke_draggable: false,
            keystroke_hotkey: "<Shift><Control>k".to_string(),
            pause_hotkey: "<Control>p".to_string(),
            toggle_focus_hotkey: "<Control>b".to_string(),
            auto_detect_layout: true,
            keyboard_layout: None,
            bubble: BubbleConfig::default(),
            audio: AudioConfig::default(),
        }
    }
}
