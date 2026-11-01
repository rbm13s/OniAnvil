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

/// Builds the standard Aseprite `meta.layers` and `meta.slices` arrays for `project`.
pub fn build_aseprite_meta_layers_and_slices(
    project: &Project,
    scale: u32,
) -> (Vec<serde_json::Value>, Vec<serde_json::Value>) {
    let s = scale.max(1) as i32;
    let frame_count = project.metadata.frame_count.max(1) as usize;

    let mut layers_arr = Vec::new();
    for layer in &project.metadata.layers {
        let opacity_u8 = (layer.opacity.clamp(0.0, 1.0) * 255.0).round() as u8;
        layers_arr.push(serde_json::json!({
            "name": layer.name,
            "opacity": opacity_u8,
            "blendMode": "normal"
        }));
    }

    let mut slices_arr = Vec::new();

    // 1. Main sprite origin slice
    let ox = project.metadata.pivot.origin_x * s;
    let oy = project.metadata.pivot.origin_y * s;
    let mut origin_keys = Vec::with_capacity(frame_count);
    for f_idx in 0..frame_count {
        origin_keys.push(serde_json::json!({
            "frame": f_idx,
            "bounds": { "x": ox, "y": oy, "w": s, "h": s },
            "pivot": { "x": 0, "y": 0 }
        }));
    }
    slices_arr.push(serde_json::json!({
        "name": "origin",
        "color": "#f0be46ff",
        "keys": origin_keys
    }));

    // 2. Per-frame action point slices
    for (ap_idx, ap) in project.metadata.pivot.action_points.iter().enumerate() {
        let [r, g, b, a] = ap.color;
        let hex_color = format!("#{r:02x}{g:02x}{b:02x}{a:02x}");
        let mut keys = Vec::with_capacity(frame_count);
        for f_idx in 0..frame_count {
            let (ax, ay) = project
                .get_frame_action_point(f_idx, ap_idx)
                .unwrap_or((ap.x, ap.y));
            keys.push(serde_json::json!({
                "frame": f_idx,
                "bounds": { "x": ax * s, "y": ay * s, "w": s, "h": s },
                "pivot": { "x": 0, "y": 0 }
            }));
        }
        slices_arr.push(serde_json::json!({
            "name": ap.name,
            "color": hex_color,
            "keys": keys
        }));
    }

    (layers_arr, slices_arr)
}

/// Encodes standard sprite sheet JSON format (Aseprite JSON Array standard) for game engines
/// (Godot, Defold, Raylib, Bevy, Phaser, PixiJS, Unity, etc.).
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

        let mut action_points_obj = serde_json::Map::new();
        for (ap_idx, ap) in project.metadata.pivot.action_points.iter().enumerate() {
            let (ax, ay) = project
                .get_frame_action_point(f_idx, ap_idx)
                .unwrap_or((ap.x, ap.y));
            action_points_obj.insert(
                ap.name.clone(),
                serde_json::json!({
                    "x": ax,
                    "y": ay
                }),
            );
        }

        let norm_pivot_x = (project.metadata.pivot.origin_x as f64) / (fw.max(1) as f64);
        let norm_pivot_y = (project.metadata.pivot.origin_y as f64) / (fh.max(1) as f64);

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
            "pivot": {
                "x": norm_pivot_x,
                "y": norm_pivot_y
            },
            "origin": {
                "x": project.metadata.pivot.origin_x,
                "y": project.metadata.pivot.origin_y
            },
            "actionPoints": action_points_obj,
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

    let (layers, slices) = build_aseprite_meta_layers_and_slices(project, 1);

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
            "frameTags": frame_tags,
            "layers": layers,
            "slices": slices
        }
    });

    serde_json::to_string_pretty(&out).unwrap_or_default()
}

