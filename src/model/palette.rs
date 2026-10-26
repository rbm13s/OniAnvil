use image::{Rgba, RgbaImage};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaletteSwap {
    pub name: String,
    pub colors: Vec<[u8; 4]>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaletteData {
    pub colors: Vec<[u8; 4]>,
    pub swaps: Vec<PaletteSwap>,
    pub active_swap_index: usize,
    pub global_color_replace: bool,
}

impl Default for PaletteData {
    fn default() -> Self {
        let default_colors = vec![
            [20, 18, 29, 255],
            [48, 44, 66, 255],
            [87, 82, 107, 255],
            [140, 135, 162, 255],
            [203, 200, 218, 255],
            [245, 244, 250, 255],
            [189, 55, 68, 255],
            [230, 108, 76, 255],
            [245, 185, 86, 255],
            [140, 199, 88, 255],
            [62, 150, 108, 255],
            [52, 118, 168, 255],
            [88, 166, 224, 255],
            [132, 92, 179, 255],
            [196, 112, 174, 255],
            [148, 95, 71, 255],
        ];
        Self {
            colors: default_colors.clone(),
            swaps: vec![PaletteSwap {
                name: String::from("Default"),
                colors: default_colors,
            }],
            active_swap_index: 0,
            global_color_replace: true,
        }
    }
}

/// Parses either a plain-text `.hex` file or a JASC-PAL `.pal` file into RGBA color slots.
pub fn parse_palette_text(content: &str) -> Vec<[u8; 4]> {
    let mut colors = Vec::new();
    for raw_line in content.lines() {
        let line = raw_line.trim();
        if line.is_empty()
            || line.starts_with(';')
            || line.starts_with("//")
            || line.eq_ignore_ascii_case("JASC-PAL")
            || line == "0100"
        {
            continue;
        }

        // Check if line is space-separated R G B (JASC-PAL)
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            if let (Ok(r), Ok(g), Ok(b)) = (
                parts[0].parse::<u8>(),
                parts[1].parse::<u8>(),
                parts[2].parse::<u8>(),
            ) {
                colors.push([r, g, b, 255]);
                continue;
            }
        }

        // Otherwise try parsing hex string (#RRGGBB or RRGGBB or RRGGBBAA)
        let hex = line.trim_start_matches('#');
        if hex.len() == 6 {
            if let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&hex[0..2], 16),
                u8::from_str_radix(&hex[2..4], 16),
                u8::from_str_radix(&hex[4..6], 16),
            ) {
                colors.push([r, g, b, 255]);
            }
        } else if hex.len() == 8 {
            if let (Ok(r), Ok(g), Ok(b), Ok(a)) = (
                u8::from_str_radix(&hex[0..2], 16),
                u8::from_str_radix(&hex[2..4], 16),
                u8::from_str_radix(&hex[4..6], 16),
                u8::from_str_radix(&hex[6..8], 16),
            ) {
                colors.push([r, g, b, a]);
            }
        }
    }
    colors
}

/// Formats palette colors into a clean `.hex` string (one `RRGGBB` per line).
pub fn format_palette_hex(colors: &[[u8; 4]]) -> String {
    let mut out = String::new();
    for &[r, g, b, _] in colors {
        out.push_str(&format!("{:02x}{:02x}{:02x}\n", r, g, b));
    }
    out
}

/// Formats palette colors into standard GIMP `.gpl` format.
pub fn format_palette_gpl(palette_name: &str, colors: &[[u8; 4]]) -> String {
    let mut out = String::from("GIMP Palette\n");
    out.push_str(&format!("Name: {}\n", palette_name));
    out.push_str("Columns: 8\n#\n");
    for (i, &[r, g, b, _]) in colors.iter().enumerate() {
        out.push_str(&format!("{:3} {:3} {:3}\tColor {}\n", r, g, b, i + 1));
    }
    out
}

/// Extracts unique non-transparent colors from a `.png` palette image in order of appearance.
pub fn parse_palette_png(png_bytes: &[u8]) -> Result<Vec<[u8; 4]>, String> {
    let img = image::load_from_memory(png_bytes)
        .map_err(|e| format!("Failed to decode palette PNG: {}", e))?
        .to_rgba8();
    let mut colors = Vec::new();
    for p in img.pixels() {
        if p[3] > 0 {
            let rgba = [p[0], p[1], p[2], 255];
            if !colors.contains(&rgba) {
                colors.push(rgba);
                if colors.len() >= 256 {
                    break;
                }
            }
        }
    }
    if colors.is_empty() {
        return Err(String::from("No opaque colors found in PNG"));
    }
    Ok(colors)
}

/// Encodes palette colors into a `.png` swatch grid image (8 columns, 8x8 px per swatch).
pub fn encode_palette_png(colors: &[[u8; 4]]) -> Result<Vec<u8>, String> {
    let count = colors.len().max(1);
    let cols = count.min(8) as u32;
    let rows = (count as u32).div_ceil(cols).max(1);
    let swatch = 8u32;
    let mut img = RgbaImage::from_pixel(cols * swatch, rows * swatch, Rgba([0, 0, 0, 0]));

    for (idx, &col) in colors.iter().enumerate() {
        let c = (idx as u32) % cols;
        let r = (idx as u32) / cols;
        let ox = c * swatch;
        let oy = r * swatch;
        for dy in 0..swatch {
            for dx in 0..swatch {
                img.put_pixel(ox + dx, oy + dy, Rgba([col[0], col[1], col[2], 255]));
            }
        }
    }

    let mut buf = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut buf);
    img.write_to(&mut cursor, image::ImageFormat::Png)
        .map_err(|e| format!("Failed to encode palette PNG: {}", e))?;
    Ok(buf)
}

