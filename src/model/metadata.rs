use crate::model::frame::{FrameMeta, FrameTag, LayerMeta, PivotConfig};
use crate::model::palette::PaletteData;
use crate::model::sketch::{OnionSkinSettings, SketchReferenceState};
use serde::{Deserialize, Serialize};

pub const METADATA_FORMAT_VERSION: &str = "1.0";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectMetadata {
    pub format_version: String,
    pub project_name: String,
    pub frame_width: u32,
    pub frame_height: u32,
    pub columns: u32,
    pub rows: u32,
    pub frame_count: u32,
    pub target_color_limit: Option<u32>,
    pub fps: u32,
    #[serde(default)]
    pub frames: Vec<FrameMeta>,
    #[serde(default)]
    pub layers: Vec<LayerMeta>,
    #[serde(default)]
    pub active_layer: usize,
    #[serde(default)]
    pub tags: Vec<FrameTag>,
    #[serde(default)]
    pub palette: PaletteData,
    #[serde(default)]
    pub pivot: PivotConfig,
    #[serde(default)]
    pub sketch: SketchReferenceState,
    #[serde(default)]
    pub onion_skin: OnionSkinSettings,
}

impl Default for ProjectMetadata {
    fn default() -> Self {
        Self::new_config("sprite".to_string(), 32, 32, 4, 4, Some(16))
    }
}

impl ProjectMetadata {
    pub fn new_config(
        project_name: String,
        frame_width: u32,
        frame_height: u32,
        frame_count: u32,
        columns: u32,
        target_color_limit: Option<u32>,
    ) -> Self {
        let safe_cols = columns.max(1).min(frame_count.max(1));
        let safe_frames = frame_count.max(1);
        let rows = safe_frames.div_ceil(safe_cols);

        let frames = (0..safe_frames).map(|_| FrameMeta::default()).collect();
        let tags = vec![FrameTag {
            name: String::from("idle"),
            start_frame: 0,
            end_frame: (safe_frames as usize).saturating_sub(1),
            color: [110, 175, 235, 255],
            direction: crate::model::frame::TagDirection::Forward,
        }];

        let pivot = PivotConfig {
            origin_x: (frame_width / 2) as i32,
            origin_y: frame_height as i32,
            ..Default::default()
        };

        Self {
            format_version: METADATA_FORMAT_VERSION.to_string(),
            project_name,
            frame_width: frame_width.max(1),
            frame_height: frame_height.max(1),
            columns: safe_cols,
            rows: rows.max(1),
            frame_count: safe_frames,
            target_color_limit,
            fps: 12,
            frames,
            layers: vec![LayerMeta::default()],
            active_layer: 0,
            tags,
            palette: PaletteData::default(),
            pivot,
            sketch: SketchReferenceState::default(),
            onion_skin: OnionSkinSettings::default(),
        }
    }

    pub fn atlas_dimensions(&self) -> (u32, u32) {
        (
            self.frame_width.saturating_mul(self.columns.max(1)),
            self.frame_height.saturating_mul(self.rows.max(1)),
        )
    }

    /// Returns the top-left pixel coordinates `(x, y)` of a given frame index in the atlas.
    pub fn frame_origin(&self, frame_index: usize) -> (u32, u32) {
        let cols = self.columns.max(1);
        let idx = (frame_index as u32).min(self.frame_count.saturating_sub(1));
        let col = idx % cols;
        let row = idx / cols;
        (col * self.frame_width, row * self.frame_height)
    }
}
