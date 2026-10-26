use crate::model::frame::{FrameMeta, LayerMeta};
use crate::model::metadata::ProjectMetadata;
use crate::model::sketch::nearest_palette_color;
use crate::tools::selection::SelectionMask;
use egui::ColorImage;
use image::{Rgba, RgbaImage};
use std::collections::HashSet;
use std::path::PathBuf;

pub struct Project {
    pub metadata: ProjectMetadata,
    pub atlas_rgba: RgbaImage,
    pub sketch_rgba: Option<RgbaImage>,
    pub active_frame: usize,
    pub file_path: Option<PathBuf>,
    pub canvas_dirty: bool,
    pub sketch_dirty: bool,
    pub has_unsaved_changes: bool,
}

impl Default for Project {
    fn default() -> Self {
        Self::new(ProjectMetadata::default())
    }
}

impl Project {
    pub fn new(metadata: ProjectMetadata) -> Self {
        let (atlas_w, atlas_h) = metadata.atlas_dimensions();
        let atlas_rgba = RgbaImage::from_pixel(atlas_w.max(1), atlas_h.max(1), Rgba([0, 0, 0, 0]));

        let mut proj = Self {
            metadata,
            atlas_rgba,
            sketch_rgba: None,
            active_frame: 0,
            file_path: None,
            canvas_dirty: true,
            sketch_dirty: true,
            has_unsaved_changes: false,
        };
        proj.ensure_layers_initialized();
        proj
    }

    pub fn from_loaded(
        metadata: ProjectMetadata,
        atlas_rgba: RgbaImage,
        file_path: Option<PathBuf>,
    ) -> Self {
        let mut proj = Self {
            metadata,
            atlas_rgba,
            sketch_rgba: None,
            active_frame: 0,
            file_path,
            canvas_dirty: true,
            sketch_dirty: true,
            has_unsaved_changes: false,
        };
        proj.ensure_layers_initialized();
        proj
    }

    /// Ensures at least one layer exists in `metadata.layers` and every layer's `layer_rgba`
    /// matches `atlas_rgba` dimensions.
    pub fn ensure_layers_initialized(&mut self) {
        if self.metadata.layers.is_empty() {
            self.metadata.layers.push(LayerMeta::default());
        }
        self.metadata.active_layer = self
            .metadata
            .active_layer
            .min(self.metadata.layers.len().saturating_sub(1));

        let w = self.atlas_rgba.width().max(1);
        let h = self.atlas_rgba.height().max(1);
        let active_idx = self.metadata.active_layer;

        for (idx, layer) in self.metadata.layers.iter_mut().enumerate() {
            if idx == active_idx {
                layer.layer_rgba = Some(self.atlas_rgba.clone());
            } else {
                let needs_init = match &layer.layer_rgba {
                    Some(img) => img.width() != w || img.height() != h,
                    None => true,
                };
                if needs_init {
                    let mut resized = RgbaImage::from_pixel(w, h, Rgba([0, 0, 0, 0]));
                    if let Some(old) = &layer.layer_rgba {
                        let cw = w.min(old.width());
                        let ch = h.min(old.height());
                        for y in 0..ch {
                            for x in 0..cw {
                                resized.put_pixel(x, y, *old.get_pixel(x, y));
                            }
                        }
                    }
                    layer.layer_rgba = Some(resized);
                }
            }
        }
    }

    /// Synchronizes `self.atlas_rgba` into the active layer's `layer_rgba` buffer.
    pub fn sync_active_layer(&mut self) {
        if self.metadata.layers.is_empty() {
            self.ensure_layers_initialized();
            return;
        }
        let active_idx = self
            .metadata
            .active_layer
            .min(self.metadata.layers.len().saturating_sub(1));
        self.metadata.active_layer = active_idx;
        if let Some(layer) = self.metadata.layers.get_mut(active_idx) {
            layer.layer_rgba = Some(self.atlas_rgba.clone());
        }
    }

    /// Returns whether the currently active layer is locked or hidden.
    pub fn active_layer_locked(&self) -> bool {
        let active_idx = self
            .metadata
            .active_layer
            .min(self.metadata.layers.len().saturating_sub(1));
        self.metadata
            .layers
            .get(active_idx)
            .map(|l| l.locked || !l.visible)
            .unwrap_or(false)
    }

    /// Switches the active layer to `index`, saving the current `atlas_rgba` into the previous
    /// layer and loading the target layer's `layer_rgba` into `atlas_rgba`.
    pub fn set_active_layer(&mut self, index: usize) {
        self.ensure_layers_initialized();
        let target = index.min(self.metadata.layers.len().saturating_sub(1));
        if target == self.metadata.active_layer {
            self.sync_active_layer();
            return;
        }
        self.sync_active_layer();
        self.metadata.active_layer = target;
        if let Some(img) = self
            .metadata
            .layers
            .get(target)
            .and_then(|l| l.layer_rgba.clone())
        {
            self.atlas_rgba = img;
        }
        self.canvas_dirty = true;
    }

    /// Adds a new blank layer at the bottom of the layer track list and makes it active.
    pub fn add_layer(&mut self, name: String) -> usize {
        self.ensure_layers_initialized();
        self.sync_active_layer();
        let w = self.atlas_rgba.width().max(1);
        let h = self.atlas_rgba.height().max(1);
        let blank = RgbaImage::from_pixel(w, h, Rgba([0, 0, 0, 0]));
        self.metadata.layers.push(LayerMeta {
            name,
            visible: true,
            locked: false,
            opacity: 1.0,
            embedded_base64: None,
            layer_rgba: Some(blank.clone()),
        });
        let new_idx = self.metadata.layers.len() - 1;
        self.metadata.active_layer = new_idx;
        self.atlas_rgba = blank;
        self.canvas_dirty = true;
        self.has_unsaved_changes = true;
        new_idx
    }

    /// Duplicates the layer at `index` and inserts the copy right after it.
    pub fn duplicate_layer(&mut self, index: usize) -> usize {
        self.ensure_layers_initialized();
        self.sync_active_layer();
        let safe_idx = index.min(self.metadata.layers.len().saturating_sub(1));
        let mut cloned = self.metadata.layers[safe_idx].clone();
        cloned.name = format!("{} Copy", cloned.name);
        let insert_idx = safe_idx + 1;
        self.metadata.layers.insert(insert_idx, cloned);
        self.metadata.active_layer = insert_idx;
        if let Some(img) = self.metadata.layers[insert_idx].layer_rgba.clone() {
            self.atlas_rgba = img;
        }
        self.canvas_dirty = true;
        self.has_unsaved_changes = true;
        insert_idx
    }