pub struct PredefinedPalette {
    pub name: &'static str,
    pub hex_colors: &'static [&'static str],
}

impl PredefinedPalette {
    pub fn to_rgba_vec(&self) -> Vec<[u8; 4]> {
        let mut out = Vec::with_capacity(self.hex_colors.len());
        for &hex_raw in self.hex_colors {
            let hex = hex_raw.trim_start_matches('#');
            if hex.len() == 6 {
                if let (Ok(r), Ok(g), Ok(b)) = (
                    u8::from_str_radix(&hex[0..2], 16),
                    u8::from_str_radix(&hex[2..4], 16),
                    u8::from_str_radix(&hex[4..6], 16),
                ) {
                    out.push([r, g, b, 255]);
                }
            }
        }
        out
    }

    pub fn to_palette_data(&self) -> PaletteData {
        let colors = self.to_rgba_vec();
        PaletteData {
            colors: colors.clone(),
            swaps: vec![PaletteSwap {
                name: String::from(self.name),
                colors,
            }],
            active_swap_index: 0,
            global_color_replace: true,
        }
    }
}

pub const PREDEFINED_PALETTES: &[PredefinedPalette] = &[
    PredefinedPalette {
        name: "Studio Default (16)",
        hex_colors: &[
            "#14121d", "#302c42", "#57526b", "#8c87a2", "#cbc8da", "#f5f4fa", "#bd3744",
            "#e66c4c", "#f5b956", "#8cc758", "#3e966c", "#3476a8", "#58a6e0", "#845cb3",
            "#c470ae", "#945f47",
        ],
    },
    PredefinedPalette {
        name: "DawnBringer (8)",
        hex_colors: &[
            "#000000", "#55415f", "#646964", "#d77355", "#508cd7", "#e6c86e", "#acf5a4",
            "#fbfcfc",
        ],
    },
    PredefinedPalette {
        name: "DawnBringer (16)",
        hex_colors: &[
            "#140c1c", "#442434", "#30346d", "#4e4a4e", "#854c30", "#346524", "#d04648",
            "#757161", "#597dce", "#d27d2c", "#8595a1", "#6daa2c", "#d2aa99", "#6caa96",
            "#d2c9a5", "#ebe4be",
        ],
    },
    PredefinedPalette {
        name: "DawnBringer (32)",
        hex_colors: &[
            "#000000", "#222034", "#45283c", "#663931", "#8f563b", "#df7126", "#d9a066",
            "#eec39a", "#fbf236", "#99e550", "#6abe30", "#37946e", "#4b692f", "#524b24",
            "#323c39", "#3f3f74", "#306082", "#5b6ee1", "#639bff", "#5fcde4", "#cbdbfc",
            "#ffffff", "#9badb7", "#847e87", "#696a6a", "#595652", "#76428a", "#ac3232",
            "#d95763", "#d77643", "#8f974a", "#8a6f30",
        ],
    },
    PredefinedPalette {
        name: "PICO-8 (16)",
        hex_colors: &[
            "#000000", "#1D2B53", "#7E2553", "#008751", "#AB5236", "#5F574F", "#C2C3C7",
            "#FFF1E8", "#FF004D", "#FFA300", "#FFEC27", "#00E436", "#29ADFF", "#83769C",
            "#FF77A8", "#FFCCAA",
        ],
    },
    PredefinedPalette {
        name: "TIC-80 (16)",
        hex_colors: &[
            "#1a1c2c", "#5d275d", "#b13e53", "#ef7d57", "#ffcd75", "#a7f070", "#38b764",
            "#257179", "#29366f", "#3b5dc9", "#41a6f6", "#73eff7", "#f4f4f4", "#94b0c2",
            "#566c86", "#333c57",
        ],
    },
    PredefinedPalette {
        name: "NES (55)",
        hex_colors: &[
            "#000000", "#fcfcfc", "#f8f8f8", "#bcbcbc", "#7c7c7c", "#a4e4fc", "#3cbcfc",
            "#0078f8", "#0000fc", "#b8b8f8", "#6888fc", "#0058f8", "#0000bc", "#d8b8f8",
            "#9878f8", "#6844fc", "#4428bc", "#f8b8f8", "#f878f8", "#d800cc", "#940084",
            "#f8a4c0", "#f85898", "#e40058", "#a80020", "#f0d0b0", "#f87858", "#f83800",
            "#a81000", "#fce0a8", "#fca044", "#e45c10", "#881400", "#f8d878", "#f8b800",
            "#ac7c00", "#503000", "#d8f878", "#b8f818", "#00b800", "#007800", "#b8f8b8",
            "#58d854", "#00a800", "#006800", "#b8f8d8", "#58f898", "#00a844", "#005800",
            "#00fcfc", "#00e8d8", "#008888", "#004058", "#f8d8f8", "#787878",
        ],
    },
    PredefinedPalette {
        name: "SNES (32)",
        hex_colors: &[
            "#000000", "#00021c", "#1c284d", "#343473", "#2d5280", "#4d7a99", "#7497a6",
            "#a3ccd9", "#f0edd8", "#732866", "#a6216e", "#d94c87", "#d9214f", "#f25565",
            "#f27961", "#993649", "#b36159", "#f09c60", "#b38f24", "#b3b324", "#f7c93e",
            "#17735f", "#119955", "#67b31b", "#1ba683", "#47cca9", "#96e3c9", "#2469b3",
            "#0b8be6", "#0bafe6", "#f28d85", "#f0bb90",
        ],
    },
    PredefinedPalette {
        name: "Game Boy (4)",
        hex_colors: &["#2e463d", "#385d49", "#577b46", "#7e8416"],
    },
    PredefinedPalette {
        name: "Game Boy Color (31)",
        hex_colors: &[
            "#f0eeef", "#f2d3c9", "#f1aeb1", "#ed6d8b", "#ee1764", "#942f55", "#8a7ba5",
            "#483e5a", "#fde5a7", "#ebac7d", "#a7888d", "#7a566d", "#34355a", "#a1efee",
            "#2276a4", "#fece7c", "#ed8631", "#ed0d0f", "#721d28", "#efef80", "#95de40",
            "#539a36", "#217319", "#b4561f", "#d0a46d", "#8d7320", "#5a541f", "#e1ebb0",
            "#b3bd85", "#5d7841", "#1b1020",
        ],
    },
    PredefinedPalette {
        name: "Virtual Boy (4)",
        hex_colors: &["#ef0000", "#a40000", "#550000", "#000000"],
    },
    PredefinedPalette {
        name: "Pokémon SGB (4)",
        hex_colors: &["#181010", "#84739c", "#f7b58c", "#ffefff"],
    },
    PredefinedPalette {
        name: "Mario Paint (15)",
        hex_colors: &[
            "#ff0000", "#ff8200", "#fffb00", "#00fb00", "#008242", "#00fbff", "#0000ff",
            "#c64121", "#846100", "#ffc384", "#c600c6", "#000000", "#848284", "#c6c3c6",
            "#fffbff",
        ],
    },
    PredefinedPalette {
        name: "Master System (64)",
        hex_colors: &[
            "#000000", "#550000", "#aa0000", "#ff0000", "#005500", "#555500", "#aa5500",
            "#ff5500", "#00aa00", "#55aa00", "#aaaa00", "#ffaa00", "#00ff00", "#55ff00",
            "#aaff00", "#ffff00", "#000055", "#550055", "#aa0055", "#ff0055", "#005555",
            "#555555", "#aa5555", "#ff5555", "#00aa55", "#55aa55", "#aaaa55", "#ffaa55",
            "#00ff55", "#55ff55", "#aaff55", "#ffff55", "#0000aa", "#5500aa", "#aa00aa",
            "#ff00aa", "#0055aa", "#5555aa", "#aa55aa", "#ff55aa", "#00aaaa", "#55aaaa",
            "#aaaaaa", "#ffaaaa", "#00ffaa", "#55ffaa", "#aaffaa", "#ffffaa", "#0000ff",
            "#5500ff", "#aa00ff", "#ff00ff", "#0055ff", "#5555ff", "#aa55ff", "#ff55ff",
            "#00aaff", "#55aaff", "#aaaaff", "#ffaaff", "#00ffff", "#55ffff", "#aaffff",
            "#ffffff",
        ],
    },
    PredefinedPalette {
        name: "Mega Drive / Genesis (34)",
        hex_colors: &[
            "#2b120d", "#9f1211", "#fc1400", "#fc6a00", "#fcfc00", "#000000", "#330445",
            "#9c0c9c", "#ff099d", "#00072c", "#040085", "#0000ff", "#0066ff", "#67cdfc",
            "#004849", "#067826", "#00c908", "#52ff00", "#c6f022", "#352c2e", "#754944",
            "#ad5950", "#f6757a", "#fac59b", "#693623", "#b0572b", "#e98e42", "#fcb448",
            "#fffcff", "#666688", "#9892ad", "#b8b8d1", "#ddd9e6", "#fcfcfc",
        ],
    },
    PredefinedPalette {
        name: "Atari 2600 (104)",
        hex_colors: &[
            "#000000", "#805800", "#445c00", "#703400", "#006414", "#700014", "#005c5c",
            "#70005c", "#003c70", "#580070", "#002070", "#3c0080", "#000088", "#404040",
            "#947020", "#5c7820", "#885020", "#208034", "#882034", "#207474", "#842074",
            "#1c5888", "#6c2088", "#1c3c88", "#542094", "#20209c", "#6c6c6c", "#a8843c",
            "#74903c", "#a0683c", "#3c9850", "#a03c50", "#3c8c8c", "#943c88", "#3874a0",
            "#803ca0", "#3858a0", "#6c3ca8", "#3c3cb0", "#909090", "#bc9c58", "#8cac58",
            "#b48458", "#58b06c", "#b4586c", "#58a4a4", "#a8589c", "#508cb4", "#9458b4",
            "#5074b4", "#8058bc", "#5858c0", "#b0b0b0", "#ccac70", "#a0c070", "#c89870",
            "#70c484", "#c87084", "#70b8b8", "#b470b0", "#68a4c8", "#a470c8", "#6888c8",
            "#9470cc", "#7070d0", "#c8c8c8", "#dcc084", "#b0d484", "#dcac84", "#84d89c",
            "#dc849c", "#84c8c8", "#c484c0", "#7cb8dc", "#b484dc", "#7ca0dc", "#a884dc",
            "#8484e0", "#dcdcdc", "#ecd09c", "#c0e89c", "#ecc09c", "#9ce8b4", "#ec9cb4",
            "#9cdcdc", "#d09cd0", "#90ccec", "#c49cec", "#90b4ec", "#b89cec", "#9c9cec",
            "#ececec", "#fce0b0", "#d4fcb0", "#fcd4b0", "#b0fcc8", "#fcb0c8", "#b0ecec",
            "#e0b0e0", "#a4e0fc", "#d4b0fc", "#a4c8fc", "#c8b0fc", "#b0b0fc",
        ],
    },
    PredefinedPalette {
        name: "C64 (16)",
        hex_colors: &[
            "#000000", "#626262", "#898989", "#adadad", "#ffffff", "#9f4e44", "#cb7e75",
            "#6d5412", "#a1683c", "#c9d487", "#9ae29b", "#5cab5e", "#6abfc6", "#887ecb",
            "#50459b", "#a057a3",
        ],
    },
    PredefinedPalette {
        name: "ZX Spectrum (15)",
        hex_colors: &[
            "#000000", "#0000d8", "#0000ff", "#d80000", "#ff0000", "#d800d8", "#ff00ff",
            "#00d800", "#00ff00", "#00d8d8", "#00ffff", "#d8d800", "#ffff00", "#d8d8d8",
            "#ffffff",
        ],
    },
    PredefinedPalette {
        name: "CGA (16)",
        hex_colors: &[
            "#000000", "#555555", "#aaaaaa", "#ffffff", "#0000aa", "#5555ff", "#00aa00",
            "#55ff55", "#00aaaa", "#55ffff", "#aa0000", "#ff5555", "#aa00aa", "#ff55ff",
            "#aa5500", "#ffff55",
        ],
    },
    PredefinedPalette {
        name: "EGA (64)",
        hex_colors: &[
            "#000000", "#0000aa", "#00aa00", "#00aaaa", "#aa0000", "#aa00aa", "#aaaa00",
            "#aaaaaa", "#000055", "#0000ff", "#00aa55", "#00aaff", "#aa0055", "#aa00ff",
            "#aaaa55", "#aaaaff", "#005500", "#0055aa", "#00ff00", "#00ffaa", "#aa5500",
            "#aa55aa", "#aaff00", "#aaffaa", "#005555", "#0055ff", "#00ff55", "#00ffff",
            "#aa5555", "#aa55ff", "#aaff55", "#aaffff", "#550000", "#5500aa", "#55aa00",
            "#55aaaa", "#ff0000", "#ff00aa", "#ffaa00", "#ffaaaa", "#550055", "#5500ff",
            "#55aa55", "#55aaff", "#ff0055", "#ff00ff", "#ffaa55", "#ffaaff", "#555500",
            "#5555aa", "#55ff00", "#55ffaa", "#ff5500", "#ff55aa", "#ffff00", "#ffffaa",
            "#555555", "#5555ff", "#55ff55", "#55ffff", "#ff5555", "#ff55ff", "#ffff55",
            "#ffffff",
        ],
    },
    PredefinedPalette {
        name: "Amstrad CPC (27)",
        hex_colors: &[
            "#040404", "#808080", "#ffffff", "#800000", "#ff0000", "#ff8080", "#ff7f00",
            "#ffff80", "#ffff00", "#808000", "#008000", "#01ff00", "#80ff00", "#80ff80",
            "#01ff80", "#008080", "#01ffff", "#80ffff", "#0080ff", "#0000ff", "#00007f",
            "#7f00ff", "#8080ff", "#ff80ff", "#ff00ff", "#ff0080", "#800080",
        ],
    },
    PredefinedPalette {
        name: "MSX (15)",
        hex_colors: &[
            "#000000", "#cacaca", "#ffffff", "#b75e51", "#d96459", "#fe877c", "#cac15e",
            "#ddce85", "#3ca042", "#40b64a", "#73ce7c", "#5955df", "#7e75f0", "#64daee",
            "#b565b3",
        ],
    },
    PredefinedPalette {
        name: "Apple II (15)",
        hex_colors: &[
            "#000000", "#515c16", "#843d52", "#ea7d27", "#514888", "#e85def", "#f5b7c9",
            "#006752", "#00c82c", "#919191", "#c9d199", "#00a6f0", "#98dbc9", "#c8c1f7",
            "#ffffff",
        ],
    },
    PredefinedPalette {
        name: "MacPaint (2)",
        hex_colors: &["#8bc8fe", "#051b2c"],
    },
    PredefinedPalette {
        name: "Intellivision (16)",
        hex_colors: &[
            "#0c0005", "#a7a8a8", "#fffcff", "#ff3e00", "#ffa600", "#faea27", "#00780f",
            "#00a720", "#6ccd30", "#002dff", "#5acbff", "#bd95ff", "#c81a7d", "#ff3276",
            "#3c5800", "#c9d464",
        ],
    },
    PredefinedPalette {
        name: "Windows 95 (256)",
        hex_colors: &[
            "#000000", "#800000", "#008000", "#808000", "#000080", "#800080", "#008080",
            "#c0c0c0", "#c0dcc0", "#a6caf0", "#2a3faa", "#2a3fff", "#2a5f00", "#2a5f55",
            "#2a5faa", "#2a5fff", "#2a7f00", "#2a7f55", "#2a7faa", "#2a7fff", "#2a9f00",
            "#2a9f55", "#2a9faa", "#2a9fff", "#2abf00", "#2abf55", "#2abfaa", "#2abfff",
            "#2adf00", "#2adf55", "#2adfaa", "#2adfff", "#2aff00", "#2aff55", "#2affaa",
            "#2affff", "#550000", "#550055", "#5500aa", "#5500ff", "#551f00", "#551f55",
            "#551faa", "#551fff", "#553f00", "#553f55", "#553faa", "#553fff", "#555f00",
            "#555f55", "#555faa", "#555fff", "#557f00", "#557f55", "#557faa", "#557fff",
            "#559f00", "#559f55", "#559faa", "#559fff", "#55bf00", "#55bf55", "#55bfaa",
            "#55bfff", "#55df00", "#55df55", "#55dfaa", "#55dfff", "#55ff00", "#55ff55",
            "#55ffaa", "#55ffff", "#7f0000", "#7f0055", "#7f00aa", "#7f00ff", "#7f1f00",
            "#7f1f55", "#7f1faa", "#7f1fff", "#7f3f00", "#7f3f55", "#7f3faa", "#7f3fff",
            "#7f5f00", "#7f5f55", "#7f5faa", "#7f5fff", "#7f7f00", "#7f7f55", "#7f7faa",
            "#7f7fff", "#7f9f00", "#7f9f55", "#7f9faa", "#7f9fff", "#7fbf00", "#7fbf55",
            "#7fbfaa", "#7fbfff", "#7fdf00", "#7fdf55", "#7fdfaa", "#7fdfff", "#7fff00",
            "#7fff55", "#7fffaa", "#7fffff", "#aa0000", "#aa0055", "#aa00aa", "#aa00ff",
            "#aa1f00", "#aa1f55", "#aa1faa", "#aa1fff", "#aa3f00", "#aa3f55", "#aa3faa",
            "#aa3fff", "#aa5f00", "#aa5f55", "#aa5faa", "#aa5fff", "#aa7f00", "#aa7f55",
            "#aa7faa", "#aa7fff", "#aa9f00", "#aa9f55", "#aa9faa", "#aa9fff", "#aabf00",
            "#aabf55", "#aabfaa", "#aabfff", "#aadf00", "#aadf55", "#aadfaa", "#aadfff",
            "#aaff00", "#aaff55", "#aaffaa", "#aaffff", "#d40000", "#d40055", "#d400aa",
            "#d400ff", "#d41f00", "#d41f55", "#d41faa", "#d41fff", "#d43f00", "#d43f55",
            "#d43faa", "#d43fff", "#d45f00", "#d45f55", "#d45faa", "#d45fff", "#d47f00",
            "#d47f55", "#d47faa", "#d47fff", "#d49f00", "#d49f55", "#d49faa", "#d49fff",
            "#d4bf00", "#d4bf55", "#d4bfaa", "#d4bfff", "#d4df00", "#d4df55", "#d4dfaa",
            "#d4dfff", "#d4ff00", "#d4ff55", "#d4ffaa", "#d4ffff", "#ff0055", "#ff00aa",
            "#ff1f00", "#ff1f55", "#ff1faa", "#ff1fff", "#ff3f00", "#ff3f55", "#ff3faa",
            "#ff3fff", "#ff5f00", "#ff5f55", "#ff5faa", "#ff5fff", "#ff7f00", "#ff7f55",
            "#ff7faa", "#ff7fff", "#ff9f00", "#ff9f55", "#ff9faa", "#ff9fff", "#ffbf00",
            "#ffbf55", "#ffbfaa", "#ffbfff", "#ffdf00", "#ffdf55", "#ffdfaa", "#ffdfff",
            "#ffff55", "#ffffaa", "#ccccff", "#ffccff", "#33ffff", "#66ffff", "#99ffff",
            "#ccffff", "#007f00", "#007f55", "#007faa", "#007fff", "#009f00", "#009f55",
            "#009faa", "#009fff", "#00bf00", "#00bf55", "#00bfaa", "#00bfff", "#00df00",
            "#00df55", "#00dfaa", "#00dfff", "#00ff55", "#00ffaa", "#2a0000", "#2a0055",
            "#2a00aa", "#2a00ff", "#2a1f00", "#2a1f55", "#2a1faa", "#2a1fff", "#2a3f00",
            "#2a3f55", "#fffbf0", "#a0a0a4", "#808080", "#ff0000", "#00ff00", "#ffff00",
            "#0000ff", "#ff00ff", "#00ffff", "#ffffff",
        ],
    },
];

fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let rf = r as f32 / 255.0;
    let gf = g as f32 / 255.0;
    let bf = b as f32 / 255.0;
    let max = rf.max(gf).max(bf);
    let min = rf.min(gf).min(bf);
    let delta = max - min;

    let h = if delta <= 1e-5 {
        0.0
    } else if (max - rf).abs() <= 1e-5 {
        60.0 * (((gf - bf) / delta).rem_euclid(6.0))
    } else if (max - gf).abs() <= 1e-5 {
        60.0 * (((bf - rf) / delta) + 2.0)
    } else {
        60.0 * (((rf - gf) / delta) + 4.0)
    };

    let s = if max <= 1e-5 { 0.0 } else { delta / max };
    (h, s, max)
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let h_norm = h.rem_euclid(360.0);
    let c = v * s;
    let x = c * (1.0 - ((h_norm / 60.0).rem_euclid(2.0) - 1.0).abs());
    let m = v - c;

    let (r1, g1, b1) = if h_norm < 60.0 {
        (c, x, 0.0)
    } else if h_norm < 120.0 {
        (x, c, 0.0)
    } else if h_norm < 180.0 {
        (0.0, c, x)
    } else if h_norm < 240.0 {
        (0.0, x, c)
    } else if h_norm < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let r = ((r1 + m) * 255.0).round().clamp(0.0, 255.0) as u8;
    let g = ((g1 + m) * 255.0).round().clamp(0.0, 255.0) as u8;
    let b = ((b1 + m) * 255.0).round().clamp(0.0, 255.0) as u8;
    (r, g, b)
}

