use crate::model::metadata::ProjectMetadata;
use image::RgbaImage;

pub const MAX_HISTORY_DEPTH: usize = 100;

fn max_depth_for_resolution(w: u32, h: u32, layer_count: usize) -> usize {
    let layers = layer_count.max(1) as u64;
    let pixels = (w as u64) * (h as u64) * layers;
    if pixels <= 256 * 256 {
        MAX_HISTORY_DEPTH
    } else if pixels <= 512 * 512 {
        60
    } else if pixels <= 1024 * 1024 {
        30
    } else {
        15
    }
}

#[derive(Clone)]
pub struct HistorySnapshot {
    pub atlas_rgba: RgbaImage,
    pub metadata: ProjectMetadata,
    pub active_frame: usize,
}

use std::collections::VecDeque;

#[derive(Default)]
pub struct HistoryStack {
    undo_stack: VecDeque<HistorySnapshot>,
    redo_stack: VecDeque<HistorySnapshot>,
}

impl HistoryStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, snapshot: HistorySnapshot) {
        let max_depth = max_depth_for_resolution(
            snapshot.atlas_rgba.width(),
            snapshot.atlas_rgba.height(),
            snapshot.metadata.layers.len(),
        );
        while self.undo_stack.len() >= max_depth {
            self.undo_stack.pop_front();
        }
        self.undo_stack.push_back(snapshot);
        self.redo_stack.clear();
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    pub fn undo(&mut self, current: HistorySnapshot) -> Option<HistorySnapshot> {
        let prev = self.undo_stack.pop_back()?;
        let max_depth = max_depth_for_resolution(
            current.atlas_rgba.width(),
            current.atlas_rgba.height(),
            current.metadata.layers.len(),
        );
        while self.redo_stack.len() >= max_depth {
            self.redo_stack.pop_front();
        }
        self.redo_stack.push_back(current);
        Some(prev)
    }

    pub fn redo(&mut self, current: HistorySnapshot) -> Option<HistorySnapshot> {
        let next = self.redo_stack.pop_back()?;
        let max_depth = max_depth_for_resolution(
            current.atlas_rgba.width(),
            current.atlas_rgba.height(),
            current.metadata.layers.len(),
        );
        while self.undo_stack.len() >= max_depth {
            self.undo_stack.pop_front();
        }
        self.undo_stack.push_back(current);
        Some(next)
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}