    /// Deletes the layer at `index` if more than 1 layer exists.
    pub fn delete_layer(&mut self, index: usize) -> bool {
        self.ensure_layers_initialized();
        if self.metadata.layers.len() <= 1 || index >= self.metadata.layers.len() {
            return false;
        }
        self.sync_active_layer();
        self.metadata.layers.remove(index);
        if self.metadata.active_layer >= self.metadata.layers.len() {
            self.metadata.active_layer = self.metadata.layers.len() - 1;
        } else if self.metadata.active_layer > index {
            self.metadata.active_layer -= 1;
        }
        if let Some(img) = self.metadata.layers[self.metadata.active_layer]
            .layer_rgba
            .clone()
        {
            self.atlas_rgba = img;
        }
        self.canvas_dirty = true;
        self.has_unsaved_changes = true;
        true
    }

    /// Swaps two layers `idx_a` and `idx_b` in the layer list.
    pub fn swap_layers(&mut self, idx_a: usize, idx_b: usize) -> bool {
        self.ensure_layers_initialized();
        if idx_a >= self.metadata.layers.len()
            || idx_b >= self.metadata.layers.len()
            || idx_a == idx_b
        {
            return false;
        }
        self.sync_active_layer();
        self.metadata.layers.swap(idx_a, idx_b);
        if self.metadata.active_layer == idx_a {
            self.metadata.active_layer = idx_b;
        } else if self.metadata.active_layer == idx_b {
            self.metadata.active_layer = idx_a;
        }
        if let Some(img) = self.metadata.layers[self.metadata.active_layer]
            .layer_rgba
            .clone()
        {
            self.atlas_rgba = img;
        }
        self.canvas_dirty = true;
        self.has_unsaved_changes = true;
        true
    }

    /// Merges layer `index` onto the layer below it in composite order (`index - 1` if `index > 0`,
    /// otherwise `index + 1` onto `index`).
    pub fn merge_layer_down(&mut self, index: usize) -> bool {
        self.ensure_layers_initialized();
        if self.metadata.layers.len() <= 1 || index >= self.metadata.layers.len() {
            return false;
        }
        self.sync_active_layer();
        // Since layers in self.metadata.layers are ordered top-to-bottom (V1 at 0 is on top),
        // merging "down" means merging `index` into the layer below it in the timeline, which is `index + 1`.
        let (top_idx, base_idx) = if index + 1 < self.metadata.layers.len() {
            (index, index + 1)
        } else {
            (index, index - 1)
        };
        let w = self.atlas_rgba.width().max(1);
        let h = self.atlas_rgba.height().max(1);
        let base_img = self.metadata.layers[base_idx]
            .layer_rgba
            .clone()
            .unwrap_or_else(|| RgbaImage::from_pixel(w, h, Rgba([0, 0, 0, 0])));
        let top_img = self.metadata.layers[top_idx]
            .layer_rgba
            .clone()
            .unwrap_or_else(|| RgbaImage::from_pixel(w, h, Rgba([0, 0, 0, 0])));
        let top_opacity = if self.metadata.layers[top_idx].visible {
            self.metadata.layers[top_idx].opacity.clamp(0.0, 1.0)
        } else {
            0.0
        };

        let mut merged = base_img;
        if top_opacity > 0.0 {
            for y in 0..h.min(top_img.height()) {
                for x in 0..w.min(top_img.width()) {
                    let sp = top_img.get_pixel(x, y);
                    let sa = (sp[3] as f32 / 255.0) * top_opacity;
                    if sa <= 0.0 {
                        continue;
                    }
                    let dp = merged.get_pixel(x, y);
                    let da = dp[3] as f32 / 255.0;
                    let out_a = sa + da * (1.0 - sa);
                    if out_a > 0.0 {
                        let r = ((sp[0] as f32 * sa + dp[0] as f32 * da * (1.0 - sa)) / out_a)
                            .round()
                            .clamp(0.0, 255.0) as u8;
                        let g = ((sp[1] as f32 * sa + dp[1] as f32 * da * (1.0 - sa)) / out_a)
                            .round()
                            .clamp(0.0, 255.0) as u8;
                        let b = ((sp[2] as f32 * sa + dp[2] as f32 * da * (1.0 - sa)) / out_a)
                            .round()
                            .clamp(0.0, 255.0) as u8;
                        let a = (out_a * 255.0).round().clamp(0.0, 255.0) as u8;
                        merged.put_pixel(x, y, Rgba([r, g, b, a]));
                    }
                }
            }
        }

        self.metadata.layers[base_idx].layer_rgba = Some(merged.clone());
        self.metadata.layers.remove(top_idx);
        let new_active = if top_idx < base_idx {
            base_idx - 1
        } else {
            base_idx
        };
        self.metadata.active_layer = new_active.min(self.metadata.layers.len() - 1);
        self.atlas_rgba = merged;
        self.canvas_dirty = true;
        self.has_unsaved_changes = true;
        true
    }

    /// Alpha-blends all visible layers in `0..metadata.layers.len()` order into a single composited atlas.
    pub fn composited_atlas_rgba(&self) -> RgbaImage {
        let w = self.atlas_rgba.width().max(1);
        let h = self.atlas_rgba.height().max(1);
        if self.metadata.layers.is_empty() {
            return self.atlas_rgba.clone();
        }
        if self.metadata.layers.len() == 1 {
            let layer = &self.metadata.layers[0];
            if !layer.visible || layer.opacity <= 0.0 {
                return RgbaImage::from_pixel(w, h, Rgba([0, 0, 0, 0]));
            }
            if (layer.opacity - 1.0).abs() < 1e-4 {
                return self.atlas_rgba.clone();
            }
        }

        let mut out = RgbaImage::from_pixel(w, h, Rgba([0, 0, 0, 0]));
        let active_idx = self
            .metadata
            .active_layer
            .min(self.metadata.layers.len().saturating_sub(1));

        for (idx, layer) in self.metadata.layers.iter().enumerate().rev() {
            if !layer.visible || layer.opacity <= 0.0 {
                continue;
            }
            let src_img: &RgbaImage = if idx == active_idx {
                &self.atlas_rgba
            } else if let Some(ref img) = layer.layer_rgba {
                img
            } else {
                continue;
            };
            let opacity = layer.opacity.clamp(0.0, 1.0);
            let cw = w.min(src_img.width());
            let ch = h.min(src_img.height());
            for y in 0..ch {
                for x in 0..cw {
                    let sp = src_img.get_pixel(x, y);
                    let sa = (sp[3] as f32 / 255.0) * opacity;
                    if sa <= 0.0 {
                        continue;
                    }
                    let dp = out.get_pixel(x, y);
                    if dp[3] == 0 && (opacity - 1.0).abs() < 1e-4 {
                        out.put_pixel(x, y, *sp);
                        continue;
                    }
                    let da = dp[3] as f32 / 255.0;
                    let out_a = sa + da * (1.0 - sa);
                    if out_a > 0.0 {
                        let r = ((sp[0] as f32 * sa + dp[0] as f32 * da * (1.0 - sa)) / out_a)
                            .round()
                            .clamp(0.0, 255.0) as u8;
                        let g = ((sp[1] as f32 * sa + dp[1] as f32 * da * (1.0 - sa)) / out_a)
                            .round()
                            .clamp(0.0, 255.0) as u8;
                        let b = ((sp[2] as f32 * sa + dp[2] as f32 * da * (1.0 - sa)) / out_a)
                            .round()
                            .clamp(0.0, 255.0) as u8;
                        let a = (out_a * 255.0).round().clamp(0.0, 255.0) as u8;
                        out.put_pixel(x, y, Rgba([r, g, b, a]));
                    }
                }
            }
        }
        out
    }

