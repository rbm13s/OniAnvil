use crate::io::storage::IoManager;
use crate::model::project::Project;
use image::{Rgba, RgbaImage};

/// Builds a Nearest-Neighbor scaled atlas `RgbaImage` with optional edge pixel extrusion (`extrude_px` around each frame).
pub fn build_scaled_and_extruded_atlas(
    project: &Project,
    scale_factor: u32,
    extrude_px: u32,
) -> RgbaImage {
    let scale = scale_factor.clamp(1, 16);
    let pad = extrude_px.min(8);

    let frames = project.extract_all_frames();
    let fw = project.metadata.frame_width.max(1);
    let fh = project.metadata.frame_height.max(1);
    let sfw = fw * scale;
    let sfh = fh * scale;

    let cell_w = sfw + pad * 2;
    let cell_h = sfh + pad * 2;
    let cols = project.metadata.columns.max(1);
    let rows = project.metadata.rows.max(1);

    let out_w = (cell_w * cols).max(1);
    let out_h = (cell_h * rows).max(1);
    let mut out_img = RgbaImage::from_pixel(out_w, out_h, Rgba([0, 0, 0, 0]));

    for (idx, frame_buf) in frames.iter().enumerate() {
        let col = (idx as u32) % cols;
        let row = (idx as u32) / cols;
        let cell_ox = col * cell_w;
        let cell_oy = row * cell_h;

        for cy in 0..cell_h {
            for cx in 0..cell_w {
                // Clamp into [0..sfw-1, 0..sfh-1] so padding duplicates edge pixels (extrusion)
                let clamped_sx = (cx as i32 - pad as i32).clamp(0, sfw as i32 - 1) as u32;
                let clamped_sy = (cy as i32 - pad as i32).clamp(0, sfh as i32 - 1) as u32;
                let src_x = (clamped_sx / scale).min(fw - 1);
                let src_y = (clamped_sy / scale).min(fh - 1);
                let pixel = *frame_buf.get_pixel(src_x, src_y);
                out_img.put_pixel(cell_ox + cx, cell_oy + cy, pixel);
            }
        }
    }

    out_img
}

pub fn encode_scaled_png_bytes(
    project: &Project,
    scale_factor: u32,
    extrude_px: u32,
) -> Result<Vec<u8>, String> {
    let img = build_scaled_and_extruded_atlas(project, scale_factor, extrude_px);
    IoManager::encode_png_bytes(&img)
}

/// Encodes an animated `.gif` respecting per-frame hold multipliers and the selected `[start_frame..=end_frame]` range.
#[allow(clippy::needless_range_loop)]
pub fn encode_animated_gif_bytes(
    project: &Project,
    scale_factor: u32,
    start_frame: usize,
    end_frame: usize,
) -> Result<Vec<u8>, String> {
    let scale = scale_factor.clamp(1, 16);
    let frames = project.extract_all_frames();
    if frames.is_empty() {
        return Err("Project has no frames".to_string());
    }

    let fw = project.metadata.frame_width.max(1);
    let fh = project.metadata.frame_height.max(1);
    let sfw = (fw * scale).min(4096) as u16;
    let sfh = (fh * scale).min(4096) as u16;

    let mut gif_bytes = Vec::new();
    {
        // Note: `gif` crate is a transitive dependency of `image` with the "gif" feature,
        // or we can use `image::codecs::gif::GifEncoder` directly!
        use image::codecs::gif::{GifEncoder, Repeat};
        use image::{Delay, Frame};

        let mut encoder = GifEncoder::new_with_speed(&mut gif_bytes, 10);
        encoder
            .set_repeat(Repeat::Infinite)
            .map_err(|e| e.to_string())?;

        let max_idx = frames.len() - 1;
        let s_idx = start_frame.min(max_idx);
        let e_idx = end_frame.min(max_idx).max(s_idx);
        let fps = project.metadata.fps.max(1);

        for idx in s_idx..=e_idx {
            let src = &frames[idx];
            let mut scaled = RgbaImage::from_pixel(sfw as u32, sfh as u32, Rgba([0, 0, 0, 0]));
            for sy in 0..(sfh as u32) {
                for sx in 0..(sfw as u32) {
                    let p = *src.get_pixel((sx / scale).min(fw - 1), (sy / scale).min(fh - 1));
                    scaled.put_pixel(sx, sy, p);
                }
            }

            let hold = project
                .metadata
                .frames
                .get(idx)
                .map(|f| f.hold_multiplier.max(1))
                .unwrap_or(1);
            let duration_ms = ((hold * 1000) / fps).max(20);
            let delay = Delay::from_numer_denom_ms(duration_ms, 1);
            let gif_frame = Frame::from_parts(scaled, 0, 0, delay);
            encoder.encode_frame(gif_frame).map_err(|e| e.to_string())?;
        }
    }

    Ok(gif_bytes)
}

