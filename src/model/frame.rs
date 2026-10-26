use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActionPoint {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub color: [u8; 4],
}

impl Default for ActionPoint {
    fn default() -> Self {
        Self {
            name: String::from("action_point"),
            x: 0,
            y: 0,
            color: [0, 220, 255, 255],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PivotConfig {
    pub show_gizmos: bool,
    pub origin_x: i32,
    pub origin_y: i32,
    pub action_points: Vec<ActionPoint>,
}

impl Default for PivotConfig {
    fn default() -> Self {
        Self {
            show_gizmos: false,
            origin_x: 16,
            origin_y: 32,
            action_points: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerMeta {
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    pub opacity: f32,
    #[serde(default)]
    pub embedded_base64: Option<String>,
    #[serde(skip)]
    pub layer_rgba: Option<image::RgbaImage>,
}

impl PartialEq for LayerMeta {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.visible == other.visible
            && self.locked == other.locked
            && (self.opacity - other.opacity).abs() < 1e-5
            && self.embedded_base64 == other.embedded_base64
    }
}

impl Default for LayerMeta {
    fn default() -> Self {
        Self {
            name: String::from("Layer 1"),
            visible: true,
            locked: false,
            opacity: 1.0,
            embedded_base64: None,
            layer_rgba: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FrameMeta {
    /// Duration multiplier in animation ticks (e.g., 1 = standard frame time, 3 = hold 3x ticks).
    pub hold_multiplier: u32,
    /// If Some(linked_index), this frame shares pixel content with frame `linked_index`.
    #[serde(default)]
    pub linked_from: Option<usize>,
}

impl Default for FrameMeta {
    fn default() -> Self {
        Self {
            hold_multiplier: 1,
            linked_from: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TagDirection {
    #[default]
    #[serde(rename = "forward")]
    Forward,
    #[serde(rename = "reverse")]
    Reverse,
    #[serde(rename = "pingpong")]
    PingPong,
}

impl TagDirection {
    pub fn as_direction_str(self) -> &'static str {
        match self {
            Self::Forward => "forward",
            Self::Reverse => "reverse",
            Self::PingPong => "pingpong",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FrameTag {
    pub name: String,
    pub start_frame: usize,
    pub end_frame: usize,
    pub color: [u8; 4],
    #[serde(default)]
    pub direction: TagDirection,
}

impl Default for FrameTag {
    fn default() -> Self {
        Self {
            name: String::from("idle"),
            start_frame: 0,
            end_frame: 0,
            color: [110, 175, 235, 255],
            direction: TagDirection::Forward,
        }
    }
}