    pub fn set_sketch_image(
        &mut self,
        img: Option<RgbaImage>,
        relative_path: Option<String>,
        embedded_base64: Option<String>,
    ) {
        self.sketch_rgba = img;
        self.metadata.sketch.relative_path = relative_path;
        self.metadata.sketch.embedded_base64 = embedded_base64;
        self.metadata.sketch.visible = self.sketch_rgba.is_some();
        self.sketch_dirty = true;
    }

    pub fn to_color_image(&self) -> ColorImage {
        let composited = self.composited_atlas_rgba();
        let size = [composited.width() as usize, composited.height() as usize];
        ColorImage::from_rgba_unmultiplied(size, composited.as_raw())
    }

    pub fn sketch_to_color_image(&self) -> Option<ColorImage> {
        let img = self.sketch_rgba.as_ref()?;
        let size = [img.width() as usize, img.height() as usize];
        Some(ColorImage::from_rgba_unmultiplied(size, img.as_raw()))
    }

    /// Rasterizes the positioned high-resolution sketch directly into `frame_index`,
    /// automatically quantized to the active palette colors.
    pub fn bake_sketch_to_frame(
        &mut self,
        frame_index: usize,
        lock_alpha: bool,
        selection: &SelectionMask,
    ) -> bool {
        let sketch_img = match &self.sketch_rgba {
            Some(img) => img,
            None => return false,
        };

        let fw = self.metadata.frame_width.max(1);
        let fh = self.metadata.frame_height.max(1);
        let (ox, oy) = self.metadata.frame_origin(frame_index);
        let mut changed = false;

        for ly in 0..(fh as i32) {
            for lx in 0..(fw as i32) {
                if !selection.contains(lx, ly) {
                    continue;
                }
                let ax = ox + lx as u32;
                let ay = oy + ly as u32;
                if ax >= self.atlas_rgba.width() || ay >= self.atlas_rgba.height() {
                    continue;
                }
                if lock_alpha && self.atlas_rgba.get_pixel(ax, ay)[3] == 0 {
                    continue;
                }

                let fx = lx as f32 + 0.5;
                let fy = ly as f32 + 0.5;
                if let Some(sampled) =
                    self.metadata
                        .sketch
                        .sample_rgba_at_frame_pos(sketch_img, fw, fh, fx, fy)
                {
                    if sampled[3] >= 32 {
                        let quantized =
                            nearest_palette_color(sampled, &self.metadata.palette.colors);
                        self.atlas_rgba.put_pixel(ax, ay, Rgba(quantized));
                        changed = true;
                    }
                }
            }
        }

        if changed {
            self.canvas_dirty = true;
        }
        changed
    }

    pub fn extract_unique_colors(&self, max_colors: usize) -> Vec<[u8; 4]> {
        let mut seen = HashSet::new();
        let mut ordered = Vec::new();

        for pixel in self.atlas_rgba.pixels() {
            if pixel[3] > 0 {
                let rgba = [pixel[0], pixel[1], pixel[2], pixel[3]];
                if seen.insert(rgba) {
                    ordered.push(rgba);
                    if ordered.len() >= max_colors {
                        break;
                    }
                }
            }
        }
        ordered
    }

    /// Slices a given `atlas` buffer into individual frame `RgbaImage` buffers.
    pub fn extract_frames_from_atlas(&self, atlas: &RgbaImage) -> Vec<RgbaImage> {
        let fw = self.metadata.frame_width.max(1);
        let fh = self.metadata.frame_height.max(1);
        let total = self.metadata.frame_count.max(1) as usize;
        let mut frames = Vec::with_capacity(total);

        for idx in 0..total {
            let (ox, oy) = self.metadata.frame_origin(idx);
            let mut sub = RgbaImage::from_pixel(fw, fh, Rgba([0, 0, 0, 0]));
            for y in 0..fh {
                for x in 0..fw {
                    let ax = ox + x;
                    let ay = oy + y;
                    if ax < atlas.width() && ay < atlas.height() {
                        sub.put_pixel(x, y, *atlas.get_pixel(ax, ay));
                    }
                }
            }
            frames.push(sub);
        }
        frames
    }

    /// Packs `frames_img` into an atlas `RgbaImage` according to current `self.metadata`.
    pub fn pack_frames_into_atlas(&self, frames_img: &[RgbaImage]) -> RgbaImage {
        let fw = self.metadata.frame_width.max(1);
        let fh = self.metadata.frame_height.max(1);
        let (atlas_w, atlas_h) = self.metadata.atlas_dimensions();
        let mut new_atlas =
            RgbaImage::from_pixel(atlas_w.max(1), atlas_h.max(1), Rgba([0, 0, 0, 0]));

        for (idx, frame_buf) in frames_img.iter().enumerate() {
            let (ox, oy) = self.metadata.frame_origin(idx);
            let copy_w = fw.min(frame_buf.width());
            let copy_h = fh.min(frame_buf.height());
            for y in 0..copy_h {
                for x in 0..copy_w {
                    new_atlas.put_pixel(ox + x, oy + y, *frame_buf.get_pixel(x, y));
                }
            }
        }
        new_atlas
    }

    /// Slices the composited atlas into individual frame `RgbaImage` buffers.
    pub fn extract_all_frames(&self) -> Vec<RgbaImage> {
        let composited = self.composited_atlas_rgba();
        self.extract_frames_from_atlas(&composited)
    }