/// Encodes a native Godot 4 `SpriteFrames` resource (`.tres`) referencing `texture_res_path`.
/// Includes all animation tags (including Ping-Pong / Reverse expansion) or a fallback `"default"` animation,
/// per-frame hold durations, and per-frame action points metadata.
pub fn encode_godot_sprite_frames_tres(project: &Project, texture_res_path: &str) -> String {
    let fw = project.metadata.frame_width.max(1);
    let fh = project.metadata.frame_height.max(1);
    let fps = project.metadata.fps.max(1);
    let total_frames = project.metadata.frame_count.max(1) as usize;

    let mut out = String::new();
    let load_steps = total_frames + 2;
    out.push_str(&format!(
        "[gd_resource type=\"SpriteFrames\" load_steps={load_steps} format=3]\n\n"
    ));
    out.push_str(&format!(
        "[ext_resource type=\"Texture2D\" path=\"{texture_res_path}\" id=\"1_atlas\"]\n\n"
    ));

    for f_idx in 0..total_frames {
        let (ox, oy) = project.metadata.frame_origin(f_idx);
        out.push_str(&format!(
            "[sub_resource type=\"AtlasTexture\" id=\"AtlasTexture_f{f_idx}\"]\n"
        ));
        out.push_str("atlas = ExtResource(\"1_atlas\")\n");
        out.push_str(&format!("region = Rect2({ox}, {oy}, {fw}, {fh})\n"));
        if !project.metadata.pivot.action_points.is_empty() {
            let mut ap_entries = Vec::new();
            for (ap_idx, ap) in project.metadata.pivot.action_points.iter().enumerate() {
                let (ax, ay) = project
                    .get_frame_action_point(f_idx, ap_idx)
                    .unwrap_or((ap.x, ap.y));
                let escaped_name = ap.name.replace('"', "\\\"");
                ap_entries.push(format!("\"{escaped_name}\": Vector2i({ax}, {ay})"));
            }
            out.push_str(&format!(
                "metadata/action_points = {{ {} }}\n",
                ap_entries.join(", ")
            ));
        }
        out.push_str(&format!(
            "metadata/origin = Vector2i({}, {})\n\n",
            project.metadata.pivot.origin_x, project.metadata.pivot.origin_y
        ));
    }

    out.push_str("[resource]\n");
    out.push_str("animations = [");

    let build_anim_block = |name: &str, frame_indices: &[usize]| -> String {
        let mut frames_str = Vec::new();
        for &f_idx in frame_indices {
            let clamped = f_idx.min(total_frames.saturating_sub(1));
            let hold = project
                .metadata
                .frames
                .get(clamped)
                .map(|f| f.hold_multiplier.max(1))
                .unwrap_or(1);
            frames_str.push(format!(
                "{{\n\"duration\": {hold:.1},\n\"texture\": SubResource(\"AtlasTexture_f{clamped}\")\n}}"
            ));
        }
        let escaped = name.replace('"', "\\\"");
        format!(
            "{{\n\"frames\": [{}],\n\"loop\": true,\n\"name\": &\"{escaped}\",\n\"speed\": {fps:.1}\n}}",
            frames_str.join(", ")
        )
    };

    let mut anim_blocks = Vec::new();
    if project.metadata.tags.is_empty() {
        let seq: Vec<usize> = (0..total_frames).collect();
        anim_blocks.push(build_anim_block("default", &seq));
    } else {
        for tag in &project.metadata.tags {
            let start = tag.start_frame.min(total_frames.saturating_sub(1));
            let end = tag.end_frame.min(total_frames.saturating_sub(1)).max(start);
            let mut seq: Vec<usize> = match tag.direction {
                crate::model::frame::TagDirection::Forward => (start..=end).collect(),
                crate::model::frame::TagDirection::Reverse => (start..=end).rev().collect(),
                crate::model::frame::TagDirection::PingPong => {
                    let mut s: Vec<usize> = (start..=end).collect();
                    if end > start + 1 {
                        s.extend(((start + 1)..end).rev());
                    }
                    s
                }
            };
            if seq.is_empty() {
                seq.push(0);
            }
            anim_blocks.push(build_anim_block(&tag.name, &seq));
        }
    }

    out.push_str(&anim_blocks.join(", "));
    out.push_str("]\n");
    out
}

/// Writes an Aseprite String (`WORD` length + UTF-8 bytes) into `buf`.
fn write_ase_string(buf: &mut Vec<u8>, s: &str) {
    let bytes = s.as_bytes();
    let len = bytes.len().min(u16::MAX as usize) as u16;
    buf.extend_from_slice(&len.to_le_bytes());
    buf.extend_from_slice(&bytes[..len as usize]);
}

/// Appends an Aseprite chunk (`DWORD` size + `WORD` type + payload) into `frame_buf`.
fn write_ase_chunk(frame_buf: &mut Vec<u8>, chunk_type: u16, payload: &[u8]) {
    let chunk_size = (6 + payload.len()) as u32;
    frame_buf.extend_from_slice(&chunk_size.to_le_bytes());
    frame_buf.extend_from_slice(&chunk_type.to_le_bytes());
    frame_buf.extend_from_slice(payload);
}

