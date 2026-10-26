use crate::model::project::Project;
use image::{Rgba, RgbaImage};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackerSettings {
    pub padding: u32,
    pub trim: bool,
    pub power_of_two: bool,
    pub scale: u32,
}

impl Default for PackerSettings {
    fn default() -> Self {
        Self {
            padding: 1,
            trim: true,
            power_of_two: true,
            scale: 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackedRect {
    pub frame_index: usize,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub source_w: u32,
    pub source_h: u32,
    pub trim_offset_x: u32,
    pub trim_offset_y: u32,
}

#[derive(Debug, Clone)]
pub struct PackedAtlasResult {
    pub atlas_image: RgbaImage,
    pub width: u32,
    pub height: u32,
    pub rects: Vec<PackedRect>,
    pub efficiency: f32,
}

struct ItemToPack {
    frame_index: usize,
    image: RgbaImage,
    source_w: u32,
    source_h: u32,
    trim_x: u32,
    trim_y: u32,
    trimmed_w: u32,
    trimmed_h: u32,
    pack_w: u32,
    pack_h: u32,
}

struct BinPlacement {
    sheet_w: u32,
    sheet_h: u32,
    positions: Vec<(u32, u32)>,
}

#[derive(Clone, Copy)]
struct Shelf {
    y: u32,
    height: u32,
    current_x: u32,
}

/// Packs all frames of `project` into a compact texture atlas using 2D shelf bin packing.
pub fn pack_project_atlas(
    project: &Project,
    settings: &PackerSettings,
) -> Result<PackedAtlasResult, String> {
    let frame_count = project.metadata.frame_count.max(1) as usize;
    let fw = project.metadata.frame_width.max(1);
    let fh = project.metadata.frame_height.max(1);
    let pad = settings.padding;

    // 1. Extract and optionally trim frames
    let mut items: Vec<ItemToPack> = Vec::with_capacity(frame_count);

    for idx in 0..frame_count {
        let (ox, oy) = project.metadata.frame_origin(idx);
        let mut min_x = fw;
        let mut min_y = fh;
        let mut max_x = 0;
        let mut max_y = 0;
        let mut has_pixel = false;

        for y in 0..fh {
            for x in 0..fw {
                let ax = ox + x;
                let ay = oy + y;
                if ax < project.atlas_rgba.width() && ay < project.atlas_rgba.height() {
                    let p = project.atlas_rgba.get_pixel(ax, ay);
                    if p[3] > 0 {
                        has_pixel = true;
                        min_x = min_x.min(x);
                        min_y = min_y.min(y);
                        max_x = max_x.max(x);
                        max_y = max_y.max(y);
                    }
                }
            }
        }

        let (trim_x, trim_y, trim_w, trim_h) = if settings.trim && has_pixel {
            let tw = max_x - min_x + 1;
            let th = max_y - min_y + 1;
            (min_x, min_y, tw, th)
        } else if !has_pixel && settings.trim {
            (0, 0, 1, 1)
        } else {
            (0, 0, fw, fh)
        };

        let mut sub_img = RgbaImage::from_pixel(trim_w, trim_h, Rgba([0, 0, 0, 0]));
        for y in 0..trim_h {
            for x in 0..trim_w {
                let ax = ox + trim_x + x;
                let ay = oy + trim_y + y;
                if ax < project.atlas_rgba.width() && ay < project.atlas_rgba.height() {
                    sub_img.put_pixel(x, y, *project.atlas_rgba.get_pixel(ax, ay));
                }
            }
        }

        items.push(ItemToPack {
            frame_index: idx,
            image: sub_img,
            source_w: fw,
            source_h: fh,
            trim_x,
            trim_y,
            trimmed_w: trim_w,
            trimmed_h: trim_h,
            pack_w: trim_w + pad * 2,
            pack_h: trim_h + pad * 2,
        });
    }

    // Sort items by packed height descending (shelf packing heuristic)
    items.sort_by(|a, b| b.pack_h.cmp(&a.pack_h).then_with(|| b.pack_w.cmp(&a.pack_w)));

    // 2. Candidate bin dimensions search
    let total_area: u32 = items.iter().map(|i| i.pack_w * i.pack_h).sum();
    let min_dim = (total_area as f64).sqrt().ceil() as u32;

    let candidate_sizes: Vec<(u32, u32)> = if settings.power_of_two {
        vec![
            (32, 32), (64, 64), (128, 64), (128, 128),
            (256, 128), (256, 256), (512, 256), (512, 512),
            (1024, 512), (1024, 1024), (2048, 1024), (2048, 2048),
            (4096, 4096),
        ]
    } else {
        let base_w = min_dim.next_power_of_two().max(32);
        vec![
            (base_w, base_w),
            (base_w * 2, base_w),
            (base_w * 2, base_w * 2),
            (base_w * 4, base_w * 2),
            (base_w * 4, base_w * 4),
        ]
    };

    let mut placement: Option<BinPlacement> = None;

    for &(cand_w, cand_h) in &candidate_sizes {
        if cand_w * cand_h < total_area {
            continue;
        }

        let mut shelves: Vec<Shelf> = Vec::new();
        let mut item_positions: Vec<(u32, u32)> = Vec::with_capacity(items.len());
        let mut all_fit = true;
        let mut total_shelf_height = 0u32;

        for item in &items {
            if item.pack_w > cand_w || item.pack_h > cand_h {
                all_fit = false;
                break;
            }

            // Find best fitting existing shelf
            let mut best_shelf_idx = None;
            let mut min_height_waste = u32::MAX;

            for (s_idx, s) in shelves.iter().enumerate() {
                if s.current_x + item.pack_w <= cand_w && s.height >= item.pack_h {
                    let waste = s.height - item.pack_h;
                    if waste < min_height_waste {
                        min_height_waste = waste;
                        best_shelf_idx = Some(s_idx);
                    }
                }
            }

            if let Some(s_idx) = best_shelf_idx {
                let s = &mut shelves[s_idx];
                item_positions.push((s.current_x, s.y));
                s.current_x += item.pack_w;
            } else {
                // Create new shelf
                let new_y = total_shelf_height;
                if new_y + item.pack_h > cand_h {
                    all_fit = false;
                    break;
                }
                item_positions.push((0, new_y));
                shelves.push(Shelf {
                    y: new_y,
                    height: item.pack_h,
                    current_x: item.pack_w,
                });
                total_shelf_height += item.pack_h;
            }
        }

        if all_fit {
            placement = Some(BinPlacement {
                sheet_w: cand_w,
                sheet_h: cand_h,
                positions: item_positions,
            });
            break;
        }
    }

    let placed = placement
        .ok_or_else(|| String::from("Sprite sheet is too large to pack within max texture dimensions"))?;

    // 3. Render packed atlas image
    let (final_w, final_h) = if settings.power_of_two {
        (placed.sheet_w, placed.sheet_h)
    } else {
        let max_used_x = items.iter().zip(&placed.positions).map(|(i, &(px, _))| px + i.pack_w).max().unwrap_or(placed.sheet_w);
        let max_used_y = items.iter().zip(&placed.positions).map(|(i, &(_, py))| py + i.pack_h).max().unwrap_or(placed.sheet_h);
        (max_used_x.max(1), max_used_y.max(1))
    };

    let scale = settings.scale.clamp(1, 8);
    let mut packed_img = RgbaImage::from_pixel(final_w, final_h, Rgba([0, 0, 0, 0]));
    let mut rects = Vec::with_capacity(items.len());
    let mut used_pixel_area = 0u64;

    for (item, &(px, py)) in items.iter().zip(&placed.positions) {
        let dest_x = px + pad;
        let dest_y = py + pad;

        for y in 0..item.trimmed_h {
            for x in 0..item.trimmed_w {
                let p = *item.image.get_pixel(x, y);
                if p[3] > 0 {
                    let dpx = dest_x + x;
                    let dpy = dest_y + y;
                    if dpx < final_w && dpy < final_h {
                        packed_img.put_pixel(dpx, dpy, p);
                    }
                }
            }
        }

        used_pixel_area += (item.trimmed_w as u64) * (item.trimmed_h as u64);
        rects.push(PackedRect {
            frame_index: item.frame_index,
            x: dest_x * scale,
            y: dest_y * scale,
            width: item.trimmed_w * scale,
            height: item.trimmed_h * scale,
            source_w: item.source_w * scale,
            source_h: item.source_h * scale,
            trim_offset_x: item.trim_x * scale,
            trim_offset_y: item.trim_y * scale,
        });
    }

    // Sort rects back to frame_index order for predictable output
    rects.sort_by_key(|r| r.frame_index);

    let efficiency = ((used_pixel_area as f64) / ((final_w as f64) * (final_h as f64)) * 100.0) as f32;

    // Apply scale if requested
    let output_img = if scale > 1 {
        let sw = final_w * scale;
        let sh = final_h * scale;
        let mut scaled_img = RgbaImage::new(sw, sh);
        for y in 0..sh {
            let sy = y / scale;
            for x in 0..sw {
                let sx = x / scale;
                scaled_img.put_pixel(x, y, *packed_img.get_pixel(sx, sy));
            }
        }
        scaled_img
    } else {
        packed_img
    };

    Ok(PackedAtlasResult {
        atlas_image: output_img,
        width: final_w * scale,
        height: final_h * scale,
        rects,
        efficiency,
    })
}

/// Serializes packed atlas layout into standard sprite sheet JSON format.
pub fn encode_packed_atlas_json(
    result: &PackedAtlasResult,
    project: &Project,
    image_filename: &str,
) -> String {
    let fps = project.metadata.fps.max(1);
    let base_duration_ms = (1000.0 / fps as f64).round() as u64;

    let frames_json: Vec<String> = result
        .rects
        .iter()
        .map(|r| {
            let hold = project
                .metadata
                .frames
                .get(r.frame_index)
                .map(|m| m.hold_multiplier.max(1) as u64)
                .unwrap_or(1);
            let duration_ms = base_duration_ms * hold;

            format!(
                r#"    {{
      "filename": "{}_{}.png",
      "frame": {{ "x": {}, "y": {}, "w": {}, "h": {} }},
      "rotated": false,
      "trimmed": {},
      "spriteSourceSize": {{ "x": {}, "y": {}, "w": {}, "h": {} }},
      "sourceSize": {{ "w": {}, "h": {} }},
      "duration": {}
    }}"#,
                project.metadata.project_name,
                r.frame_index,
                r.x,
                r.y,
                r.width,
                r.height,
                r.width != r.source_w || r.height != r.source_h,
                r.trim_offset_x,
                r.trim_offset_y,
                r.width,
                r.height,
                r.source_w,
                r.source_h,
                duration_ms
            )
        })
        .collect();

    format!(
        r#"{{
  "frames": [
{}
  ],
  "meta": {{
    "app": "OniAnvil",
    "version": "{}",
    "image": "{}",
    "format": "RGBA8888",
    "size": {{ "w": {}, "h": {} }},
    "scale": "1"
  }}
}}"#,
        frames_json.join(",\n"),
        env!("CARGO_PKG_VERSION"),
        image_filename,
        result.width,
        result.height
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::metadata::ProjectMetadata;