    /// Repackages `frames_img` into `self.atlas_rgba` according to `self.metadata.columns`.
    pub fn rebuild_atlas_from_frames(&mut self, frames_img: &[RgbaImage]) {
        let count = frames_img.len().max(1) as u32;
        let cols = self.metadata.columns.max(1);
        let rows = count.div_ceil(cols);

        self.metadata.frame_count = count;
        self.metadata.rows = rows.max(1);

        while self.metadata.frames.len() < count as usize {
            self.metadata.frames.push(FrameMeta::default());
        }
        self.metadata.frames.truncate(count as usize);

        self.atlas_rgba = self.pack_frames_into_atlas(frames_img);
        self.ensure_layers_initialized();
        self.sync_active_layer();

        let max_idx = (count as usize).saturating_sub(1);
        self.active_frame = self.active_frame.min(max_idx);
        for tag in &mut self.metadata.tags {
            tag.start_frame = tag.start_frame.min(max_idx);
            tag.end_frame = tag.end_frame.min(max_idx).max(tag.start_frame);
        }
        self.canvas_dirty = true;
    }

    /// Helper that transforms the frame list of every layer simultaneously.
    fn transform_all_layers_frames<F>(&mut self, mut transform: F)
    where
        F: FnMut(&mut Vec<RgbaImage>),
    {
        self.ensure_layers_initialized();
        self.sync_active_layer();
        let layer_count = self.metadata.layers.len();
        let mut all_layer_frames: Vec<Vec<RgbaImage>> = Vec::with_capacity(layer_count);
        for layer in &self.metadata.layers {
            let buf = layer.layer_rgba.as_ref().unwrap_or(&self.atlas_rgba);
            let mut frames = self.extract_frames_from_atlas(buf);
            transform(&mut frames);
            all_layer_frames.push(frames);
        }

        let active_idx = self
            .metadata
            .active_layer
            .min(all_layer_frames.len().saturating_sub(1));
        if let Some(active_frames) = all_layer_frames.get(active_idx) {
            self.rebuild_atlas_from_frames(active_frames);
        }
        for (idx, frames) in all_layer_frames.into_iter().enumerate() {
            let packed = self.pack_frames_into_atlas(&frames);
            if let Some(layer) = self.metadata.layers.get_mut(idx) {
                layer.layer_rgba = Some(packed);
            }
        }
        if let Some(active_img) = self
            .metadata
            .layers
            .get(active_idx)
            .and_then(|l| l.layer_rgba.clone())
        {
            self.atlas_rgba = active_img;
        }
    }

    /// Inserts a blank transparent frame immediately after `index`.
    pub fn insert_blank_frame_after(&mut self, index: usize) {
        let total = self.metadata.frame_count.max(1) as usize;
        let fw = self.metadata.frame_width.max(1);
        let fh = self.metadata.frame_height.max(1);
        let insert_pos = (index + 1).min(total);

        if insert_pos <= self.metadata.frames.len() {
            self.metadata.frames.insert(insert_pos, FrameMeta::default());
        }

        for (i, meta) in self.metadata.frames.iter_mut().enumerate() {
            if i != insert_pos {
                if let Some(target) = meta.linked_from {
                    if target >= insert_pos {
                        meta.linked_from = Some(target + 1);
                    }
                }
            }
        }

        for tag in &mut self.metadata.tags {
            if tag.start_frame >= insert_pos {
                tag.start_frame += 1;
            }
            if tag.end_frame >= index {
                tag.end_frame += 1;
            }
        }

        self.active_frame = insert_pos;
        self.transform_all_layers_frames(|frames_img| {
            let pos = insert_pos.min(frames_img.len());
            frames_img.insert(pos, RgbaImage::from_pixel(fw, fh, Rgba([0, 0, 0, 0])));
        });
    }

    /// Duplicates the frame at `index` and inserts the copy at `index + 1`.
    pub fn duplicate_frame_at(&mut self, index: usize) {
        let total = self.metadata.frame_count.max(1) as usize;
        if total == 0 {
            return;
        }
        let safe_idx = index.min(total - 1);
        let cloned_meta = self
            .metadata
            .frames
            .get(safe_idx)
            .cloned()
            .unwrap_or_default();

        let insert_pos = safe_idx + 1;
        if insert_pos <= self.metadata.frames.len() {
            self.metadata.frames.insert(insert_pos, cloned_meta);
        }

        for (i, meta) in self.metadata.frames.iter_mut().enumerate() {
            if i != insert_pos {
                if let Some(target) = meta.linked_from {
                    if target >= insert_pos {
                        meta.linked_from = Some(target + 1);
                    }
                }
            }
        }

        for tag in &mut self.metadata.tags {
            if tag.start_frame > safe_idx {
                tag.start_frame += 1;
            }
            if tag.end_frame >= safe_idx {
                tag.end_frame += 1;
            }
        }

        self.active_frame = insert_pos;
        self.transform_all_layers_frames(|frames_img| {
            if !frames_img.is_empty() {
                let s = safe_idx.min(frames_img.len() - 1);
                let cloned_img = frames_img[s].clone();
                frames_img.insert(s + 1, cloned_img);
            }
        });
    }

    /// Creates a linked copy of frame `index` inserted at `index + 1`.
    /// Changes in any frame in this linked group will synchronize across all linked frames.
    pub fn create_linked_frame_after(&mut self, index: usize) {
        let total = self.metadata.frame_count.max(1) as usize;
        if total == 0 {
            return;
        }
        let safe_idx = index.min(total - 1);
        let root = self.get_link_group_root(safe_idx);
        let insert_pos = safe_idx + 1;

        let new_meta = FrameMeta {
            hold_multiplier: 1,
            linked_from: Some(root),
        };
        if insert_pos <= self.metadata.frames.len() {
            self.metadata.frames.insert(insert_pos, new_meta);
        }

        for (i, meta) in self.metadata.frames.iter_mut().enumerate() {
            if i != insert_pos {
                if let Some(target) = meta.linked_from {
                    if target >= insert_pos {
                        meta.linked_from = Some(target + 1);
                    }
                }
            }
        }

        for tag in &mut self.metadata.tags {
            if tag.start_frame > safe_idx {
                tag.start_frame += 1;
            }
            if tag.end_frame >= safe_idx {
                tag.end_frame += 1;
            }
        }

        self.active_frame = insert_pos;
        self.transform_all_layers_frames(|frames_img| {
            if !frames_img.is_empty() {
                let s = safe_idx.min(frames_img.len() - 1);
                let cloned_img = frames_img[s].clone();
                frames_img.insert(s + 1, cloned_img);
            }
        });
    }

    /// Unlinks the frame at `index`, making its pixel content independent.
    pub fn unlink_frame(&mut self, index: usize) {
        if let Some(meta) = self.metadata.frames.get_mut(index) {
            meta.linked_from = None;
            self.has_unsaved_changes = true;
        }
    }

    /// Links `source_frame` to share pixel content with `target_frame`.
    pub fn link_frame_to(&mut self, source_frame: usize, target_frame: usize) {
        let total_frames = self.metadata.frame_count as usize;
        if source_frame >= total_frames || target_frame >= total_frames || source_frame == target_frame {
            return;
        }
        let root = self.get_link_group_root(target_frame);
        if let Some(meta) = self.metadata.frames.get_mut(source_frame) {
            meta.linked_from = Some(root);
        }
        self.sync_linked_frames(target_frame);
        self.canvas_dirty = true;
        self.has_unsaved_changes = true;
    }