/// Compresses `raw` bytes into a valid ZLIB stream (`CMF=0x78`, `FLG=0x01`, stored DEFLATE blocks, big-endian Adler-32).
/// Requires zero external crates and is 100% compliant with RFC 1950 / RFC 1951.
fn compress_zlib_stored(raw: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(raw.len() + (raw.len() / 65535 + 1) * 5 + 6);
    // ZLIB header: CMF = 0x78 (Deflate, 32K window), FLG = 0x01 (no dict, fastest check bits: (0x78*256+0x01) % 31 == 0)
    out.push(0x78);
    out.push(0x01);

    if raw.is_empty() {
        out.push(0x01); // BFINAL = 1, BTYPE = 00
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&(!0u16).to_le_bytes());
    } else {
        let chunks: Vec<&[u8]> = raw.chunks(65535).collect();
        for (i, chunk) in chunks.iter().enumerate() {
            let is_last = i + 1 == chunks.len();
            out.push(if is_last { 0x01 } else { 0x00 });
            let len = chunk.len() as u16;
            let nlen = !len;
            out.extend_from_slice(&len.to_le_bytes());
            out.extend_from_slice(&nlen.to_le_bytes());
            out.extend_from_slice(chunk);
        }
    }

    // Adler-32 checksum (big-endian)
    let mut a: u32 = 1;
    let mut b: u32 = 0;
    for &byte in raw {
        a = (a + byte as u32) % 65521;
        b = (b + a) % 65521;
    }
    let adler = (b << 16) | a;
    out.extend_from_slice(&adler.to_be_bytes());
    out
}