/// Encodes standard sprite sheet JSON format for game engines (Godot, Defold, Raylib, Bevy, Phaser, etc.).
pub fn encode_sprite_sheet_json(project: &Project, image_filename: &str) -> String {
    let (atlas_w, atlas_h) = project.metadata.atlas_dimensions();
    let fw = project.metadata.frame_width;
    let fh = project.metadata.frame_height;
    let fps = project.metadata.fps.max(1);

    let mut frames_arr = Vec::new();
    for f_idx in 0..project.metadata.frame_count as usize {
        let (ox, oy) = project.metadata.frame_origin(f_idx);
        let hold = project
            .metadata
            .frames
            .get(f_idx)
            .map(|f| f.hold_multiplier.max(1))
            .unwrap_or(1);
        let duration_ms = ((hold * 1000) / fps).max(1);

        frames_arr.push(serde_json::json!({
            "filename": format!("{}_{}.png", project.metadata.project_name, f_idx),
            "frame": {
                "x": ox,
                "y": oy,
                "w": fw,
                "h": fh
            },
            "rotated": false,
            "trimmed": false,
            "spriteSourceSize": {
                "x": 0,
                "y": 0,
                "w": fw,
                "h": fh
            },
            "sourceSize": {
                "w": fw,
                "h": fh
            },
            "duration": duration_ms
        }));
    }

    let mut frame_tags = Vec::new();
    for tag in &project.metadata.tags {
        frame_tags.push(serde_json::json!({
            "name": tag.name,
            "from": tag.start_frame,
            "to": tag.end_frame,
            "direction": tag.direction.as_direction_str()
        }));
    }

    let out = serde_json::json!({
        "frames": frames_arr,
        "meta": {
            "app": "OniAnvil",
            "version": env!("CARGO_PKG_VERSION"),
            "image": image_filename,
            "format": "RGBA8888",
            "size": {
                "w": atlas_w,
                "h": atlas_h
            },
            "scale": "1",
            "frameTags": frame_tags
        }
    });

    serde_json::to_string_pretty(&out).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::metadata::ProjectMetadata;

    #[test]
    fn test_scaled_and_extruded_png_and_gif_export() {
        let meta = ProjectMetadata::new_config("exp".to_string(), 8, 8, 2, 2, None);
        let mut proj = Project::new(meta);
        proj.atlas_rgba.put_pixel(0, 0, Rgba([255, 100, 50, 255]));

        let scaled = build_scaled_and_extruded_atlas(&proj, 2, 1);
        // Each 8x8 frame scaled 2x is 16x16, plus 1px extrusion on each side = 18x18 per cell; 2x1 cells = 36x18
        assert_eq!(scaled.dimensions(), (36, 18));
        // Extruded top-left corner (0,0) must duplicate frame pixel (0,0)
        assert_eq!(*scaled.get_pixel(0, 0), Rgba([255, 100, 50, 255]));

        let gif_bytes = encode_animated_gif_bytes(&proj, 2, 0, 1).expect("encode gif");
        assert!(gif_bytes.starts_with(b"GIF"));
    }

    #[test]
    fn test_encode_sprite_sheet_json() {
        let mut meta = ProjectMetadata::new_config("hero".to_string(), 16, 16, 2, 2, None);
        meta.tags.push(crate::model::frame::FrameTag {
            name: "Walk".to_string(),
            start_frame: 0,
            end_frame: 1,
            color: [255, 0, 0, 255],
            direction: crate::model::frame::TagDirection::PingPong,
        });
        let proj = Project::new(meta);

        let json_str = encode_sprite_sheet_json(&proj, "hero.png");
        assert!(json_str.contains("\"image\": \"hero.png\""));
        assert!(json_str.contains("\"name\": \"Walk\""));
        assert!(json_str.contains("\"direction\": \"pingpong\""));
        assert!(json_str.contains("\"duration\":"));
        assert!(json_str.contains("\"frames\":"));
    }
}