    /// Returns true if `frame_idx` is linked to or shares artwork with any other frame.
    pub fn is_frame_linked(&self, frame_idx: usize) -> bool {
        if let Some(meta) = self.metadata.frames.get(frame_idx) {
            if meta.linked_from.is_some() {
                return true;
            }
        }
        self.metadata
            .frames
            .iter()
            .any(|m| m.linked_from == Some(frame_idx))
    }

    /// Returns the root frame index of the link group for `frame_idx`.
    pub fn get_link_group_root(&self, frame_idx: usize) -> usize {
        if let Some(meta) = self.metadata.frames.get(frame_idx) {
            if let Some(target) = meta.linked_from {
                if target < self.metadata.frame_count as usize && target != frame_idx {
                    return target;
                }
            }
        }
        frame_idx
    }

    /// Synchronizes pixel artwork from `source_frame` to all other frames linked in its group.
    pub fn sync_linked_frames(&mut self, source_frame: usize) -> bool {
        let total_frames = self.metadata.frame_count as usize;
        if source_frame >= total_frames || self.metadata.frames.len() < total_frames {
            return false;
        }
        if !self.is_frame_linked(source_frame) {
            return false;
        }
        let root = self.get_link_group_root(source_frame);
        let fw = self.metadata.frame_width.max(1);
        let fh = self.metadata.frame_height.max(1);
        let (src_ox, src_oy) = self.metadata.frame_origin(source_frame);

        let mut src_img = RgbaImage::from_pixel(fw, fh, Rgba([0, 0, 0, 0]));
        for y in 0..fh {
            for x in 0..fw {
                let ax = src_ox + x;
                let ay = src_oy + y;
                if ax < self.atlas_rgba.width() && ay < self.atlas_rgba.height() {
                    src_img.put_pixel(x, y, *self.atlas_rgba.get_pixel(ax, ay));
                }
            }
        }

        let mut changed = false;
        for idx in 0..total_frames {
            if idx == source_frame {
                continue;
            }
            let idx_root = self.get_link_group_root(idx);
            if idx_root == root || (idx == root && self.get_link_group_root(source_frame) == root) {
                let (dst_ox, dst_oy) = self.metadata.frame_origin(idx);
                for y in 0..fh {
                    for x in 0..fw {
                        let dax = dst_ox + x;
                        let day = dst_oy + y;
                        if dax < self.atlas_rgba.width() && day < self.atlas_rgba.height() {
                            let sp = *src_img.get_pixel(x, y);
                            if *self.atlas_rgba.get_pixel(dax, day) != sp {
                                self.atlas_rgba.put_pixel(dax, day, sp);
                                changed = true;
                            }
                        }
                    }
                }
            }
        }
        changed
    }

    /// Deletes the frame at `index` if there are at least 2 frames.
    pub fn delete_frame_at(&mut self, index: usize) -> bool {
        if self.metadata.frame_count <= 1 {
            return false;
        }
        let total = self.metadata.frame_count as usize;
        if index >= total {
            return false;
        }
        if index < self.metadata.frames.len() {
            self.metadata.frames.remove(index);
        }

        for meta in &mut self.metadata.frames {
            if let Some(target) = meta.linked_from {
                if target == index {
                    meta.linked_from = None;
                } else if target > index {
                    meta.linked_from = Some(target - 1);
                }
            }
        }

        for tag in &mut self.metadata.tags {
            if tag.start_frame > index {
                tag.start_frame = tag.start_frame.saturating_sub(1);
            }
            if tag.end_frame >= index && tag.end_frame > 0 {
                tag.end_frame = tag.end_frame.saturating_sub(1);
            }
        }

        self.active_frame = index.min(total.saturating_sub(2));
        self.transform_all_layers_frames(|frames_img| {
            if index < frames_img.len() && frames_img.len() > 1 {
                frames_img.remove(index);
            }
        });
        true
    }

    /// Swaps two frames `idx_a` and `idx_b` in both the pixel atlas and frame metadata list.
    pub fn swap_frames(&mut self, idx_a: usize, idx_b: usize) -> bool {
        let total = self.metadata.frame_count as usize;
        if idx_a >= total || idx_b >= total || idx_a == idx_b {
            return false;
        }
        if idx_a < self.metadata.frames.len() && idx_b < self.metadata.frames.len() {
            self.metadata.frames.swap(idx_a, idx_b);
        }
        self.active_frame = idx_b;
        self.transform_all_layers_frames(|frames_img| {
            if idx_a < frames_img.len() && idx_b < frames_img.len() {
                frames_img.swap(idx_a, idx_b);
            }
        });
        true
    }

    /// Dynamically changes the column/row layout of the atlas while preserving frame order.
    pub fn regrid_atlas(&mut self, new_columns: u32) {
        self.ensure_layers_initialized();
        self.sync_active_layer();
        let all_layer_frames: Vec<Vec<RgbaImage>> = self
            .metadata
            .layers
            .iter()
            .map(|l| {
                let buf = l.layer_rgba.as_ref().unwrap_or(&self.atlas_rgba);
                self.extract_frames_from_atlas(buf)
            })
            .collect();

        self.metadata.columns = new_columns.clamp(1, self.metadata.frame_count.max(1));
        let active_idx = self
            .metadata
            .active_layer
            .min(all_layer_frames.len().saturating_sub(1));
        if let Some(active_frames) = all_layer_frames.get(active_idx) {
            self.rebuild_atlas_from_frames(active_frames);
        }
        for (idx, frames) in all_layer_frames.into_iter().enumerate() {
            let packed = self.pack_frames_into_atlas(&frames);
            if let Some(layer) = self.metadata.layers.get_mut(idx) {
                layer.layer_rgba = Some(packed);
            }
        }
        if let Some(active_img) = self
            .metadata
            .layers
            .get(active_idx)
            .and_then(|l| l.layer_rgba.clone())
        {
            self.atlas_rgba = active_img;
        }
    }