/// Generates `steps` intermediate colors between `start` and `end` with a parabolic `hue_shift_deg` curve.
pub fn generate_color_ramp(
    start: [u8; 4],
    end: [u8; 4],
    steps: u32,
    hue_shift_deg: f32,
) -> Vec<[u8; 4]> {
    let count = steps.clamp(1, 32);
    let (h0, s0, v0) = rgb_to_hsv(start[0], start[1], start[2]);
    let (h1, s1, v1) = rgb_to_hsv(end[0], end[1], end[2]);

    // Shortest angular hue difference in [-180, 180]
    let mut dh = (h1 - h0).rem_euclid(360.0);
    if dh > 180.0 {
        dh -= 360.0;
    }

    let mut ramp = Vec::with_capacity(count as usize);
    for i in 1..=count {
        let t = i as f32 / (count + 1) as f32;
        let bell = 4.0 * t * (1.0 - t);
        let h = h0 + dh * t + hue_shift_deg * bell;
        let s = (s0 + (s1 - s0) * t).clamp(0.0, 1.0);
        let v = (v0 + (v1 - v0) * t).clamp(0.0, 1.0);
        let (r, g, b) = hsv_to_rgb(h, s, v);
        ramp.push([r, g, b, 255]);
    }
    ramp
}

/// Replaces all occurrences of `old_col` with `new_col` across the entire `atlas_rgba`.
pub fn replace_color_in_atlas(
    atlas_rgba: &mut RgbaImage,
    old_col: [u8; 4],
    new_col: [u8; 4],
) -> bool {
    if old_col == new_col {
        return false;
    }
    let target = Rgba(old_col);
    let replacement = Rgba(new_col);
    let mut changed = false;

    for pixel in atlas_rgba.pixels_mut() {
        if *pixel == target {
            *pixel = replacement;
            changed = true;
        }
    }
    changed
}

