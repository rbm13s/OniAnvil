use image::{Rgba, RgbaImage};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConvolutionPreset {
    #[default]
    Blur,
    Sharpen,
    EdgeDetect,
    Emboss,
    Custom,
}

impl ConvolutionPreset {
    pub const ALL: [Self; 5] = [
        Self::Blur,
        Self::Sharpen,
        Self::EdgeDetect,
        Self::Emboss,
        Self::Custom,
    ];

    pub fn kernel(&self) -> ([[f32; 3]; 3], f32, f32) {
        match self {
            Self::Blur => (
                [
                    [1.0, 2.0, 1.0],
                    [2.0, 4.0, 2.0],
                    [1.0, 2.0, 1.0],
                ],
                16.0,
                0.0,
            ),
            Self::Sharpen => (
                [
                    [0.0, -1.0, 0.0],
                    [-1.0, 5.0, -1.0],
                    [0.0, -1.0, 0.0],
                ],
                1.0,
                0.0,
            ),
            Self::EdgeDetect => (
                [
                    [-1.0, -1.0, -1.0],
                    [-1.0, 8.0, -1.0],
                    [-1.0, -1.0, -1.0],
                ],
                1.0,
                0.0,
            ),
            Self::Emboss => (
                [
                    [-2.0, -1.0, 0.0],
                    [-1.0, 1.0, 1.0],
                    [0.0, 1.0, 2.0],
                ],
                1.0,
                128.0,
            ),
            Self::Custom => (
                [
                    [0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0],
                ],
                1.0,
                0.0,
            ),
        }
    }
}

/// Applies a 3x3 convolution matrix kernel to the frame region, optionally restricted to a selection.
#[allow(clippy::too_many_arguments)]
pub fn apply_convolution_matrix(
    atlas_rgba: &mut RgbaImage,
    origin_x: u32,
    origin_y: u32,
    region_w: u32,
    region_h: u32,
    kernel: &[[f32; 3]; 3],
    divisor: f32,
    bias: f32,
    preserve_alpha: bool,
    selection: Option<&HashSet<(i32, i32)>>,
) -> bool {
    if region_w == 0 || region_h == 0 {
        return false;
    }
    let div = if divisor.abs() < 0.0001 { 1.0 } else { divisor };

    // Read source frame buffer first so convolutions don't mutate input data in-place
    let mut src = vec![[0u8; 4]; (region_w * region_h) as usize];
    for ly in 0..(region_h as i32) {
        for lx in 0..(region_w as i32) {
            let ax = origin_x + lx as u32;
            let ay = origin_y + ly as u32;
            if ax < atlas_rgba.width() && ay < atlas_rgba.height() {
                src[(ly as u32 * region_w + lx as u32) as usize] = atlas_rgba.get_pixel(ax, ay).0;
            }
        }
    }

    let mut changed = false;

    for ly in 0..(region_h as i32) {
        for lx in 0..(region_w as i32) {
            if let Some(sel) = selection {
                if !sel.is_empty() && !sel.contains(&(lx, ly)) {
                    continue;
                }
            }

            let idx = (ly as u32 * region_w + lx as u32) as usize;
            let orig = src[idx];
            if preserve_alpha && orig[3] == 0 {
                continue;
            }

            let mut sum_r = 0.0f32;
            let mut sum_g = 0.0f32;
            let mut sum_b = 0.0f32;
            let mut sum_a = 0.0f32;

            for ky in -1..=1 {
                let sy = (ly + ky).clamp(0, region_h as i32 - 1);
                for kx in -1..=1 {
                    let sx = (lx + kx).clamp(0, region_w as i32 - 1);
                    let s_idx = (sy as u32 * region_w + sx as u32) as usize;
                    let p = src[s_idx];
                    let weight = kernel[(ky + 1) as usize][(kx + 1) as usize];

                    sum_r += p[0] as f32 * weight;
                    sum_g += p[1] as f32 * weight;
                    sum_b += p[2] as f32 * weight;
                    sum_a += p[3] as f32 * weight;
                }
            }

            let r = ((sum_r / div) + bias).clamp(0.0, 255.0).round() as u8;
            let g = ((sum_g / div) + bias).clamp(0.0, 255.0).round() as u8;
            let b = ((sum_b / div) + bias).clamp(0.0, 255.0).round() as u8;
            let a = if preserve_alpha {
                orig[3]
            } else {
                ((sum_a / div) + bias).clamp(0.0, 255.0).round() as u8
            };

            let new_pixel = [r, g, b, a];
            if new_pixel != orig {
                let ax = origin_x + lx as u32;
                let ay = origin_y + ly as u32;
                if ax < atlas_rgba.width() && ay < atlas_rgba.height() {
                    atlas_rgba.put_pixel(ax, ay, Rgba(new_pixel));
                    changed = true;
                }
            }
        }
    }

    changed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convolution_blur_and_edge_detect() {
        let mut img = RgbaImage::from_pixel(4, 4, Rgba([0, 0, 0, 255]));
        // Put white pixel in center at (1, 1)
        img.put_pixel(1, 1, Rgba([255, 255, 255, 255]));

        let (blur_k, blur_div, blur_bias) = ConvolutionPreset::Blur.kernel();
        let mut blurred = img.clone();
        assert!(apply_convolution_matrix(
            &mut blurred,
            0,
            0,
            4,
            4,
            &blur_k,
            blur_div,
            blur_bias,
            true,
            None,
        ));
        // Center pixel diffused from 255 down to ~64 (4/16 * 255)
        let c = blurred.get_pixel(1, 1);
        assert!(c[0] < 200 && c[0] > 40);
        // Orthogonal neighbor received energy
        let neighbor = blurred.get_pixel(1, 0);
        assert!(neighbor[0] > 0);

        // Edge detect on solid image produces 0
        let mut solid = RgbaImage::from_pixel(4, 4, Rgba([128, 128, 128, 255]));
        let (edge_k, edge_div, edge_bias) = ConvolutionPreset::EdgeDetect.kernel();
        let _ = apply_convolution_matrix(
            &mut solid,
            0,
            0,
            4,
            4,
            &edge_k,
            edge_div,
            edge_bias,
            true,
            None,
        );
        assert_eq!(*solid.get_pixel(1, 1), Rgba([0, 0, 0, 255]));
    }
}