    /// Resizes all frames to `(new_w, new_h)` aligned to a 3×3 anchor `(anchor_col: 0..=2, anchor_row: 0..=2)`.
    pub fn resize_frames_9grid(&mut self, new_w: u32, new_h: u32, anchor_col: u8, anchor_row: u8) {
        let nw = new_w.clamp(1, 2048);
        let nh = new_h.clamp(1, 2048);
        let ow = self.metadata.frame_width.max(1);
        let oh = self.metadata.frame_height.max(1);
        if nw == ow && nh == oh {
            return;
        }

        let offset_x = match anchor_col {
            0 => 0,
            1 => (nw as i32 - ow as i32) / 2,
            _ => nw as i32 - ow as i32,
        };
        let offset_y = match anchor_row {
            0 => 0,
            1 => (nh as i32 - oh as i32) / 2,
            _ => nh as i32 - oh as i32,
        };

        self.ensure_layers_initialized();
        self.sync_active_layer();
        let all_layer_frames: Vec<Vec<RgbaImage>> = self
            .metadata
            .layers
            .iter()
            .map(|l| {
                let buf = l.layer_rgba.as_ref().unwrap_or(&self.atlas_rgba);
                let old_frames = self.extract_frames_from_atlas(buf);
                old_frames
                    .into_iter()
                    .map(|old_buf| {
                        let mut resized = RgbaImage::from_pixel(nw, nh, Rgba([0, 0, 0, 0]));
                        for y in 0..(oh as i32) {
                            for x in 0..(ow as i32) {
                                let dx = x + offset_x;
                                let dy = y + offset_y;
                                if dx >= 0 && dy >= 0 && (dx as u32) < nw && (dy as u32) < nh {
                                    resized.put_pixel(
                                        dx as u32,
                                        dy as u32,
                                        *old_buf.get_pixel(x as u32, y as u32),
                                    );
                                }
                            }
                        }
                        resized
                    })
                    .collect()
            })
            .collect();

        self.metadata.frame_width = nw;
        self.metadata.frame_height = nh;
        self.metadata.pivot.origin_x += offset_x;
        self.metadata.pivot.origin_y += offset_y;
        for pt in &mut self.metadata.pivot.action_points {
            pt.x += offset_x;
            pt.y += offset_y;
        }

        let active_idx = self
            .metadata
            .active_layer
            .min(all_layer_frames.len().saturating_sub(1));
        if let Some(active_frames) = all_layer_frames.get(active_idx) {
            self.rebuild_atlas_from_frames(active_frames);
        }
        for (idx, frames) in all_layer_frames.into_iter().enumerate() {
            let packed = self.pack_frames_into_atlas(&frames);
            if let Some(layer) = self.metadata.layers.get_mut(idx) {
                layer.layer_rgba = Some(packed);
            }
        }
        if let Some(active_img) = self
            .metadata
            .layers
            .get(active_idx)
            .and_then(|l| l.layer_rgba.clone())
        {
            self.atlas_rgba = active_img;
        }
    }

    /// Automatically trims transparent borders around all frames, adjusting frame dimensions
    /// and shifting origin / action points accordingly. Returns `true` if any cropping took place.
    pub fn auto_crop_canvas(&mut self) -> bool {
        let old_frames = self.extract_all_frames();
        if old_frames.is_empty() {
            return false;
        }

        let ow = self.metadata.frame_width.max(1);
        let oh = self.metadata.frame_height.max(1);

        let mut min_x = u32::MAX;
        let mut min_y = u32::MAX;
        let mut max_x = 0u32;
        let mut max_y = 0u32;
        let mut has_any_pixels = false;

        for frame in &old_frames {
            for y in 0..oh {
                for x in 0..ow {
                    if frame.get_pixel(x, y)[3] > 0 {
                        has_any_pixels = true;
                        min_x = min_x.min(x);
                        min_y = min_y.min(y);
                        max_x = max_x.max(x);
                        max_y = max_y.max(y);
                    }
                }
            }
        }

        if !has_any_pixels {
            return false;
        }

        let nw = max_x - min_x + 1;
        let nh = max_y - min_y + 1;

        if min_x == 0 && min_y == 0 && nw == ow && nh == oh {
            return false;
        }

        self.ensure_layers_initialized();
        self.sync_active_layer();
        let all_layer_frames: Vec<Vec<RgbaImage>> = self
            .metadata
            .layers
            .iter()
            .map(|l| {
                let buf = l.layer_rgba.as_ref().unwrap_or(&self.atlas_rgba);
                let l_frames = self.extract_frames_from_atlas(buf);
                l_frames
                    .into_iter()
                    .map(|old_buf| {
                        let mut cropped = RgbaImage::from_pixel(nw, nh, Rgba([0, 0, 0, 0]));
                        for y in 0..nh {
                            for x in 0..nw {
                                let ox = min_x + x;
                                let oy = min_y + y;
                                if ox < old_buf.width() && oy < old_buf.height() {
                                    cropped.put_pixel(x, y, *old_buf.get_pixel(ox, oy));
                                }
                            }
                        }
                        cropped
                    })
                    .collect()
            })
            .collect();

        let offset_x = -(min_x as i32);
        let offset_y = -(min_y as i32);

        self.metadata.frame_width = nw;
        self.metadata.frame_height = nh;
        self.metadata.pivot.origin_x += offset_x;
        self.metadata.pivot.origin_y += offset_y;
        for pt in &mut self.metadata.pivot.action_points {
            pt.x += offset_x;
            pt.y += offset_y;
        }

        let active_idx = self
            .metadata
            .active_layer
            .min(all_layer_frames.len().saturating_sub(1));
        if let Some(active_frames) = all_layer_frames.get(active_idx) {
            self.rebuild_atlas_from_frames(active_frames);
        }
        for (idx, frames) in all_layer_frames.into_iter().enumerate() {
            let packed = self.pack_frames_into_atlas(&frames);
            if let Some(layer) = self.metadata.layers.get_mut(idx) {
                layer.layer_rgba = Some(packed);
            }
        }
        if let Some(active_img) = self
            .metadata
            .layers
            .get(active_idx)
            .and_then(|l| l.layer_rgba.clone())
        {
            self.atlas_rgba = active_img;
        }
        self.canvas_dirty = true;
        self.has_unsaved_changes = true;
        true
    }