/// Remaps pixels across `atlas_rgba` from `from_colors` slots to `to_colors` slots in a single pass.
pub fn apply_palette_swap_to_atlas(
    atlas_rgba: &mut RgbaImage,
    from_colors: &[[u8; 4]],
    to_colors: &[[u8; 4]],
) -> bool {
    let len = from_colors.len().min(to_colors.len());
    if len == 0 {
        return false;
    }

    let mut changed = false;
    for pixel in atlas_rgba.pixels_mut() {
        if pixel[3] == 0 {
            continue;
        }
        let cur = [pixel[0], pixel[1], pixel[2], pixel[3]];
        if let Some(idx) = from_colors[..len].iter().position(|c| *c == cur) {
            let mapped = to_colors[idx];
            if mapped != cur {
                *pixel = Rgba(mapped);
                changed = true;
            }
        }
    }
    changed
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaletteSortCriterion {
    Hue,
    Luminance,
    Saturation,
    Reverse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaletteDitherMode {
    None,
    Bayer4x4,
    Bayer8x8,
}

/// Converts RGB [0..255] to HSL (Hue: 0..360°, Saturation: 0..1, Lightness: 0..1).
pub fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let rf = r as f32 / 255.0;
    let gf = g as f32 / 255.0;
    let bf = b as f32 / 255.0;

    let max = rf.max(gf).max(bf);
    let min = rf.min(gf).min(bf);
    let delta = max - min;

    let l = (max + min) * 0.5;

    if delta < 1e-5 {
        return (0.0, 0.0, l);
    }

    let s = if l < 0.5 {
        delta / (max + min)
    } else {
        delta / (2.0 - max - min)
    };

    let mut h = if (max - rf).abs() < 1e-5 {
        (gf - bf) / delta
    } else if (max - gf).abs() < 1e-5 {
        2.0 + (bf - rf) / delta
    } else {
        4.0 + (rf - gf) / delta
    };

    h *= 60.0;
    if h < 0.0 {
        h += 360.0;
    }

    (h, s, l)
}

/// Perceptual luminance calculation (ITU-R BT.601).
pub fn color_luminance(c: [u8; 4]) -> f32 {
    0.299 * (c[0] as f32) + 0.587 * (c[1] as f32) + 0.114 * (c[2] as f32)
}

/// Sorts the palette colors in place according to the chosen criterion.
pub fn sort_palette_colors(colors: &mut [ [u8; 4] ], criterion: PaletteSortCriterion) {
    match criterion {
        PaletteSortCriterion::Reverse => {
            colors.reverse();
        }
        PaletteSortCriterion::Hue => {
            colors.sort_by(|a, b| {
                let (ha, sa, la) = rgb_to_hsl(a[0], a[1], a[2]);
                let (hb, sb, lb) = rgb_to_hsl(b[0], b[1], b[2]);
                let neutral_a = sa < 0.05;
                let neutral_b = sb < 0.05;
                match (neutral_a, neutral_b) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    (true, true) => la.total_cmp(&lb),
                    (false, false) => ha
                        .total_cmp(&hb)
                        .then_with(|| la.total_cmp(&lb))
                        .then_with(|| sa.total_cmp(&sb)),
                }
            });
        }
        PaletteSortCriterion::Luminance => {
            colors.sort_by(|a, b| {
                let la = color_luminance(*a);
                let lb = color_luminance(*b);
                la.total_cmp(&lb)
            });
        }
        PaletteSortCriterion::Saturation => {
            colors.sort_by(|a, b| {
                let (_, sa, la) = rgb_to_hsl(a[0], a[1], a[2]);
                let (_, sb, lb) = rgb_to_hsl(b[0], b[1], b[2]);
                sa.total_cmp(&sb).then_with(|| la.total_cmp(&lb))
            });
        }
    }
}