/// Encodes `project` directly into a binary Aseprite (`.aseprite` / `.ase`) file.
/// Includes all layers (ordered bottom-to-top per Aseprite spec), frames, frame hold durations,
/// palette (chunk `0x2019`), animation tags (chunk `0x2018`), and slices with per-frame keys (chunk `0x2022`).
pub fn encode_aseprite_bytes(project: &Project) -> Vec<u8> {
    let fw = project.metadata.frame_width.clamp(1, 65535) as u16;
    let fh = project.metadata.frame_height.clamp(1, 65535) as u16;
    let frame_count = project.metadata.frame_count.clamp(1, 65535) as u16;
    let fps = project.metadata.fps.max(1);

    // Extract per-layer frame images. In OniAnvil, layers[0] is the top-most layer in the UI,
    // whereas Aseprite stores layers from bottom (index 0) to top (index N-1).
    let layer_count = project.metadata.layers.len().max(1);
    let active_layer_idx = project
        .metadata
        .active_layer
        .min(layer_count.saturating_sub(1));

    let mut ase_layers: Vec<(String, bool, bool, u8, Vec<RgbaImage>)> =
        Vec::with_capacity(layer_count);
    if project.metadata.layers.is_empty() {
        ase_layers.push((
            "Layer 1".to_string(),
            true,
            false,
            255,
            project.extract_frames_from_atlas(&project.atlas_rgba),
        ));
    } else {
        for (idx, layer) in project.metadata.layers.iter().enumerate().rev() {
            let src_atlas = if idx == active_layer_idx {
                &project.atlas_rgba
            } else {
                layer.layer_rgba.as_ref().unwrap_or(&project.atlas_rgba)
            };
            let frames = project.extract_frames_from_atlas(src_atlas);
            let opacity_u8 = (layer.opacity.clamp(0.0, 1.0) * 255.0).round() as u8;
            ase_layers.push((
                layer.name.clone(),
                layer.visible,
                layer.locked,
                opacity_u8,
                frames,
            ));
        }
    }

    let mut all_frames_bytes = Vec::new();

    for f_idx in 0..(frame_count as usize) {
        let hold = project
            .metadata
            .frames
            .get(f_idx)
            .map(|f| f.hold_multiplier.max(1))
            .unwrap_or(1);
        let duration_ms = ((hold * 1000) / fps).clamp(1, 65535) as u16;

        let mut chunks_buf = Vec::new();
        let mut chunk_count: u32 = 0;

        if f_idx == 0 {
            // 1. Color Profile Chunk (0x2007) -> sRGB
            let mut cp = Vec::with_capacity(8);
            cp.extend_from_slice(&1u16.to_le_bytes()); // Type 1 = sRGB
            cp.extend_from_slice(&0u16.to_le_bytes()); // Flags = 0
            cp.extend_from_slice(&0u32.to_le_bytes()); // Fixed gamma = 0
            cp.extend_from_slice(&[0u8; 8]); // Reserved
            write_ase_chunk(&mut chunks_buf, 0x2007, &cp);
            chunk_count += 1;

            // 2. Palette Chunk (0x2019) + Old Palette Chunk (0x0004)
            let colors = &project.metadata.palette.colors;
            if !colors.is_empty() {
                let pal_len = colors.len().min(256);
                let mut pal = Vec::new();
                pal.extend_from_slice(&(pal_len as u32).to_le_bytes());
                pal.extend_from_slice(&0u32.to_le_bytes()); // first index
                pal.extend_from_slice(&((pal_len - 1) as u32).to_le_bytes()); // last index
                pal.extend_from_slice(&[0u8; 8]); // reserved
                for &[r, g, b, a] in &colors[..pal_len] {
                    pal.extend_from_slice(&0u16.to_le_bytes()); // flags = 0 (no name)
                    pal.push(r);
                    pal.push(g);
                    pal.push(b);
                    pal.push(a);
                }
                write_ase_chunk(&mut chunks_buf, 0x2019, &pal);
                chunk_count += 1;
            }

            // 3. Layer Chunks (0x2004) for each layer (bottom-to-top)
            for (name, visible, locked, opacity, _) in &ase_layers {
                let mut l_chunk = Vec::new();
                let mut flags: u16 = 0;
                if *visible {
                    flags |= 1; // Visible
                }
                if !*locked {
                    flags |= 2; // Editable
                } else {
                    flags |= 4; // Lock movement
                }
                l_chunk.extend_from_slice(&flags.to_le_bytes());
                l_chunk.extend_from_slice(&0u16.to_le_bytes()); // Layer type = 0 (Normal image layer)
                l_chunk.extend_from_slice(&0u16.to_le_bytes()); // Child level = 0
                l_chunk.extend_from_slice(&0u16.to_le_bytes()); // Default width (ignored)
                l_chunk.extend_from_slice(&0u16.to_le_bytes()); // Default height (ignored)
                l_chunk.extend_from_slice(&0u16.to_le_bytes()); // Blend mode = 0 (Normal)
                l_chunk.push(*opacity);
                l_chunk.extend_from_slice(&[0u8; 3]); // Reserved
                write_ase_string(&mut l_chunk, name);
                write_ase_chunk(&mut chunks_buf, 0x2004, &l_chunk);
                chunk_count += 1;
            }

            // 4. Frame Tags Chunk (0x2018)
            if !project.metadata.tags.is_empty() {
                let mut tags_chunk = Vec::new();
                let num_tags = project.metadata.tags.len().min(u16::MAX as usize) as u16;
                tags_chunk.extend_from_slice(&num_tags.to_le_bytes());
                tags_chunk.extend_from_slice(&[0u8; 8]); // Reserved
                for tag in project.metadata.tags.iter().take(num_tags as usize) {
                    let from = (tag.start_frame as u16).min(frame_count.saturating_sub(1));
                    let to = (tag.end_frame as u16)
                        .min(frame_count.saturating_sub(1))
                        .max(from);
                    let dir_byte: u8 = match tag.direction {
                        crate::model::frame::TagDirection::Forward => 0,
                        crate::model::frame::TagDirection::Reverse => 1,
                        crate::model::frame::TagDirection::PingPong => 2,
                    };
                    tags_chunk.extend_from_slice(&from.to_le_bytes());
                    tags_chunk.extend_from_slice(&to.to_le_bytes());
                    tags_chunk.push(dir_byte);
                    tags_chunk.extend_from_slice(&0u16.to_le_bytes()); // Repeat = 0 (infinite)
                    tags_chunk.extend_from_slice(&[0u8; 6]); // Reserved
                    tags_chunk.push(tag.color[0]);
                    tags_chunk.push(tag.color[1]);
                    tags_chunk.push(tag.color[2]);
                    tags_chunk.push(0); // Extra byte
                    write_ase_string(&mut tags_chunk, &tag.name);
                }
                write_ase_chunk(&mut chunks_buf, 0x2018, &tags_chunk);
                chunk_count += 1;
            }

            // 5. Slice Chunks (0x2022): Origin + Per-Frame Action Points
            {
                let mut origin_slice = Vec::new();
                origin_slice.extend_from_slice(&1u32.to_le_bytes()); // 1 key starting at frame 0
                origin_slice.extend_from_slice(&2u32.to_le_bytes()); // Flags = 2 (has pivot information)
                origin_slice.extend_from_slice(&0u32.to_le_bytes()); // Reserved
                write_ase_string(&mut origin_slice, "origin");
                // Key 0
                origin_slice.extend_from_slice(&0u32.to_le_bytes()); // Frame 0
                origin_slice.extend_from_slice(&project.metadata.pivot.origin_x.to_le_bytes());
                origin_slice.extend_from_slice(&project.metadata.pivot.origin_y.to_le_bytes());
                origin_slice.extend_from_slice(&1u32.to_le_bytes()); // Width = 1
                origin_slice.extend_from_slice(&1u32.to_le_bytes()); // Height = 1
                origin_slice.extend_from_slice(&0i32.to_le_bytes()); // Pivot X relative to slice
                origin_slice.extend_from_slice(&0i32.to_le_bytes()); // Pivot Y relative to slice
                write_ase_chunk(&mut chunks_buf, 0x2022, &origin_slice);
                chunk_count += 1;
            }

            for (ap_idx, ap) in project.metadata.pivot.action_points.iter().enumerate() {
                let mut slice_chunk = Vec::new();
                slice_chunk.extend_from_slice(&(frame_count as u32).to_le_bytes()); // Keys for every frame
                slice_chunk.extend_from_slice(&2u32.to_le_bytes()); // Flags = 2 (has pivot information)
                slice_chunk.extend_from_slice(&0u32.to_le_bytes()); // Reserved
                write_ase_string(&mut slice_chunk, &ap.name);

                for k_frame in 0..(frame_count as usize) {
                    let (ax, ay) = project
                        .get_frame_action_point(k_frame, ap_idx)
                        .unwrap_or((ap.x, ap.y));
                    slice_chunk.extend_from_slice(&(k_frame as u32).to_le_bytes());
                    slice_chunk.extend_from_slice(&ax.to_le_bytes());
                    slice_chunk.extend_from_slice(&ay.to_le_bytes());
                    slice_chunk.extend_from_slice(&1u32.to_le_bytes()); // Width = 1
                    slice_chunk.extend_from_slice(&1u32.to_le_bytes()); // Height = 1
                    slice_chunk.extend_from_slice(&0i32.to_le_bytes()); // Pivot X = 0
                    slice_chunk.extend_from_slice(&0i32.to_le_bytes()); // Pivot Y = 0
                }
                write_ase_chunk(&mut chunks_buf, 0x2022, &slice_chunk);
                chunk_count += 1;
            }
        }

        // 6. Cel Chunks (0x2005) for each layer on this frame
        for (l_idx, (_, _, _, _, frames_vec)) in ase_layers.iter().enumerate() {
            if let Some(frame_img) = frames_vec.get(f_idx) {
                let mut cel = Vec::new();
                cel.extend_from_slice(&(l_idx as u16).to_le_bytes()); // Layer index
                cel.extend_from_slice(&0i16.to_le_bytes()); // X position = 0
                cel.extend_from_slice(&0i16.to_le_bytes()); // Y position = 0
                cel.push(255); // Cel opacity = 255
                cel.extend_from_slice(&2u16.to_le_bytes()); // Cel type = 2 (Compressed Image)
                cel.extend_from_slice(&0i16.to_le_bytes()); // Z-index = 0
                cel.extend_from_slice(&[0u8; 5]); // Reserved
                cel.extend_from_slice(&fw.to_le_bytes()); // Width in pixels
                cel.extend_from_slice(&fh.to_le_bytes()); // Height in pixels
                let z_data = compress_zlib_stored(frame_img.as_raw());
                cel.extend_from_slice(&z_data);
                write_ase_chunk(&mut chunks_buf, 0x2005, &cel);
                chunk_count += 1;
            }
        }

        // Write 16-byte Frame Header + Chunks
        let frame_bytes_len = (16 + chunks_buf.len()) as u32;
        all_frames_bytes.extend_from_slice(&frame_bytes_len.to_le_bytes());
        all_frames_bytes.extend_from_slice(&0xF1FAu16.to_le_bytes()); // Magic number
        let old_chunks = chunk_count.min(0xFFFE) as u16;
        all_frames_bytes.extend_from_slice(&old_chunks.to_le_bytes());
        all_frames_bytes.extend_from_slice(&duration_ms.to_le_bytes());
        all_frames_bytes.extend_from_slice(&[0u8; 2]); // Reserved
        all_frames_bytes.extend_from_slice(&chunk_count.to_le_bytes());
        all_frames_bytes.extend_from_slice(&chunks_buf);
    }

    // Write 128-byte Aseprite File Header + All Frames
    let total_file_size = (128 + all_frames_bytes.len()) as u32;
    let mut file_bytes = Vec::with_capacity(total_file_size as usize);
    file_bytes.extend_from_slice(&total_file_size.to_le_bytes()); // 0..4: File size
    file_bytes.extend_from_slice(&0xA5E0u16.to_le_bytes()); // 4..6: Magic number (0xA5E0)
    file_bytes.extend_from_slice(&frame_count.to_le_bytes()); // 6..8: Number of frames
    file_bytes.extend_from_slice(&fw.to_le_bytes()); // 8..10: Width in pixels
    file_bytes.extend_from_slice(&fh.to_le_bytes()); // 10..12: Height in pixels
    file_bytes.extend_from_slice(&32u16.to_le_bytes()); // 12..14: Color depth = 32 bpp (RGBA)
    file_bytes.extend_from_slice(&1u32.to_le_bytes()); // 14..18: Flags = 1 (Layer opacity has valid value)
    let default_speed = (1000 / fps).clamp(1, 65535) as u16;
    file_bytes.extend_from_slice(&default_speed.to_le_bytes()); // 18..20: Speed (deprecated, ms)
    file_bytes.extend_from_slice(&0u32.to_le_bytes()); // 20..24: Set be 0
    file_bytes.extend_from_slice(&0u32.to_le_bytes()); // 24..28: Set be 0
    file_bytes.push(0); // 28: Transparent palette index
    file_bytes.extend_from_slice(&[0u8; 3]); // 29..32: Ignore
    let num_colors = project.metadata.palette.colors.len().clamp(1, 256) as u16;
    file_bytes.extend_from_slice(&num_colors.to_le_bytes()); // 32..34: Number of colors
    file_bytes.push(1); // 34: Pixel width = 1
    file_bytes.push(1); // 35: Pixel height = 1
    file_bytes.extend_from_slice(&0i16.to_le_bytes()); // 36..38: Grid X
    file_bytes.extend_from_slice(&0i16.to_le_bytes()); // 38..40: Grid Y
    file_bytes.extend_from_slice(&fw.to_le_bytes()); // 40..42: Grid width
    file_bytes.extend_from_slice(&fh.to_le_bytes()); // 42..44: Grid height
    file_bytes.extend_from_slice(&[0u8; 84]); // 44..128: For future (zeroes)
    debug_assert_eq!(file_bytes.len(), 128);

    file_bytes.extend_from_slice(&all_frames_bytes);
    file_bytes
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
        let mut proj = Project::new(meta);
        let ap_idx = proj.add_action_point("muzzle", 4, 6, [0, 255, 255, 255]);
        proj.set_frame_action_point(1, ap_idx, 10, 7);

        let json_str = encode_sprite_sheet_json(&proj, "hero.png");
        assert!(json_str.contains("\"image\": \"hero.png\""));
        assert!(json_str.contains("\"name\": \"Walk\""));
        assert!(json_str.contains("\"direction\": \"pingpong\""));
        assert!(json_str.contains("\"duration\":"));
        assert!(json_str.contains("\"frames\":"));
        assert!(json_str.contains("\"actionPoints\":"));
        assert!(json_str.contains("\"muzzle\":"));

        let parsed: serde_json::Value = serde_json::from_str(&json_str).expect("valid json");
        // Verify normalized pivot (0.0 .. 1.0) and pixel origin
        assert_eq!(parsed["frames"][0]["pivot"]["x"], 0.5);
        assert_eq!(parsed["frames"][0]["pivot"]["y"], 1.0);
        assert_eq!(parsed["frames"][0]["origin"]["x"], 8);
        assert_eq!(parsed["frames"][0]["origin"]["y"], 16);

        let f0_muzzle = &parsed["frames"][0]["actionPoints"]["muzzle"];
        let f1_muzzle = &parsed["frames"][1]["actionPoints"]["muzzle"];
        assert_eq!(f0_muzzle["x"], 4);
        assert_eq!(f0_muzzle["y"], 6);
        assert_eq!(f1_muzzle["x"], 10);
        assert_eq!(f1_muzzle["y"], 7);

        // Verify Aseprite-standard meta.layers and meta.slices
        assert_eq!(parsed["meta"]["layers"][0]["name"], "Layer 1");
        assert_eq!(parsed["meta"]["layers"][0]["opacity"], 255);
        assert_eq!(parsed["meta"]["layers"][0]["blendMode"], "normal");

        let slices = parsed["meta"]["slices"].as_array().expect("slices array");
        assert_eq!(slices.len(), 2); // "origin" + "muzzle"
        assert_eq!(slices[0]["name"], "origin");
        assert_eq!(slices[1]["name"], "muzzle");
        assert_eq!(slices[1]["color"], "#00ffffff");
        assert_eq!(slices[1]["keys"][0]["frame"], 0);
        assert_eq!(slices[1]["keys"][0]["bounds"]["x"], 4);
        assert_eq!(slices[1]["keys"][0]["bounds"]["y"], 6);
        assert_eq!(slices[1]["keys"][1]["frame"], 1);
        assert_eq!(slices[1]["keys"][1]["bounds"]["x"], 10);
        assert_eq!(slices[1]["keys"][1]["bounds"]["y"], 7);
    }

    #[test]
    fn test_encode_godot_sprite_frames_tres() {
        let mut meta = ProjectMetadata::new_config("hero".to_string(), 16, 16, 3, 3, None);
        meta.tags = vec![crate::model::frame::FrameTag {
            name: "Run".to_string(),
            start_frame: 0,
            end_frame: 2,
            color: [255, 0, 0, 255],
            direction: crate::model::frame::TagDirection::PingPong,
        }];
        let mut proj = Project::new(meta);
        let ap_idx = proj.add_action_point("hand", 5, 6, [0, 220, 255, 255]);
        proj.set_frame_action_point(1, ap_idx, 9, 8);

        let tres = encode_godot_sprite_frames_tres(&proj, "res://hero.png");
        assert!(tres.starts_with("[gd_resource type=\"SpriteFrames\" load_steps=5 format=3]"));
        assert!(tres.contains("[ext_resource type=\"Texture2D\" path=\"res://hero.png\" id=\"1_atlas\"]"));
        assert!(tres.contains("region = Rect2(16, 0, 16, 16)"));
        assert!(tres.contains("\"hand\": Vector2i(9, 8)"));
        assert!(tres.contains("\"name\": &\"Run\""));
        // Ping-pong of 0..=2 expands to 0, 1, 2, 1 (4 frame references)
        assert_eq!(tres.matches("SubResource(\"AtlasTexture_f1\")").count(), 2);
    }

    #[test]
    fn test_encode_aseprite_binary_format() {
        let meta = ProjectMetadata::new_config("knight".to_string(), 8, 8, 2, 2, None);
        let mut proj = Project::new(meta);
        proj.atlas_rgba.put_pixel(0, 0, Rgba([255, 64, 32, 255]));
        let ap_idx = proj.add_action_point("sword_tip", 7, 3, [255, 255, 0, 255]);
        proj.set_frame_action_point(1, ap_idx, 6, 2);

        let bytes = encode_aseprite_bytes(&proj);
        assert!(bytes.len() > 128);

        // Verify 128-byte header
        let total_size = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        assert_eq!(total_size as usize, bytes.len());
        let magic = u16::from_le_bytes([bytes[4], bytes[5]]);
        assert_eq!(magic, 0xA5E0);
        let frames = u16::from_le_bytes([bytes[6], bytes[7]]);
        assert_eq!(frames, 2);
        let width = u16::from_le_bytes([bytes[8], bytes[9]]);
        let height = u16::from_le_bytes([bytes[10], bytes[11]]);
        assert_eq!((width, height), (8, 8));
        let depth = u16::from_le_bytes([bytes[12], bytes[13]]);
        assert_eq!(depth, 32);

        // Verify Frame 0 header magic (0xF1FA) at offset 128
        let frame0_magic = u16::from_le_bytes([bytes[132], bytes[133]]);
        assert_eq!(frame0_magic, 0xF1FA);
    }
}