    /// Slices an external sprite sheet `src` using custom frame dimensions, outer margin, and spacing,
    /// filtering by `selected_indices` if provided, and repackages the extracted frames into a clean `Project`.
    #[allow(clippy::too_many_arguments)]
    pub fn from_sliced_sheet_selected(
        project_name: String,
        src: &RgbaImage,
        frame_w: u32,
        frame_h: u32,
        margin_x: u32,
        margin_y: u32,
        spacing_x: u32,
        spacing_y: u32,
        target_columns: u32,
        selected_indices: Option<&[bool]>,
    ) -> Self {
        let fw = frame_w.max(1);
        let fh = frame_h.max(1);
        let step_x = fw.saturating_add(spacing_x).max(1);
        let step_y = fh.saturating_add(spacing_y).max(1);

        let mut frames = Vec::new();
        let mut y = margin_y;
        let mut idx = 0;
        while y.saturating_add(fh) <= src.height() {
            let mut x = margin_x;
            while x.saturating_add(fw) <= src.width() {
                let keep = match selected_indices {
                    Some(flags) => flags.get(idx).copied().unwrap_or(true),
                    None => true,
                };
                if keep {
                    let mut cell = RgbaImage::from_pixel(fw, fh, Rgba([0, 0, 0, 0]));
                    for ly in 0..fh {
                        for lx in 0..fw {
                            cell.put_pixel(lx, ly, *src.get_pixel(x + lx, y + ly));
                        }
                    }
                    frames.push(cell);
                }
                idx += 1;
                x = x.saturating_add(step_x);
            }
            y = y.saturating_add(step_y);
        }

        if frames.is_empty() {
            let mut fallback = RgbaImage::from_pixel(fw, fh, Rgba([0, 0, 0, 0]));
            let cw = fw.min(src.width());
            let ch = fh.min(src.height());
            for ly in 0..ch {
                for lx in 0..cw {
                    fallback.put_pixel(lx, ly, *src.get_pixel(lx, ly));
                }
            }
            frames.push(fallback);
        }

        let count = frames.len() as u32;
        let cols = target_columns.clamp(1, count.max(1));
        let meta = ProjectMetadata::new_config(project_name, fw, fh, count, cols, None);
        let mut proj = Self::new(meta);
        proj.has_unsaved_changes = true;
        proj.rebuild_atlas_from_frames(&frames);

        let extracted = proj.extract_unique_colors(64);
        if !extracted.is_empty() {
            proj.metadata.palette.colors = extracted.clone();
            if let Some(first_swap) = proj.metadata.palette.swaps.first_mut() {
                first_swap.colors = extracted;
            }
        }
        proj
    }

    /// Slices an external sprite sheet `src` using custom frame dimensions, outer margin, and spacing,
    /// and repackages the extracted frames into a clean `Project`.
    #[allow(dead_code, clippy::too_many_arguments)]
    pub fn from_sliced_sheet(
        project_name: String,
        src: &RgbaImage,
        frame_w: u32,
        frame_h: u32,
        margin_x: u32,
        margin_y: u32,
        spacing_x: u32,
        spacing_y: u32,
        target_columns: u32,
    ) -> Self {
        Self::from_sliced_sheet_selected(
            project_name,
            src,
            frame_w,
            frame_h,
            margin_x,
            margin_y,
            spacing_x,
            spacing_y,
            target_columns,
            None,
        )
    }

    /// Extracts a single frame as an isolated `RgbaImage`.
    pub fn export_single_frame_rgba(&self, frame_index: usize) -> RgbaImage {
        let composited = self.composited_atlas_rgba();
        let fw = self.metadata.frame_width.max(1);
        let fh = self.metadata.frame_height.max(1);
        let (ox, oy) = self.metadata.frame_origin(frame_index);
        let mut out = RgbaImage::from_pixel(fw, fh, Rgba([0, 0, 0, 0]));
        for y in 0..fh {
            for x in 0..fw {
                let ax = ox + x;
                let ay = oy + y;
                if ax < composited.width() && ay < composited.height() {
                    out.put_pixel(x, y, *composited.get_pixel(ax, ay));
                }
            }
        }
        out
    }

    /// Completely replaces the pixel buffer of `frame_index` with `incoming` (centered or top-left blitted)
    /// and appends any new opaque colors into the project palette.
    pub fn replace_single_frame_rgba(&mut self, frame_index: usize, incoming: &RgbaImage) -> bool {
        let total = self.metadata.frame_count.max(1) as usize;
        if frame_index >= total {
            return false;
        }
        let fw = self.metadata.frame_width.max(1);
        let fh = self.metadata.frame_height.max(1);
        let (ox, oy) = self.metadata.frame_origin(frame_index);

        for y in 0..fh {
            for x in 0..fw {
                let ax = ox + x;
                let ay = oy + y;
                if ax < self.atlas_rgba.width() && ay < self.atlas_rgba.height() {
                    let px = if x < incoming.width() && y < incoming.height() {
                        *incoming.get_pixel(x, y)
                    } else {
                        Rgba([0, 0, 0, 0])
                    };
                    self.atlas_rgba.put_pixel(ax, ay, px);
                    if px[3] > 0 {
                        let col = [px[0], px[1], px[2], 255];
                        if !self.metadata.palette.colors.contains(&col)
                            && self.metadata.palette.colors.len() < 256
                        {
                            self.metadata.palette.colors.push(col);
                        }
                    }
                }
            }
        }
        self.canvas_dirty = true;
        true
    }