    #[test]
    fn test_pack_project_atlas_trim_and_pot() {
        let meta = ProjectMetadata::new_config("hero".to_string(), 16, 16, 2, 2, None);
        let mut proj = Project::new(meta);

        // Frame 0 has a 4x4 box
        let (ox0, oy0) = proj.metadata.frame_origin(0);
        for y in 0..4 {
            for x in 0..4 {
                proj.atlas_rgba.put_pixel(ox0 + x, oy0 + y, Rgba([255, 0, 0, 255]));
            }
        }

        // Frame 1 has an 8x8 box
        let (ox1, oy1) = proj.metadata.frame_origin(1);
        for y in 0..8 {
            for x in 0..8 {
                proj.atlas_rgba.put_pixel(ox1 + x, oy1 + y, Rgba([0, 255, 0, 255]));
            }
        }

        let settings = PackerSettings {
            padding: 1,
            trim: true,
            power_of_two: true,
            scale: 1,
        };

        let result = pack_project_atlas(&proj, &settings).expect("Packing failed");
        assert!(result.width.is_power_of_two());
        assert!(result.height.is_power_of_two());
        assert_eq!(result.rects.len(), 2);
        assert_eq!(result.rects[0].width, 4);
        assert_eq!(result.rects[1].width, 8);

        let json = encode_packed_atlas_json(&result, &proj, "hero_sheet.png");
        assert!(json.contains("hero_0.png"));
        assert!(json.contains("hero_1.png"));
        assert!(json.contains("\"image\": \"hero_sheet.png\""));
    }
}