/// Finds the nearest color in `palette` matching `target` using human perceptual redmean distance.
pub fn nearest_palette_color_rgb(target: [u8; 4], palette: &[[u8; 4]]) -> [u8; 4] {
    if palette.is_empty() {
        return target;
    }
    if target[3] == 0 {
        return [0, 0, 0, 0];
    }
    let tr = target[0] as f32;
    let tg = target[1] as f32;
    let tb = target[2] as f32;

    let mut best_col = palette[0];
    let mut best_dist = f32::MAX;

    for &col in palette {
        let dr = tr - col[0] as f32;
        let dg = tg - col[1] as f32;
        let db = tb - col[2] as f32;
        let r_mean = (tr + col[0] as f32) * 0.5;
        let weight_r = 2.0 + (r_mean / 256.0);
        let weight_g = 4.0;
        let weight_b = 2.0 + ((255.0 - r_mean) / 256.0);
        let dist = weight_r * dr * dr + weight_g * dg * dg + weight_b * db * db;
        if dist < best_dist {
            best_dist = dist;
            best_col = col;
        }
    }
    [best_col[0], best_col[1], best_col[2], target[3]]
}

/// Snaps pixel colors within the specified region/frame to nearest palette colors,
/// with optional Bayer dithering and selection boundary constraint.
#[allow(clippy::too_many_arguments)]
pub fn remap_pixels_to_palette(
    atlas_rgba: &mut RgbaImage,
    origin_x: u32,
    origin_y: u32,
    width: u32,
    height: u32,
    palette: &[[u8; 4]],
    dither: PaletteDitherMode,
    dither_strength: f32,
    selection: Option<&HashSet<(i32, i32)>>,
) -> bool {
    if palette.is_empty() || width == 0 || height == 0 {
        return false;
    }
    let atlas_w = atlas_rgba.width();
    let atlas_h = atlas_rgba.height();
    let mut changed = false;

    for ly in 0..height {
        let gy = origin_y + ly;
        if gy >= atlas_h {
            break;
        }
        for lx in 0..width {
            let gx = origin_x + lx;
            if gx >= atlas_w {
                break;
            }
            if let Some(sel) = selection {
                if !sel.contains(&(lx as i32, ly as i32)) {
                    continue;
                }
            }

            let cur = *atlas_rgba.get_pixel(gx, gy);
            if cur[3] == 0 {
                continue;
            }
            let cur_arr = [cur[0], cur[1], cur[2], cur[3]];
            let mapped = match dither {
                PaletteDitherMode::None => nearest_palette_color_rgb(cur_arr, palette),
                PaletteDitherMode::Bayer4x4 => {
                    let d = (crate::tools::fill::BAYER_4X4[(ly % 4) as usize][(lx % 4) as usize] - 0.5) * dither_strength;
                    let pr = (cur_arr[0] as f32 + d).clamp(0.0, 255.0) as u8;
                    let pg = (cur_arr[1] as f32 + d).clamp(0.0, 255.0) as u8;
                    let pb = (cur_arr[2] as f32 + d).clamp(0.0, 255.0) as u8;
                    nearest_palette_color_rgb([pr, pg, pb, cur_arr[3]], palette)
                }
                PaletteDitherMode::Bayer8x8 => {
                    let d = (crate::tools::fill::BAYER_8X8[(ly % 8) as usize][(lx % 8) as usize] - 0.5) * dither_strength;
                    let pr = (cur_arr[0] as f32 + d).clamp(0.0, 255.0) as u8;
                    let pg = (cur_arr[1] as f32 + d).clamp(0.0, 255.0) as u8;
                    let pb = (cur_arr[2] as f32 + d).clamp(0.0, 255.0) as u8;
                    nearest_palette_color_rgb([pr, pg, pb, cur_arr[3]], palette)
                }
            };
            if mapped != cur_arr {
                atlas_rgba.put_pixel(gx, gy, Rgba(mapped));
                changed = true;
            }
        }
    }
    changed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_and_pal() {
        let hex_txt = "#14121d\n302c42\n// comment\n";
        let parsed = parse_palette_hex_or_pal_helper(hex_txt);
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0], [0x14, 0x12, 0x1d, 255]);

        let pal_txt = "JASC-PAL\n0100\n2\n255 128 0\n10 20 30\n";
        let parsed_pal = parse_palette_text(pal_txt);
        assert_eq!(parsed_pal.len(), 2);
        assert_eq!(parsed_pal[0], [255, 128, 0, 255]);
    }

    fn parse_palette_hex_or_pal_helper(s: &str) -> Vec<[u8; 4]> {
        parse_palette_text(s)
    }

    #[test]
    fn test_generate_color_ramp() {
        let ramp = generate_color_ramp([20, 20, 40, 255], [220, 220, 240, 255], 3, 15.0);
        assert_eq!(ramp.len(), 3);
        assert!(ramp[0][2] < ramp[2][2]);
    }

    #[test]
    fn test_palette_swaps_rename_and_apply() {
        let mut atlas = RgbaImage::new(2, 1);
        atlas.put_pixel(0, 0, Rgba([255, 0, 0, 255]));
        atlas.put_pixel(1, 0, Rgba([0, 255, 0, 255]));

        let mut config = PaletteData {
            colors: vec![[255, 0, 0, 255], [0, 255, 0, 255]],
            swaps: vec![
                PaletteSwap {
                    name: "Default".to_string(),
                    colors: vec![[255, 0, 0, 255], [0, 255, 0, 255]],
                },
                PaletteSwap {
                    name: "Inverted".to_string(),
                    colors: vec![[0, 255, 0, 255], [255, 0, 0, 255]],
                },
            ],
            ..Default::default()
        };

        // Rename skin
        config.swaps[1].name = "Alt Hero".to_string();
        assert_eq!(config.swaps[1].name, "Alt Hero");

        // Swap to Alt Hero
        assert!(apply_palette_swap_to_atlas(
            &mut atlas,
            &config.swaps[0].colors,
            &config.swaps[1].colors,
        ));
        assert_eq!(*atlas.get_pixel(0, 0), Rgba([0, 255, 0, 255]));
        assert_eq!(*atlas.get_pixel(1, 0), Rgba([255, 0, 0, 255]));
    }

    #[test]
    fn test_format_palette_gpl() {
        let colors = vec![[255, 0, 0, 255], [0, 255, 0, 255]];
        let gpl = format_palette_gpl("HeroPalette", &colors);
        assert!(gpl.starts_with("GIMP Palette"));
        assert!(gpl.contains("Name: HeroPalette"));
        assert!(gpl.contains("255   0   0"));
    }

    #[test]
    fn test_palette_sort_and_remap() {
        let mut cols = vec![
            [255, 255, 255, 255], // White
            [0, 0, 0, 255],       // Black
            [0, 0, 255, 255],     // Blue
            [255, 0, 0, 255],     // Red
            [0, 255, 0, 255],     // Green
        ];

        // Sort by luminance: Black is darkest, White is lightest
        sort_palette_colors(&mut cols, PaletteSortCriterion::Luminance);
        assert_eq!(cols[0], [0, 0, 0, 255]);
        assert_eq!(*cols.last().unwrap(), [255, 255, 255, 255]);

        // Reverse: White first, Black last
        sort_palette_colors(&mut cols, PaletteSortCriterion::Reverse);
        assert_eq!(cols[0], [255, 255, 255, 255]);
        assert_eq!(*cols.last().unwrap(), [0, 0, 0, 255]);

        // Nearest palette color test
        let palette = vec![[255, 0, 0, 255], [0, 0, 255, 255]];
        let red_approx = nearest_palette_color_rgb([240, 20, 10, 255], &palette);
        assert_eq!(red_approx, [255, 0, 0, 255]);

        let blue_approx = nearest_palette_color_rgb([15, 10, 245, 255], &palette);
        assert_eq!(blue_approx, [0, 0, 255, 255]);

        // Remap pixels
        let mut img = RgbaImage::new(2, 2);
        img.put_pixel(0, 0, Rgba([230, 15, 10, 255]));
        img.put_pixel(1, 0, Rgba([10, 15, 230, 255]));
        img.put_pixel(0, 1, Rgba([0, 0, 0, 0])); // transparent

        assert!(remap_pixels_to_palette(
            &mut img,
            0,
            0,
            2,
            2,
            &palette,
            PaletteDitherMode::None,
            16.0,
            None
        ));
        assert_eq!(*img.get_pixel(0, 0), Rgba([255, 0, 0, 255]));
        assert_eq!(*img.get_pixel(1, 0), Rgba([0, 0, 255, 255]));
        assert_eq!(*img.get_pixel(0, 1), Rgba([0, 0, 0, 0]));
    }

    #[test]
    fn test_predefined_palettes_validity() {
        assert!(PREDEFINED_PALETTES.len() >= 26);
        let names: Vec<&str> = PREDEFINED_PALETTES.iter().map(|p| p.name).collect();
        assert!(names.contains(&"DawnBringer (8)"));
        assert!(names.contains(&"DawnBringer (16)"));
        assert!(names.contains(&"DawnBringer (32)"));
        assert!(names.contains(&"Windows 95 (256)"));

        for p in PREDEFINED_PALETTES {
            assert!(!p.hex_colors.is_empty(), "Palette {} is empty", p.name);
            let colors = p.to_rgba_vec();
            assert_eq!(
                colors.len(),
                p.hex_colors.len(),
                "Palette {} failed to parse some colors",
                p.name
            );
        }

        let win95 = PREDEFINED_PALETTES
            .iter()
            .find(|p| p.name == "Windows 95 (256)")
            .expect("Windows 95 (256) palette not found");
        assert_eq!(win95.hex_colors.len(), 256);
    }
}