    /// Builds a new animation `Project` from an ordered sequence of individual frame images.
    pub fn from_frame_sequence(project_name: String, frames: &[RgbaImage]) -> Option<Self> {
        if frames.is_empty() {
            return None;
        }
        let fw = frames.iter().map(|f| f.width()).max().unwrap_or(32).max(1);
        let fh = frames.iter().map(|f| f.height()).max().unwrap_or(32).max(1);
        let count = frames.len() as u32;
        let cols = count.clamp(1, 8);

        let mut padded_frames = Vec::with_capacity(frames.len());
        for src in frames {
            let mut cell = RgbaImage::from_pixel(fw, fh, Rgba([0, 0, 0, 0]));
            for y in 0..src.height().min(fh) {
                for x in 0..src.width().min(fw) {
                    cell.put_pixel(x, y, *src.get_pixel(x, y));
                }
            }
            padded_frames.push(cell);
        }

        let meta = ProjectMetadata::new_config(project_name, fw, fh, count, cols, None);
        let mut proj = Self::new(meta);
        proj.rebuild_atlas_from_frames(&padded_frames);

        let extracted = proj.extract_unique_colors(64);
        if !extracted.is_empty() {
            proj.metadata.palette.colors = extracted.clone();
            if let Some(first_swap) = proj.metadata.palette.swaps.first_mut() {
                first_swap.colors = extracted;
            }
        }
        Some(proj)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_duplicate_and_reorder() {
        let meta = ProjectMetadata::new_config("anim".to_string(), 8, 8, 2, 2, None);
        let mut proj = Project::new(meta);
        proj.atlas_rgba.put_pixel(0, 0, Rgba([255, 0, 0, 255]));

        proj.duplicate_frame_at(0);
        assert_eq!(proj.metadata.frame_count, 3);
        assert_eq!(proj.active_frame, 1);

        let frames = proj.extract_all_frames();
        assert_eq!(*frames[1].get_pixel(0, 0), Rgba([255, 0, 0, 255]));

        assert!(proj.swap_frames(1, 2));
        let frames_after = proj.extract_all_frames();
        assert_eq!(*frames_after[2].get_pixel(0, 0), Rgba([255, 0, 0, 255]));

        assert!(proj.delete_frame_at(0));
        assert_eq!(proj.metadata.frame_count, 2);
    }

    #[test]
    fn test_bake_sketch_to_frame() {
        let meta = ProjectMetadata::new_config("bake_test".to_string(), 8, 8, 1, 1, None);
        let mut proj = Project::new(meta);
        let sketch = RgbaImage::from_pixel(32, 32, Rgba([189, 55, 68, 255]));
        proj.set_sketch_image(Some(sketch), None, None);

        let sel = SelectionMask::default();
        assert!(proj.bake_sketch_to_frame(0, false, &sel));
        assert_eq!(*proj.atlas_rgba.get_pixel(4, 4), Rgba([189, 55, 68, 255]));
    }

    #[test]
    fn test_from_sliced_sheet_selected_subset() {
        let mut sheet = RgbaImage::from_pixel(32, 16, Rgba([0, 0, 0, 0]));
        // Paint pixel in left frame (frame 0)
        sheet.put_pixel(0, 0, Rgba([255, 0, 0, 255]));
        // Paint pixel in right frame (frame 1)
        sheet.put_pixel(16, 0, Rgba([0, 255, 0, 255]));

        let flags = vec![false, true]; // keep only frame 1
        let proj = Project::from_sliced_sheet_selected(
            "slice_test".to_string(),
            &sheet,
            16,
            16,
            0,
            0,
            0,
            0,
            1,
            Some(&flags),
        );

        assert_eq!(proj.metadata.frame_count, 1);
        assert_eq!(*proj.atlas_rgba.get_pixel(0, 0), Rgba([0, 255, 0, 255]));
        assert!(proj.has_unsaved_changes);
    }

    #[test]
    fn test_auto_crop_canvas() {
        let meta = ProjectMetadata::new_config("crop_test".to_string(), 16, 16, 1, 1, None);
        let mut proj = Project::new(meta);
        // Canvas is completely blank: auto_crop should return false
        assert!(!proj.auto_crop_canvas());

        // Paint a 4x4 block from (4, 4) to (7, 7)
        for y in 4..8 {
            for x in 4..8 {
                proj.atlas_rgba.put_pixel(x, y, Rgba([255, 128, 0, 255]));
            }
        }

        proj.metadata.pivot.origin_x = 8;
        proj.metadata.pivot.origin_y = 8;

        assert!(proj.auto_crop_canvas());
        // New frame size should be 4x4
        assert_eq!(proj.metadata.frame_width, 4);
        assert_eq!(proj.metadata.frame_height, 4);
        assert_eq!(proj.atlas_rgba.width(), 4);
        assert_eq!(proj.atlas_rgba.height(), 4);
        // Pixel at (0, 0) of new atlas should be the orange pixel
        assert_eq!(*proj.atlas_rgba.get_pixel(0, 0), Rgba([255, 128, 0, 255]));
        // Pivot was at 8, 8; offset was -4, so pivot should now be 4, 4
        assert_eq!(proj.metadata.pivot.origin_x, 4);
        assert_eq!(proj.metadata.pivot.origin_y, 4);

        // Cropping again should return false (already trimmed)
        assert!(!proj.auto_crop_canvas());
    }

    #[test]
    fn test_linked_frames_synchronization() {
        let meta = ProjectMetadata::new_config("test_links".to_string(), 8, 8, 2, 2, None);
        let mut proj = Project::new(meta);

        // Frame 0 has pixel (0, 0) red
        let (ox0, oy0) = proj.metadata.frame_origin(0);
        proj.atlas_rgba.put_pixel(ox0, oy0, Rgba([255, 0, 0, 255]));

        // Create linked frame after 0
        proj.create_linked_frame_after(0);
        assert_eq!(proj.metadata.frame_count, 3);
        assert!(proj.is_frame_linked(0));
        assert!(proj.is_frame_linked(1));

        // Frame 1 should have pixel (0, 0) red
        let (ox1, oy1) = proj.metadata.frame_origin(1);
        assert_eq!(*proj.atlas_rgba.get_pixel(ox1, oy1), Rgba([255, 0, 0, 255]));

        // Now draw on frame 1 (blue pixel at 1, 1) and sync
        proj.atlas_rgba.put_pixel(ox1 + 1, oy1 + 1, Rgba([0, 0, 255, 255]));
        proj.sync_linked_frames(1);

        // Frame 0 should now also have the blue pixel!
        assert_eq!(*proj.atlas_rgba.get_pixel(ox0 + 1, oy0 + 1), Rgba([0, 0, 255, 255]));

        // Unlink frame 1
        proj.unlink_frame(1);
        assert!(!proj.is_frame_linked(1));
    }

    #[test]
    fn test_multi_layer_compositing_and_merge() {
        let meta = ProjectMetadata::new_config("test_layers".to_string(), 8, 8, 1, 1, None);
        let mut proj = Project::new(meta);

        // Layer 0 (top track in timeline): paint blue at (0, 0)
        proj.atlas_rgba.put_pixel(0, 0, Rgba([0, 0, 255, 255]));
        proj.sync_active_layer();

        // Add Layer 1 (track below Layer 0 in timeline)
        let l1 = proj.add_layer("Layer 2".to_string());
        assert_eq!(l1, 1);
        assert_eq!(proj.metadata.layers.len(), 2);
        assert_eq!(proj.metadata.active_layer, 1);

        // Layer 1 (below Layer 0): paint red at (0, 0) and (1, 0)
        proj.atlas_rgba.put_pixel(0, 0, Rgba([255, 0, 0, 255]));
        proj.atlas_rgba.put_pixel(1, 0, Rgba([255, 0, 0, 255]));
        proj.sync_active_layer();

        // Since Layer 0 is higher in the timeline, it is composited on top of Layer 1.
        // Composited atlas should have blue at (0, 0) and red at (1, 0).
        let comp = proj.composited_atlas_rgba();
        assert_eq!(*comp.get_pixel(0, 0), Rgba([0, 0, 255, 255]));
        assert_eq!(*comp.get_pixel(1, 0), Rgba([255, 0, 0, 255]));

        // Hide Layer 0 -> composited should have red at both (0, 0) and (1, 0)
        proj.metadata.layers[0].visible = false;
        let comp_hidden = proj.composited_atlas_rgba();
        assert_eq!(*comp_hidden.get_pixel(0, 0), Rgba([255, 0, 0, 255]));

        // Restore visibility and merge Layer 0 down into Layer 1
        proj.metadata.layers[0].visible = true;
        assert!(proj.merge_layer_down(0));
        assert_eq!(proj.metadata.layers.len(), 1);
        assert_eq!(proj.metadata.active_layer, 0);
        assert_eq!(*proj.atlas_rgba.get_pixel(0, 0), Rgba([0, 0, 255, 255]));
        assert_eq!(*proj.atlas_rgba.get_pixel(1, 0), Rgba([255, 0, 0, 255]));
    }
}
