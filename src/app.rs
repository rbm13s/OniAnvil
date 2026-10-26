use crate::i18n::I18n;
use crate::io::autosave::AutosaveService;
use crate::io::export::{encode_animated_gif_bytes, encode_scaled_png_bytes};
use crate::io::settings::AppSettings;
use crate::io::storage::{IoEvent, IoManager};
use crate::io::updater::UpdateChecker;
use crate::model::document::ProjectDocument;
use crate::model::palette::{remap_pixels_to_palette, sort_palette_colors, PaletteSortCriterion};
use crate::model::project::Project;
use crate::theme::{setup_custom_fonts, AppTheme, FA_CLOSE, FA_PLUS};
use crate::tools::fill::auto_outline_region;
use crate::tools::transforms::{
    flip_region_horizontal, flip_region_vertical, flip_selection_horizontal,
    flip_selection_vertical, nudge_region, nudge_selection_region, rotate_region_90_cw,
    rotate_region_arbitrary, rotate_selection_90_cw, rotate_selection_arbitrary,
};
use crate::tools::{ToolKind, ToolSettings};
use crate::ui::bottom_dock::show_bottom_dock;
use crate::ui::center_workspace::show_center_workspace;
use crate::ui::left_dock::show_left_dock;
use crate::ui::modals::{
    show_about_modal, show_confirm_close_doc_modal, show_confirm_delete_frame_modal,
    show_confirm_delete_layer_modal,
    show_confirm_delete_palette_slot_modal, show_confirm_delete_pivot_point_modal,
    show_confirm_delete_recent_modal, show_confirm_delete_skin_modal,
    show_confirm_delete_tag_modal, show_confirm_exit_app_modal, show_hotkeys_modal,
    show_recovery_modal, ColorAdjustModalState, ConfirmCloseDocAction, ConvolutionAction,
    ConvolutionModalState, ExportGifModalState, ExportPackedAtlasModalState,
    ExportScaledPngModalState, ModifySelectionMode, ModifySelectionModalState, NewProjectWizardState,
    OutlineFxAction, OutlineFxModalState,
    ReGridModalState, RecoveryAction, RemapPaletteAction, RemapPaletteModalState, ReplaceColorModalState, Resize9GridModalState, RotateAction, RotateModalState, SheetSlicerModalState,
};
use crate::tools::selection::SelectionModifyShape;
use crate::ui::right_dock::show_right_dock;
use crate::ui::start_screen::{show_start_screen, StartScreenAction};
use crate::ui::top_bar::show_top_bars;
use egui::{Color32, Context, TopBottomPanel};
use image::Rgba;
use serde::{Deserialize, Serialize};

pub const CLIPBOARD_PREFIX: &str = "ONIANVIL_CLIPBOARD:";

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PixelClipboard {
    pub pixels: Vec<((i32, i32), [u8; 4])>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PendingAfterSave {
    CloseDoc(usize),
    CloseApp,
}

pub struct OniAnvilApp {
    pub i18n: I18n,
    pub clipboard: Option<PixelClipboard>,
    pub documents: Vec<ProjectDocument>,
    pub active_doc_idx: usize,
    pub pending_close_doc_idx: Option<usize>,
    pub settings: AppSettings,
    pub tools: ToolSettings,
    pub io: IoManager,
    pub autosave: AutosaveService,
    pub update_checker: UpdateChecker,
    pub wizard: NewProjectWizardState,
    pub regrid_modal: ReGridModalState,
    pub resize9_modal: Resize9GridModalState,
    pub slicer_modal: SheetSlicerModalState,
    pub export_png_modal: ExportScaledPngModalState,
    pub export_gif_modal: ExportGifModalState,
    pub replace_color_modal: ReplaceColorModalState,
    pub color_adjust_modal: ColorAdjustModalState,
    pub modify_selection_modal: ModifySelectionModalState,
    pub outline_fx_modal: OutlineFxModalState,
    pub convolution_modal: ConvolutionModalState,
    pub remap_palette_modal: RemapPaletteModalState,
    pub rotate_modal: RotateModalState,
    pub pack_atlas_modal: ExportPackedAtlasModalState,
    pub pending_after_save: Option<PendingAfterSave>,
    pub pending_delete_frame: Option<usize>,
    pub pending_delete_layer: Option<(usize, String)>,
    pub pending_delete_skin: Option<(usize, String)>,
    pub pending_delete_palette_slot: Option<(usize, [u8; 4])>,
    pub pending_delete_pivot_point: Option<(usize, String)>,
    pub pending_delete_tag: Option<(usize, String)>,
    pub pending_delete_recent: Option<std::path::PathBuf>,
    pub show_right_dock: bool,
    pub show_bottom_dock: bool,
    pub show_about_modal: bool,
    pub show_hotkeys_modal: bool,
    pub show_confirm_exit_modal: bool,
    pub confirmed_exit: bool,
    pub app_theme: AppTheme,
    pub status_key: &'static str,
    pub logo_texture: Option<egui::TextureHandle>,
    pub modal_preview_backup: Option<(usize, image::RgbaImage)>,
    pub modal_selection_preview_backup: Option<(usize, crate::tools::selection::SelectionMask)>,
}

impl OniAnvilApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        let logo_texture = crate::theme::load_app_logo_texture(&cc.egui_ctx);
        let settings = AppSettings::load(cc.storage);
        settings.theme.apply(&cc.egui_ctx);
        let update_checker = UpdateChecker::start(cc.egui_ctx.clone());
        let autosave = AutosaveService::new(cc.storage);
        let tools = ToolSettings {
            two_columns: settings.tools_two_columns,
            ..Default::default()
        };

        Self {
            i18n: I18n::new(settings.language),
            clipboard: None,
            documents: Vec::new(),
            active_doc_idx: 0,
            pending_close_doc_idx: None,
            settings: settings.clone(),
            tools,
            io: IoManager::new(),
            autosave,
            update_checker,
            wizard: NewProjectWizardState::default(),
            regrid_modal: ReGridModalState::default(),
            resize9_modal: Resize9GridModalState::default(),
            slicer_modal: SheetSlicerModalState::default(),
            export_png_modal: ExportScaledPngModalState::default(),
            export_gif_modal: ExportGifModalState::default(),
            replace_color_modal: ReplaceColorModalState::default(),
            color_adjust_modal: ColorAdjustModalState::default(),
            modify_selection_modal: ModifySelectionModalState::default(),
            outline_fx_modal: OutlineFxModalState::default(),
            convolution_modal: ConvolutionModalState::default(),
            remap_palette_modal: RemapPaletteModalState::default(),
            rotate_modal: RotateModalState::default(),
            pack_atlas_modal: ExportPackedAtlasModalState::default(),
            modal_preview_backup: None,
            modal_selection_preview_backup: None,
            pending_after_save: None,
            pending_delete_frame: None,
            pending_delete_layer: None,
            pending_delete_skin: None,
            pending_delete_palette_slot: None,
            pending_delete_pivot_point: None,
            pending_delete_tag: None,
            pending_delete_recent: None,
            show_right_dock: true,
            show_bottom_dock: true,
            show_about_modal: false,
            show_hotkeys_modal: false,
            show_confirm_exit_modal: false,
            confirmed_exit: false,
            app_theme: settings.theme,
            status_key: "app.status.ready",
            logo_texture,
        }
    }

    pub fn open_project_doc(&mut self, proj: Project) {
        if let Some(path) = &proj.file_path {
            self.settings.add_recent(path.clone());
        }
        let mut doc = ProjectDocument::new(proj);
        doc.viewport.needs_fit_to_view = true;
        self.documents.push(doc);
        self.active_doc_idx = self.documents.len() - 1;
    }

    pub fn request_close_doc(&mut self, idx: usize) {
        if idx >= self.documents.len() {
            return;
        }
        if self.documents[idx].project.has_unsaved_changes {
            self.pending_close_doc_idx = Some(idx);
        } else {
            self.close_doc_now(idx);
        }
    }

    pub fn close_doc_now(&mut self, idx: usize) {
        if idx < self.documents.len() {
            self.documents.remove(idx);
            if self.documents.is_empty() {
                self.active_doc_idx = 0;
            } else if self.active_doc_idx >= self.documents.len() {
                self.active_doc_idx = self.documents.len() - 1;
            } else if self.active_doc_idx > idx {
                self.active_doc_idx -= 1;
            }
        }
    }

    pub fn perform_undo(&mut self) {
        if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
            doc.perform_undo();
        }
    }

    pub fn perform_redo(&mut self) {
        if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
            doc.perform_redo();
        }
    }

    pub fn apply_flip_horizontal(&mut self) {
        let batch = self.tools.batch_mode;
        if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
            doc.record_history();
            let fw = doc.project.metadata.frame_width;
            let fh = doc.project.metadata.frame_height;
            let frames: Vec<usize> = if batch {
                (0..doc.project.metadata.frame_count as usize).collect()
            } else {
                vec![doc.project.active_frame]
            };
            if !doc.renderer.selection.is_empty() {
                let mut last_new = None;
                for f_idx in frames {
                    let (ox, oy) = doc.project.metadata.frame_origin(f_idx);
                    let np = flip_selection_horizontal(
                        &mut doc.project.atlas_rgba,
                        ox,
                        oy,
                        fw,
                        fh,
                        &doc.renderer.selection.pixels,
                    );
                    last_new = Some(np);
                }
                if let Some(np) = last_new {
                    doc.renderer.selection.pixels = np;
                }
            } else {
                for f_idx in frames {
                    let (ox, oy) = doc.project.metadata.frame_origin(f_idx);
                    flip_region_horizontal(&mut doc.project.atlas_rgba, ox, oy, fw, fh);
                }
            }
            doc.project.canvas_dirty = true;
        }
    }

    pub fn apply_flip_vertical(&mut self) {
        let batch = self.tools.batch_mode;
        if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
            doc.record_history();
            let fw = doc.project.metadata.frame_width;
            let fh = doc.project.metadata.frame_height;
            let frames: Vec<usize> = if batch {
                (0..doc.project.metadata.frame_count as usize).collect()
            } else {
                vec![doc.project.active_frame]
            };
            if !doc.renderer.selection.is_empty() {
                let mut last_new = None;
                for f_idx in frames {
                    let (ox, oy) = doc.project.metadata.frame_origin(f_idx);
                    let np = flip_selection_vertical(
                        &mut doc.project.atlas_rgba,
                        ox,
                        oy,
                        fw,
                        fh,
                        &doc.renderer.selection.pixels,
                    );
                    last_new = Some(np);
                }
                if let Some(np) = last_new {
                    doc.renderer.selection.pixels = np;
                }
            } else {
                for f_idx in frames {
                    let (ox, oy) = doc.project.metadata.frame_origin(f_idx);
                    flip_region_vertical(&mut doc.project.atlas_rgba, ox, oy, fw, fh);
                }
            }
            doc.project.canvas_dirty = true;
        }
    }

    pub fn apply_rotate_90_cw(&mut self) {
        let batch = self.tools.batch_mode;
        if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
            doc.record_history();
            let fw = doc.project.metadata.frame_width;
            let fh = doc.project.metadata.frame_height;
            let frames: Vec<usize> = if batch {
                (0..doc.project.metadata.frame_count as usize).collect()
            } else {
                vec![doc.project.active_frame]
            };
            if !doc.renderer.selection.is_empty() {
                let mut last_new = None;
                for f_idx in frames {
                    let (ox, oy) = doc.project.metadata.frame_origin(f_idx);
                    let np = rotate_selection_90_cw(
                        &mut doc.project.atlas_rgba,
                        ox,
                        oy,
                        fw,
                        fh,
                        &doc.renderer.selection.pixels,
                    );
                    last_new = Some(np);
                }
                if let Some(np) = last_new {
                    doc.renderer.selection.pixels = np;
                }
            } else {
                for f_idx in frames {
                    let (ox, oy) = doc.project.metadata.frame_origin(f_idx);
                    rotate_region_90_cw(&mut doc.project.atlas_rgba, ox, oy, fw, fh);
                }
            }
            doc.project.canvas_dirty = true;
        }
    }

    pub fn apply_rotate_arbitrary(&mut self, action: RotateAction) {
        let batch = action.all_frames;
        if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
            doc.record_history();
            let fw = doc.project.metadata.frame_width;
            let fh = doc.project.metadata.frame_height;
            let frames: Vec<usize> = if batch {
                (0..doc.project.metadata.frame_count as usize).collect()
            } else {
                vec![doc.project.active_frame]
            };
            if !doc.renderer.selection.is_empty() && action.in_selection_only {
                let mut last_new = None;
                for f_idx in frames {
                    let (ox, oy) = doc.project.metadata.frame_origin(f_idx);
                    let np = rotate_selection_arbitrary(
                        &mut doc.project.atlas_rgba,
                        ox,
                        oy,
                        fw,
                        fh,
                        action.angle_deg,
                        action.algorithm,
                        &doc.renderer.selection.pixels,
                    );
                    last_new = Some(np);
                }
                if let Some(np) = last_new {
                    doc.renderer.selection.pixels = np;
                }
            } else {
                for f_idx in frames {
                    let (ox, oy) = doc.project.metadata.frame_origin(f_idx);
                    rotate_region_arbitrary(
                        &mut doc.project.atlas_rgba,
                        ox,
                        oy,
                        fw,
                        fh,
                        action.angle_deg,
                        action.algorithm,
                    );
                }
            }
            doc.project.has_unsaved_changes = true;
            doc.project.canvas_dirty = true;
            self.status_key = "app.status.rotated";
        }
    }

    pub fn apply_auto_outline(&mut self) {
        let batch = self.tools.batch_mode;
        let outline_col = self.tools.fg_color;
        if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
            let snap = doc.current_snapshot();
            let fw = doc.project.metadata.frame_width;
            let fh = doc.project.metadata.frame_height;
            let frames: Vec<usize> = if batch {
                (0..doc.project.metadata.frame_count as usize).collect()
            } else {
                vec![doc.project.active_frame]
            };
            let mut changed = false;
            for f_idx in frames {
                let (ox, oy) = doc.project.metadata.frame_origin(f_idx);
                if auto_outline_region(&mut doc.project.atlas_rgba, ox, oy, fw, fh, outline_col) {
                    changed = true;
                }
            }
            if changed {
                doc.history.push(snap);
                doc.project.canvas_dirty = true;
            }
        }
    }

    pub fn apply_outline_fx(&mut self, action: OutlineFxAction) {
        let batch = action.all_frames;
        if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
            let snap = doc.current_snapshot();
            let fw = doc.project.metadata.frame_width;
            let fh = doc.project.metadata.frame_height;
            let sel_pixels = if action.in_selection_only && !doc.renderer.selection.is_empty() {
                Some(&doc.renderer.selection.pixels)
            } else {
                None
            };
            let frames: Vec<usize> = if batch {
                (0..doc.project.metadata.frame_count as usize).collect()
            } else {
                vec![doc.project.active_frame]
            };
            let mut changed = false;
            for f_idx in frames {
                let (ox, oy) = doc.project.metadata.frame_origin(f_idx);
                if crate::tools::fill::generate_outline_in_region(
                    &mut doc.project.atlas_rgba,
                    ox,
                    oy,
                    fw,
                    fh,
                    action.color,
                    action.thickness,
                    action.corner_style,
                    action.placement,
                    sel_pixels,
                ) {
                    changed = true;
                }
            }
            if changed {
                doc.history.push(snap);
                doc.project.has_unsaved_changes = true;
                doc.project.canvas_dirty = true;
                self.status_key = "app.status.outline_generated";
            }
        }
    }

    pub fn apply_convolution(&mut self, action: ConvolutionAction) {
        let batch = action.all_frames;
        if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
            let snap = doc.current_snapshot();
            let fw = doc.project.metadata.frame_width;
            let fh = doc.project.metadata.frame_height;
            let sel_pixels = if action.in_selection_only && !doc.renderer.selection.is_empty() {
                Some(&doc.renderer.selection.pixels)
            } else {
                None
            };
            let frames: Vec<usize> = if batch {
                (0..doc.project.metadata.frame_count as usize).collect()
            } else {
                vec![doc.project.active_frame]
            };
            let mut changed = false;
            for f_idx in frames {
                let (ox, oy) = doc.project.metadata.frame_origin(f_idx);
                if crate::tools::filters::apply_convolution_matrix(
                    &mut doc.project.atlas_rgba,
                    ox,
                    oy,
                    fw,
                    fh,
                    &action.kernel,
                    action.divisor,
                    action.bias,
                    action.preserve_alpha,
                    sel_pixels,
                ) {
                    changed = true;
                }
            }
            if changed {
                doc.history.push(snap);
                doc.project.has_unsaved_changes = true;
                doc.project.canvas_dirty = true;
                self.status_key = "app.status.filter_applied";
            }
        }
    }

    pub fn apply_palette_remap(&mut self, action: RemapPaletteAction) {
        let batch = action.all_frames;
        if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
            let snap = doc.current_snapshot();
            let fw = doc.project.metadata.frame_width;
            let fh = doc.project.metadata.frame_height;
            let sel_pixels = if action.in_selection_only && !doc.renderer.selection.is_empty() {
                Some(&doc.renderer.selection.pixels)
            } else {
                None
            };
            let frames: Vec<usize> = if batch {
                (0..doc.project.metadata.frame_count as usize).collect()
            } else {
                vec![doc.project.active_frame]
            };
            let palette = doc.project.metadata.palette.colors.clone();
            if palette.is_empty() {
                return;
            }
            let mut changed = false;
            for f_idx in frames {
                let (ox, oy) = doc.project.metadata.frame_origin(f_idx);
                if remap_pixels_to_palette(
                    &mut doc.project.atlas_rgba,
                    ox,
                    oy,
                    fw,
                    fh,
                    &palette,
                    action.dither,
                    action.dither_strength,
                    sel_pixels,
                ) {
                    changed = true;
                }
            }
            if changed {
                doc.history.push(snap);
                doc.project.has_unsaved_changes = true;
                doc.project.canvas_dirty = true;
                self.status_key = "app.status.palette_remapped";
            }
        }
    }

    pub fn apply_palette_sort(&mut self, criterion: PaletteSortCriterion) {
        if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
            doc.record_history();
            sort_palette_colors(&mut doc.project.metadata.palette.colors, criterion);
            let active_s = doc.project.metadata.palette.active_swap_index;
            if let Some(swap) = doc.project.metadata.palette.swaps.get_mut(active_s) {
                swap.colors = doc.project.metadata.palette.colors.clone();
            }
            doc.project.has_unsaved_changes = true;
            self.status_key = "app.status.palette_sorted";
        }
    }

    pub fn apply_nudge(&mut self, dx: i32, dy: i32) {
        let batch = self.tools.batch_mode;
        let wrap = self.tools.wrap_around;
        if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
            doc.record_history();
            let fw = doc.project.metadata.frame_width;
            let fh = doc.project.metadata.frame_height;

            if !doc.renderer.selection.is_empty() {
                let (ox, oy) = doc.project.metadata.frame_origin(doc.project.active_frame);
                nudge_selection_region(
                    &mut doc.project.atlas_rgba,
                    ox,
                    oy,
                    fw,
                    fh,
                    dx,
                    dy,
                    wrap,
                    &doc.renderer.selection.pixels,
                );
                doc.renderer.selection.translate(dx, dy, fw, fh, wrap);
            } else {
                let frames: Vec<usize> = if batch {
                    (0..doc.project.metadata.frame_count as usize).collect()
                } else {
                    vec![doc.project.active_frame]
                };
                for f_idx in frames {
                    let (ox, oy) = doc.project.metadata.frame_origin(f_idx);
                    nudge_region(&mut doc.project.atlas_rgba, ox, oy, fw, fh, dx, dy, wrap);
                }
            }
            doc.project.canvas_dirty = true;
        }
    }

    pub fn copy_selection(&mut self, ctx: Option<&Context>) {
        if let Some(doc) = self.documents.get(self.active_doc_idx) {
            let fw = doc.project.metadata.frame_width;
            let fh = doc.project.metadata.frame_height;
            let (ox, oy) = doc.project.metadata.frame_origin(doc.project.active_frame);

            let mut pixels = Vec::new();
            if !doc.renderer.selection.is_empty() {
                for &(lx, ly) in &doc.renderer.selection.pixels {
                    if lx >= 0 && ly >= 0 && (lx as u32) < fw && (ly as u32) < fh {
                        let ax = ox + lx as u32;
                        let ay = oy + ly as u32;
                        if ax < doc.project.atlas_rgba.width() && ay < doc.project.atlas_rgba.height() {
                            let p = *doc.project.atlas_rgba.get_pixel(ax, ay);
                            if p[3] > 0 {
                                pixels.push(((lx, ly), [p[0], p[1], p[2], p[3]]));
                            }
                        }
                    }
                }
            } else {
                for ly in 0..(fh as i32) {
                    for lx in 0..(fw as i32) {
                        let ax = ox + lx as u32;
                        let ay = oy + ly as u32;
                        if ax < doc.project.atlas_rgba.width() && ay < doc.project.atlas_rgba.height() {
                            let p = *doc.project.atlas_rgba.get_pixel(ax, ay);
                            if p[3] > 0 {
                                pixels.push(((lx, ly), [p[0], p[1], p[2], p[3]]));
                            }
                        }
                    }
                }
            }

            if !pixels.is_empty() {
                self.clipboard = Some(PixelClipboard { pixels: pixels.clone() });
                if let Some(c) = ctx {
                    if let Ok(json) = serde_json::to_string(&pixels) {
                        c.copy_text(format!("{}{}", CLIPBOARD_PREFIX, json));
                    }
                }
                self.status_key = "app.status.copied";
            }
        }
    }

    pub fn cut_selection(&mut self, ctx: Option<&Context>) {
        self.copy_selection(ctx);
        self.delete_selected_pixels();
    }

    pub fn paste_clipboard(&mut self, ctx: Option<&Context>) {
        if let Some(clip) = &self.clipboard {
            if clip.pixels.is_empty() {
                return;
            }
            if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
                doc.record_history();
                let fw = doc.project.metadata.frame_width;
                let fh = doc.project.metadata.frame_height;
                let (ox, oy) = doc.project.metadata.frame_origin(doc.project.active_frame);

                doc.renderer.selection.clear();
                for &((lx, ly), [r, g, b, a]) in &clip.pixels {
                    if lx >= 0 && ly >= 0 && (lx as u32) < fw && (ly as u32) < fh {
                        let ax = ox + lx as u32;
                        let ay = oy + ly as u32;
                        if ax < doc.project.atlas_rgba.width() && ay < doc.project.atlas_rgba.height() {
                            doc.project.atlas_rgba.put_pixel(ax, ay, Rgba([r, g, b, a]));
                            doc.renderer.selection.pixels.insert((lx, ly));
                        }
                    }
                }
                doc.project.sync_active_layer();
                doc.project.canvas_dirty = true;
                doc.project.has_unsaved_changes = true;
                if let Some(c) = ctx {
                    c.request_repaint();
                }
                self.status_key = "app.status.pasted";
            }
        }
    }

    pub fn select_all(&mut self) {
        if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
            let fw = doc.project.metadata.frame_width;
            let fh = doc.project.metadata.frame_height;
            doc.renderer.selection.select_all(fw, fh);
        }
    }

    pub fn invert_selection(&mut self) {
        if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
            let fw = doc.project.metadata.frame_width;
            let fh = doc.project.metadata.frame_height;
            doc.renderer.selection.invert(fw, fh);
        }
    }

    pub fn deselect(&mut self) {
        if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
            doc.renderer.selection.clear();
            self.status_key = "app.status.selection_cleared";
        }
    }

    pub fn expand_selection(&mut self, n: i32, shape: SelectionModifyShape) {
        if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
            if !doc.renderer.selection.is_empty() {
                let fw = doc.project.metadata.frame_width;
                let fh = doc.project.metadata.frame_height;
                doc.renderer.selection.expand(n, shape, fw, fh);
                self.status_key = "app.status.selection_expanded";
            }
        }
    }

    pub fn contract_selection(&mut self, n: i32, shape: SelectionModifyShape) {
        if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
            if !doc.renderer.selection.is_empty() {
                doc.renderer.selection.contract(n, shape);
                self.status_key = "app.status.selection_contracted";
            }
        }
    }

    pub fn border_selection(&mut self, n: i32, shape: SelectionModifyShape) {
        if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
            if !doc.renderer.selection.is_empty() {
                doc.renderer.selection.border(n, shape);
                self.status_key = "app.status.selection_bordered";
            }
        }
    }

    fn delete_selected_pixels(&mut self) {
        let batch = self.tools.batch_mode;
        if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
            if doc.renderer.selection.is_empty() {
                return;
            }
            doc.record_history();
            let fw = doc.project.metadata.frame_width;
            let fh = doc.project.metadata.frame_height;
            let frames: Vec<usize> = if batch {
                (0..doc.project.metadata.frame_count as usize).collect()
            } else {
                vec![doc.project.active_frame]
            };
            for f_idx in frames {
                let (ox, oy) = doc.project.metadata.frame_origin(f_idx);
                for &(lx, ly) in &doc.renderer.selection.pixels {
                    if lx >= 0 && ly >= 0 && (lx as u32) < fw && (ly as u32) < fh {
                        let ax = ox + lx as u32;
                        let ay = oy + ly as u32;
                        if ax < doc.project.atlas_rgba.width()
                            && ay < doc.project.atlas_rgba.height()
                        {
                            doc.project
                                .atlas_rgba
                                .put_pixel(ax, ay, Rgba([0, 0, 0, 0]));
                        }
                    }
                }
            }
            doc.project.canvas_dirty = true;
        }
    }

    pub fn auto_crop_canvas(&mut self) {
        if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
            let snap = doc.current_snapshot();
            if doc.project.auto_crop_canvas() {
                doc.history.push(snap);
                doc.renderer.selection.clear();
                doc.viewport.needs_fit_to_view = true;
                self.status_key = "app.status.canvas_cropped";
            } else {
                self.status_key = "app.status.canvas_already_trimmed";
            }
        }
    }

    pub fn create_custom_brush_from_selection(&mut self) {
        if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
            if doc.renderer.selection.is_empty() {
                return;
            }
            let (min_x, min_y, max_x, max_y) = doc.renderer.selection.bounding_box();
            let w = (max_x - min_x + 1) as u32;
            let h = (max_y - min_y + 1) as u32;
            let (ox, oy) = doc.project.metadata.frame_origin(doc.project.active_frame);
            let mut pixels = Vec::with_capacity(doc.renderer.selection.pixels.len());

            for &(sx, sy) in &doc.renderer.selection.pixels {
                let ax = ox + sx as u32;
                let ay = oy + sy as u32;
                if ax < doc.project.atlas_rgba.width() && ay < doc.project.atlas_rgba.height() {
                    let p = *doc.project.atlas_rgba.get_pixel(ax, ay);
                    pixels.push(((sx - min_x, sy - min_y), p.0));
                }
            }

            if !pixels.is_empty() {
                self.tools.custom_brush = Some(crate::tools::CustomBrush {
                    pixels,
                    width: w,
                    height: h,
                });
                self.tools.active_tool = ToolKind::Pencil;
                doc.renderer.selection.clear();
                self.status_key = "app.status.brush_created";
            }
        }
    }

    fn handle_shortcuts(&mut self, ctx: &Context) {
        if ctx.wants_keyboard_input() {
            return;
        }

        let mut request_repaint = false;
        let mut trigger_new = false;
        let mut trigger_open = false;
        let mut trigger_close = false;
        let mut trigger_save: Option<bool> = None;
        let mut trigger_undo = false;
        let mut trigger_redo = false;
        let mut nudge_delta: Option<(i32, i32)> = None;
        let mut clear_selection = false;
        let mut delete_selection = false;
        let mut trigger_copy = false;
        let mut trigger_cut = false;
        let mut trigger_paste = false;
        let mut trigger_select_all = false;
        let mut trigger_invert_selection = false;
        let mut trigger_dup_frame = false;
        let mut trigger_zoom_in = false;
        let mut trigger_zoom_out = false;
        let mut trigger_zoom_reset = false;
        let mut trigger_fit_canvas = false;
        let mut trigger_view_frame = false;
        let mut trigger_view_atlas = false;
        let mut trigger_toggle_playback = false;
        let mut trigger_replace_color = false;
        let mut trigger_color_adjust = false;
        let mut trigger_custom_brush = false;
        let mut trigger_outline_fx = false;
        let mut trigger_rotate_modal = false;

        ctx.input(|i| {
            if i.key_pressed(egui::Key::F1) {
                self.show_hotkeys_modal = true;
            }
            if i.key_pressed(egui::Key::Space) || i.key_released(egui::Key::Space) {
                request_repaint = true;
            }

            for ev in &i.events {
                match ev {
                    egui::Event::Copy => trigger_copy = true,
                    egui::Event::Cut => trigger_cut = true,
                    egui::Event::Paste(text) => {
                        if let Some(rest) = text.strip_prefix(CLIPBOARD_PREFIX) {
                            if let Ok(pixels) = serde_json::from_str::<Vec<((i32, i32), [u8; 4])>>(rest) {
                                self.clipboard = Some(PixelClipboard { pixels });
                            }
                        }
                        trigger_paste = true;
                    }
                    egui::Event::Key {
                        key,
                        physical_key,
                        pressed: true,
                        modifiers,
                        ..
                    } => {
                        let is = |target: egui::Key| *key == target || *physical_key == Some(target);
                        if modifiers.command && is(egui::Key::C) {
                            trigger_copy = true;
                        }
                        if modifiers.command && is(egui::Key::X) {
                            trigger_cut = true;
                        }
                        if modifiers.command && is(egui::Key::V) {
                            trigger_paste = true;
                        }
                        if modifiers.command && is(egui::Key::A) {
                            trigger_select_all = true;
                        }
                        if modifiers.command && !modifiers.shift && is(egui::Key::Z) {
                            trigger_undo = true;
                        }
                        if (modifiers.command && modifiers.shift && is(egui::Key::Z))
                            || (modifiers.command && is(egui::Key::Y))
                        {
                            trigger_redo = true;
                        }
                        if modifiers.command && !modifiers.shift && is(egui::Key::D) {
                            clear_selection = true;
                        }
                        if modifiers.command && modifiers.shift && is(egui::Key::D) {
                            trigger_dup_frame = true;
                        }
                        if modifiers.command && modifiers.shift && is(egui::Key::I) {
                            trigger_invert_selection = true;
                        }
                        if modifiers.command && is(egui::Key::S) {
                            trigger_save = Some(modifiers.shift);
                        }
                        if modifiers.command && is(egui::Key::N) {
                            trigger_new = true;
                        }
                        if modifiers.command && is(egui::Key::O) {
                            trigger_open = true;
                        }
                        if modifiers.command && is(egui::Key::W) {
                            trigger_close = true;
                        }
                    }
                    _ => {}
                }
            }

            if i.modifiers.command && i.key_pressed(egui::Key::N) {
                trigger_new = true;
            }
            if i.modifiers.command && i.key_pressed(egui::Key::O) {
                trigger_open = true;
            }
            if i.modifiers.command && i.key_pressed(egui::Key::W) {
                trigger_close = true;
            }
            if i.modifiers.command && i.key_pressed(egui::Key::S) {
                trigger_save = Some(i.modifiers.shift);
            }
            if i.modifiers.command && !i.modifiers.shift && i.key_pressed(egui::Key::Z) {
                trigger_undo = true;
            }
            if (i.modifiers.command && i.modifiers.shift && i.key_pressed(egui::Key::Z))
                || (i.modifiers.command && i.key_pressed(egui::Key::Y))
            {
                trigger_redo = true;
            }
            if i.modifiers.command && i.key_pressed(egui::Key::D) && !i.modifiers.shift {
                clear_selection = true;
            }
            if i.modifiers.command && i.modifiers.shift && i.key_pressed(egui::Key::D) {
                trigger_dup_frame = true;
            }
            if i.modifiers.command && i.key_pressed(egui::Key::A) {
                trigger_select_all = true;
            }
            if i.modifiers.command && i.modifiers.shift && i.key_pressed(egui::Key::I) {
                trigger_invert_selection = true;
            }
            if i.modifiers.command && i.key_pressed(egui::Key::C) {
                trigger_copy = true;
            }
            if i.modifiers.command && i.key_pressed(egui::Key::X) {
                trigger_cut = true;
            }
            if i.modifiers.command && i.key_pressed(egui::Key::V) {
                trigger_paste = true;
            }
            if i.modifiers.command && (i.key_pressed(egui::Key::Plus) || i.key_pressed(egui::Key::Equals)) {
                trigger_zoom_in = true;
            }
            if i.modifiers.command && i.key_pressed(egui::Key::Minus) {
                trigger_zoom_out = true;
            }
            if i.modifiers.command && i.key_pressed(egui::Key::Num0) {
                trigger_zoom_reset = true;
            }
            if i.modifiers.command && i.key_pressed(egui::Key::Space) {
                trigger_toggle_playback = true;
            }
            if !i.modifiers.command && !i.modifiers.alt && i.modifiers.shift && i.key_pressed(egui::Key::R) {
                trigger_replace_color = true;
            }
            if i.modifiers.command && !i.modifiers.alt && i.modifiers.shift && i.key_pressed(egui::Key::R) {
                trigger_rotate_modal = true;
            }
            if !i.modifiers.command && !i.modifiers.alt && i.modifiers.shift && i.key_pressed(egui::Key::O) {
                trigger_outline_fx = true;
            }
            if i.modifiers.command && !i.modifiers.shift && !i.modifiers.alt && i.key_pressed(egui::Key::U) {
                trigger_color_adjust = true;
            }
            if i.modifiers.command && !i.modifiers.shift && !i.modifiers.alt && i.key_pressed(egui::Key::B) {
                trigger_custom_brush = true;
            }

            let is_single = |target: egui::Key| -> bool {
                i.key_pressed(target)
                    || i.events.iter().any(|e| match e {
                        egui::Event::Key {
                            key: k,
                            physical_key,
                            pressed: true,
                            modifiers,
                            ..
                        } => !modifiers.any() && (*k == target || *physical_key == Some(target)),
                        _ => false,
                    })
            };

            if !i.modifiers.any() {
                if is_single(egui::Key::Escape) {
                    clear_selection = true;
                } else if is_single(egui::Key::Delete) || is_single(egui::Key::Backspace) {
                    delete_selection = true;
                } else if is_single(egui::Key::OpenBracket) {
                    self.tools.brush_size = self.tools.brush_size.saturating_sub(1).max(1);
                    self.tools.custom_brush = None;
                    request_repaint = true;
                } else if is_single(egui::Key::CloseBracket) {
                    self.tools.brush_size = (self.tools.brush_size + 1).min(32);
                    self.tools.custom_brush = None;
                    request_repaint = true;
                } else if is_single(egui::Key::F) {
                    trigger_fit_canvas = true;
                } else if is_single(egui::Key::Num1) {
                    trigger_view_frame = true;
                } else if is_single(egui::Key::Num2) {
                    trigger_view_atlas = true;
                } else if is_single(egui::Key::Enter) {
                    trigger_toggle_playback = true;
                } else if is_single(egui::Key::ArrowLeft) {
                    nudge_delta = Some((-1, 0));
                } else if is_single(egui::Key::ArrowRight) {
                    nudge_delta = Some((1, 0));
                } else if is_single(egui::Key::ArrowUp) {
                    nudge_delta = Some((0, -1));
                } else if is_single(egui::Key::ArrowDown) {
                    nudge_delta = Some((0, 1));
                } else if is_single(egui::Key::X) {
                    self.tools.swap_fg_bg();
                } else if is_single(egui::Key::B) {
                    self.tools.active_tool = ToolKind::Pencil;
                } else if is_single(egui::Key::E) {
                    self.tools.active_tool = ToolKind::Eraser;
                } else if is_single(egui::Key::I) {
                    self.tools.active_tool = ToolKind::Eyedropper;
                } else if is_single(egui::Key::G) {
                    self.tools.active_tool = if self.tools.active_tool == ToolKind::FloodFill {
                        ToolKind::Gradient
                    } else {
                        ToolKind::FloodFill
                    };
                } else if is_single(egui::Key::L) {
                    self.tools.active_tool = ToolKind::Line;
                } else if is_single(egui::Key::C) {
                    self.tools.active_tool = ToolKind::Curve;
                } else if is_single(egui::Key::U) {
                    self.tools.active_tool = ToolKind::Rectangle;
                } else if is_single(egui::Key::O) {
                    self.tools.active_tool = ToolKind::Ellipse;
                } else if is_single(egui::Key::D) {
                    self.tools.active_tool = ToolKind::DitherBrush;
                } else if is_single(egui::Key::S) {
                    self.tools.active_tool = ToolKind::Shading;
                } else if is_single(egui::Key::M) {
                    self.tools.active_tool = ToolKind::RectSelect;
                } else if is_single(egui::Key::Q) {
                    self.tools.active_tool = ToolKind::Lasso;
                } else if is_single(egui::Key::W) {
                    self.tools.active_tool = ToolKind::MagicWand;
                } else if is_single(egui::Key::P) {
                    self.tools.active_tool = ToolKind::PivotPoint;
                }
            }
        });

        if request_repaint {
            ctx.request_repaint();
        }
        if trigger_new {
            self.wizard.open = true;
        }
        if trigger_open {
            self.io.trigger_open_dialog(ctx.clone());
        }
        if trigger_close && !self.documents.is_empty() {
            self.request_close_doc(self.active_doc_idx);
        }
        if let Some(force_as) = trigger_save {
            if let Some(doc) = self.documents.get(self.active_doc_idx) {
                self.io.trigger_save(&doc.project, force_as, ctx.clone());
            }
        }
        if trigger_undo {
            self.perform_undo();
        }
        if trigger_redo {
            self.perform_redo();
        }
        if trigger_copy {
            self.copy_selection(Some(ctx));
        }
        if trigger_cut {
            self.cut_selection(Some(ctx));
        }
        if trigger_paste {
            self.paste_clipboard(Some(ctx));
        }
        if trigger_select_all {
            self.select_all();
        }
        if trigger_invert_selection {
            self.invert_selection();
        }
        if trigger_dup_frame {
            if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
                doc.record_history();
                doc.project.duplicate_frame_at(doc.project.active_frame);
            }
        }
        if trigger_zoom_in {
            if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
                doc.viewport.zoom_in();
            }
        }
        if trigger_zoom_out {
            if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
                doc.viewport.zoom_out();
            }
        }
        if trigger_zoom_reset {
            if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
                doc.viewport.set_zoom(1.0);
                doc.viewport.pan_offset = egui::Vec2::ZERO;
            }
        }
        if trigger_fit_canvas {
            if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
                doc.viewport.needs_fit_to_view = true;
            }
        }
        if trigger_view_frame {
            if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
                doc.viewport.view_mode = crate::canvas::viewport::CanvasViewMode::Frame;
            }
        }
        if trigger_view_atlas {
            if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
                doc.viewport.view_mode = crate::canvas::viewport::CanvasViewMode::Atlas;
            }
        }
        if trigger_toggle_playback {
            if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
                doc.preview.toggle_play(&mut doc.project);
            }
        }
        if clear_selection {
            if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
                if !doc.renderer.selection.is_empty() {
                    doc.renderer.selection.clear();
                } else if self.tools.custom_brush.is_some() {
                    self.tools.custom_brush = None;
                }
                doc.renderer.curve_state = None;
            }
        }
        if delete_selection {
            self.delete_selected_pixels();
        }
        if let Some((dx, dy)) = nudge_delta {
            self.apply_nudge(dx, dy);
        }
        if trigger_replace_color {
            if let Some(doc) = self.documents.get(self.active_doc_idx) {
                self.replace_color_modal.open_with(
                    self.tools.fg_color,
                    self.tools.bg_color,
                    !doc.renderer.selection.is_empty(),
                );
            }
        }
        if trigger_outline_fx {
            if let Some(doc) = self.documents.get(self.active_doc_idx) {
                self.outline_fx_modal.open_with(
                    self.tools.fg_color,
                    !doc.renderer.selection.is_empty(),
                );
            }
        }
        if trigger_rotate_modal {
            if let Some(doc) = self.documents.get(self.active_doc_idx) {
                self.rotate_modal
                    .open_with(!doc.renderer.selection.is_empty());
            }
        }
        if trigger_color_adjust {
            if let Some(doc) = self.documents.get(self.active_doc_idx) {
                self.color_adjust_modal
                    .open_with(!doc.renderer.selection.is_empty());
            }
        }
        if trigger_custom_brush {
            self.create_custom_brush_from_selection();
        }
    }

    fn process_io_events(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        let dropped = ctx.input(|i| i.raw.dropped_files.clone());
        if !dropped.is_empty() {
            self.io.handle_dropped_files(&dropped);
        }

        for event in self.io.drain_events() {
            match event {
                IoEvent::ProjectLoaded(loaded) => {
                    self.open_project_doc(loaded);
                    self.status_key = "app.status.project_loaded";
                }
                IoEvent::ProjectSaved(maybe_path) => {
                    if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
                        if let Some(p) = maybe_path.clone() {
                            doc.project.file_path = Some(p.clone());
                            self.settings.add_recent(p);
                        }
                        doc.project.has_unsaved_changes = false;
                    }
                    AutosaveService::clear_recovery(frame.storage_mut());
                    self.status_key = "app.status.project_saved";
                    if let Some(pending) = self.pending_after_save.take() {
                        match pending {
                            PendingAfterSave::CloseDoc(idx) => {
                                self.close_doc_now(idx);
                            }
                            PendingAfterSave::CloseApp => {
                                if let Some(next_dirty) = self
                                    .documents
                                    .iter()
                                    .position(|d| d.project.has_unsaved_changes)
                                {
                                    self.active_doc_idx = next_dirty;
                                    self.pending_close_doc_idx = Some(next_dirty);
                                    self.pending_after_save = Some(PendingAfterSave::CloseApp);
                                } else {
                                    self.confirmed_exit = true;
                                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                                }
                            }
                        }
                    }
                }
                IoEvent::SingleFrameLoaded { frame_index, rgba } => {
                    if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
                        doc.record_history();
                        if doc.project.replace_single_frame_rgba(frame_index, &rgba) {
                            self.status_key = "app.status.frame_imported";
                        }
                    }
                }
                IoEvent::SingleFrameExported => {
                    self.status_key = "app.status.frame_exported";
                }
                IoEvent::FrameSequenceLoaded(loaded) => {
                    self.open_project_doc(loaded);
                    self.status_key = "app.status.seq_imported";
                }
                IoEvent::FrameSequenceExported => {
                    self.status_key = "app.status.seq_exported";
                }
                IoEvent::SketchLoaded {
                    rgba,
                    path_str,
                    base64_png,
                } => {
                    if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
                        let b64_opt = if doc.project.metadata.sketch.embed_in_json {
                            Some(base64_png)
                        } else {
                            None
                        };
                        doc.project
                            .set_sketch_image(Some(rgba), path_str, b64_opt);
                        self.status_key = "app.status.sketch_loaded";
                    }
                }
                IoEvent::SlicerSheetLoaded { name, rgba } => {
                    self.slicer_modal.open_with_sheet(name, rgba);
                }
                IoEvent::PaletteLoaded(colors) => {
                    if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
                        doc.record_history();
                        doc.project.metadata.palette.colors = colors.clone();
                        for swap in &mut doc.project.metadata.palette.swaps {
                            swap.colors = colors.clone();
                        }
                        self.status_key = "app.status.palette_loaded";
                    }
                }
                IoEvent::PaletteSaved => {
                    self.status_key = "app.status.palette_saved";
                }
                IoEvent::ExportedPngSaved => {
                    self.status_key = "app.status.exported_png";
                }
                IoEvent::ExportedGifSaved => {
                    self.status_key = "app.status.exported_gif";
                }
                IoEvent::Error(_err) => {
                    self.status_key = "app.status.load_error";
                }
            }
        }
    }
}

impl eframe::App for OniAnvilApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        if let Ok(json) = serde_json::to_string(&self.settings) {
            storage.set_string("onianvil_settings", json);
        }
    }

    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        if ctx.input(|i| i.viewport().close_requested()) && !self.confirmed_exit {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            let dirty_opt = self
                .documents
                .iter()
                .position(|d| d.project.has_unsaved_changes);
            if let Some(dirty_idx) = dirty_opt {
                self.active_doc_idx = dirty_idx;
                self.pending_close_doc_idx = Some(dirty_idx);
                self.pending_after_save = Some(PendingAfterSave::CloseApp);
            } else {
                self.show_confirm_exit_modal = true;
            }
        }

        if !self.documents.is_empty() && self.active_doc_idx >= self.documents.len() {
            self.active_doc_idx = self.documents.len() - 1;
        }

        let pending_fonts = crate::web::take_pending_fonts();
        if !pending_fonts.is_empty() {
            for (name, font_bytes) in pending_fonts {
                crate::web::set_installed_font(name, font_bytes);
            }
            crate::theme::setup_custom_fonts(ctx);
            ctx.request_repaint();
        }

        if crate::web::check_and_fetch_web_fonts(self.i18n.language()) {
            ctx.request_repaint();
        }

        let doc_suffix = if let Some(doc) = self.documents.get(self.active_doc_idx) {
            let dirty = if doc.project.has_unsaved_changes {
                " *"
            } else {
                ""
            };
            format!(" - {}{}", doc.project.metadata.project_name, dirty)
        } else {
            String::new()
        };
        let window_title = format!("OniAnvil v{}{}", env!("CARGO_PKG_VERSION"), doc_suffix);
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(window_title));

        self.process_io_events(ctx, frame);
        self.handle_shortcuts(ctx);

        let dt = ctx.input(|i| i.stable_dt);
        if let Some(doc) = self.documents.get(self.active_doc_idx) {
            if self.autosave.tick(dt, &doc.project, frame.storage_mut()) {
                self.status_key = "app.status.autosaved";
            }
        }

        if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
            doc.preview.tick(ctx, &mut doc.project);
            doc.renderer.active_pivot_target = doc.preview.active_pivot_target;
            let _ = doc.renderer.ensure_texture(ctx, &mut doc.project);
        }

        let update_info = self.update_checker.available_update();
        let status_text = self.i18n.tr(self.status_key).to_string();

        let active_doc_tuple = self.documents.get_mut(self.active_doc_idx).map(|doc| {
            (&mut doc.project, &mut doc.viewport, &doc.history)
        });

        let top_actions = show_top_bars(
            ctx,
            &mut self.i18n,
            active_doc_tuple,
            &mut self.tools,
            &mut self.show_right_dock,
            &mut self.show_bottom_dock,
            update_info.as_ref(),
            &mut self.app_theme,
        );

        // Row 3: Project Tabs Bar immediately below contextual options bar (when documents are open)
        if !self.documents.is_empty() {
            TopBottomPanel::top("onianvil_project_tabs_bar")
                .exact_height(28.0)
                .frame(
                    egui::Frame::NONE
                        .fill(ctx.style().visuals.panel_fill)
                        .stroke(ctx.style().visuals.widgets.noninteractive.bg_stroke),
                )
                .show(ctx, |ui| {
                    ui.horizontal_centered(|ui| {
                        ui.spacing_mut().item_spacing.x = 4.0;
                        let mut switch_to: Option<usize> = None;
                        let mut close_idx: Option<usize> = None;

                        egui::ScrollArea::horizontal()
                            .id_salt("onianvil_tabs_scroll")
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                ui.horizontal_centered(|ui| {
                                    for idx in 0..self.documents.len() {
                                        let is_active = idx == self.active_doc_idx;
                                        let doc = &mut self.documents[idx];
                                        let is_dirty = doc.project.has_unsaved_changes;

                                        let bg_color = if is_active {
                                            ui.visuals().widgets.active.bg_fill
                                        } else {
                                            ui.visuals().widgets.inactive.bg_fill
                                        };

                                        let border_color = if is_active {
                                            ui.visuals().widgets.active.bg_stroke.color
                                        } else {
                                            ui.visuals().widgets.inactive.bg_stroke.color
                                        };

                                        egui::Frame::NONE
                                            .fill(bg_color)
                                            .stroke(egui::Stroke::new(1.0, border_color))
                                            .corner_radius(4)
                                            .inner_margin(egui::Margin::symmetric(8, 2))
                                            .show(ui, |ui| {
                                                ui.horizontal_centered(|ui| {
                                                    ui.spacing_mut().item_spacing.x = 4.0;

                                                    if is_dirty {
                                                        ui.label(
                                                            egui::RichText::new("●")
                                                                .size(10.0)
                                                                .color(Color32::from_rgb(230, 160, 80)),
                                                        )
                                                        .on_hover_text(self.i18n.tr("status.unsaved"));
                                                    }

                                                    if doc.renaming_tab {
                                                        let resp = ui.add(
                                                            egui::TextEdit::singleline(
                                                                &mut doc.project.metadata.project_name,
                                                            )
                                                            .desired_width(120.0),
                                                        );
                                                        if resp.lost_focus()
                                                            || ui.input(|i| i.key_pressed(egui::Key::Enter))
                                                        {
                                                            doc.renaming_tab = false;
                                                        }
                                                    } else {
                                                        let metrics = format!(
                                                            "{}×{} ({}f)",
                                                            doc.project.metadata.frame_width,
                                                            doc.project.metadata.frame_height,
                                                            doc.project.metadata.frame_count
                                                        );
                                                        let tab_label = format!(
                                                            "{}  {}",
                                                            doc.project.metadata.project_name, metrics
                                                        );
                                                        let resp = ui.selectable_label(
                                                            is_active,
                                                            if is_active {
                                                                egui::RichText::new(tab_label).strong()
                                                            } else {
                                                                egui::RichText::new(tab_label).weak()
                                                            },
                                                        );
                                                        if resp.clicked() {
                                                            switch_to = Some(idx);
                                                        }
                                                        if resp
                                                            .on_hover_text(self.i18n.tr("tabs.rename_hint"))
                                                            .double_clicked()
                                                        {
                                                            doc.renaming_tab = true;
                                                        }
                                                    }

                                                    let close_resp = ui
                                                        .small_button(egui::RichText::new(FA_CLOSE).size(10.0))
                                                        .on_hover_text(self.i18n.tr("tabs.close"));
                                                    if close_resp.clicked() {
                                                        close_idx = Some(idx);
                                                    }
                                                });
                                            });
                                    }

                                    if ui
                                        .button(egui::RichText::new(FA_PLUS).strong())
                                        .on_hover_text(self.i18n.tr("tabs.new"))
                                        .clicked()
                                    {
                                        self.wizard.open = true;
                                    }
                                });
                            });

                        if let Some(target) = switch_to {
                            self.active_doc_idx = target;
                        }
                        if let Some(target) = close_idx {
                            self.request_close_doc(target);
                        }
                    });
                });
        }

        TopBottomPanel::bottom("onianvil_status_bar")
            .exact_height(22.0)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.label(egui::RichText::new(&status_text).small());

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if let Some(doc) = self.documents.get(self.active_doc_idx) {
                            if doc.project.has_unsaved_changes {
                                ui.label(
                                    egui::RichText::new(self.i18n.tr("status.unsaved"))
                                        .color(Color32::from_rgb(230, 160, 80))
                                        .small(),
                                );
                            } else {
                                ui.label(
                                    egui::RichText::new(self.i18n.tr("status.saved"))
                                        .color(Color32::from_rgb(110, 180, 110))
                                        .small(),
                                );
                            }
                            ui.separator();

                            let (aw, ah) = doc.project.metadata.atlas_dimensions();
                            ui.label(
                                egui::RichText::new(format!("{}: {}×{} px", self.i18n.tr("status.atlas"), aw, ah))
                                    .weak()
                                    .small(),
                            );
                            ui.separator();

                            let fw = doc.project.metadata.frame_width;
                            let fh = doc.project.metadata.frame_height;
                            let f_idx = doc.project.active_frame + 1;
                            let f_total = doc.project.metadata.frame_count;
                            ui.label(
                                egui::RichText::new(format!(
                                    "{}: {}/{} ({}×{})",
                                    self.i18n.tr("status.frame"), f_idx, f_total, fw, fh
                                ))
                                .weak()
                                .small(),
                            );
                            ui.separator();

                            ui.label(
                                egui::RichText::new(format!("{:.0}%", doc.viewport.zoom * 100.0))
                                    .weak()
                                    .small(),
                            );
                            ui.separator();

                            let cursor_str = if let Some((hx, hy)) = doc.viewport.hover_pixel {
                                format!("X: {}, Y: {}", hx, hy)
                            } else {
                                String::from("X: —, Y: —")
                            };
                            ui.label(
                                egui::RichText::new(cursor_str)
                                    .weak()
                                    .small(),
                            );

                            if !doc.renderer.selection.is_empty() {
                                ui.separator();
                                ui.label(
                                    egui::RichText::new(format!(
                                        "{}: {} px",
                                        self.i18n.tr("status.selection"),
                                        doc.renderer.selection.pixels.len()
                                    ))
                                    .color(Color32::from_rgb(100, 180, 240))
                                    .small(),
                                );
                            }
                        }
                    });
                });
            });

        if top_actions.open_new_project_wizard {
            self.wizard.open = true;
        }
        if top_actions.trigger_open_project {
            self.io.trigger_open_dialog(ctx.clone());
        }
        if top_actions.trigger_close_project && !self.documents.is_empty() {
            self.request_close_doc(self.active_doc_idx);
        }
        if top_actions.trigger_open_slicer {
            self.io.trigger_load_slicer_sheet_dialog(ctx.clone());
        }
        if top_actions.trigger_import_frame_sequence {
            self.io.trigger_import_frame_sequence_dialog(ctx.clone());
        }
        if top_actions.trigger_save_project {
            if let Some(doc) = self.documents.get(self.active_doc_idx) {
                self.io.trigger_save(&doc.project, false, ctx.clone());
            }
        }
        if top_actions.trigger_save_project_as {
            if let Some(doc) = self.documents.get(self.active_doc_idx) {
                self.io.trigger_save(&doc.project, true, ctx.clone());
            }
        }
        if top_actions.open_export_png_modal {
            self.export_png_modal.open = true;
        }
        if top_actions.trigger_export_frame_sequence {
            if let Some(doc) = self.documents.get(self.active_doc_idx) {
                self.io
                    .trigger_export_frame_sequence_dialog(&doc.project, ctx.clone());
            }
        }
        if top_actions.open_export_gif_modal {
            self.export_gif_modal.open = true;
        }
        if top_actions.trigger_undo {
            self.perform_undo();
        }
        if top_actions.trigger_redo {
            self.perform_redo();
        }
        if top_actions.trigger_copy {
            self.copy_selection(Some(ctx));
        }
        if top_actions.trigger_cut {
            self.cut_selection(Some(ctx));
        }
        if top_actions.trigger_paste {
            self.paste_clipboard(Some(ctx));
        }
        if top_actions.trigger_select_all {
            self.select_all();
        }
        if top_actions.trigger_invert_selection {
            self.invert_selection();
        }
        if top_actions.trigger_deselect {
            self.deselect();
        }
        if top_actions.trigger_expand_selection {
            self.expand_selection(1, SelectionModifyShape::Circle);
        }
        if top_actions.trigger_contract_selection {
            self.contract_selection(1, SelectionModifyShape::Circle);
        }
        if top_actions.trigger_border_selection {
            self.border_selection(1, SelectionModifyShape::Circle);
        }
        if top_actions.open_modify_selection_modal {
            self.modify_selection_modal.open = true;
        }
        if top_actions.trigger_export_sprite_json {
            if let Some(doc) = self.documents.get(self.active_doc_idx) {
                self.io
                    .trigger_export_sprite_sheet_json_dialog(&doc.project, ctx.clone());
            }
        }
        if top_actions.open_export_packed_atlas_modal {
            if let Some(doc) = self.documents.get(self.active_doc_idx) {
                self.pack_atlas_modal.open_for(&doc.project);
            }
        }
        if top_actions.trigger_flip_h {
            self.apply_flip_horizontal();
        }
        if top_actions.trigger_flip_v {
            self.apply_flip_vertical();
        }
        if top_actions.trigger_rotate_90 {
            self.apply_rotate_90_cw();
        }
        if top_actions.open_rotate_modal {
            if let Some(doc) = self.documents.get(self.active_doc_idx) {
                self.rotate_modal
                    .open_with(!doc.renderer.selection.is_empty());
            }
        }
        if top_actions.trigger_auto_outline {
            self.apply_auto_outline();
        }
        if top_actions.open_outline_fx_modal {
            if let Some(doc) = self.documents.get(self.active_doc_idx) {
                self.outline_fx_modal.open_with(
                    self.tools.fg_color,
                    !doc.renderer.selection.is_empty(),
                );
            }
        }
        if top_actions.trigger_replace_color {
            if let Some(doc) = self.documents.get(self.active_doc_idx) {
                self.replace_color_modal.open_with(
                    self.tools.fg_color,
                    self.tools.bg_color,
                    !doc.renderer.selection.is_empty(),
                );
            }
        }
        if top_actions.trigger_color_adjust {
            if let Some(doc) = self.documents.get(self.active_doc_idx) {
                self.color_adjust_modal
                    .open_with(!doc.renderer.selection.is_empty());
            }
        }
        if top_actions.open_convolution_modal {
            if let Some(doc) = self.documents.get(self.active_doc_idx) {
                self.convolution_modal
                    .open_with(!doc.renderer.selection.is_empty());
            }
        }
        if top_actions.open_remap_palette_modal {
            if let Some(doc) = self.documents.get(self.active_doc_idx) {
                self.remap_palette_modal
                    .open_with(!doc.renderer.selection.is_empty(), doc.project.metadata.palette.colors.len());
            }
        }
        if let Some(criterion) = top_actions.trigger_sort_palette {
            self.apply_palette_sort(criterion);
        }
        if top_actions.trigger_custom_brush {
            self.create_custom_brush_from_selection();
        }
        if top_actions.trigger_auto_crop {
            self.auto_crop_canvas();
        }
        if top_actions.open_regrid_modal {
            if let Some(doc) = self.documents.get(self.active_doc_idx) {
                self.regrid_modal.open_for(&doc.project);
            }
        }
        if top_actions.open_resize9_modal {
            if let Some(doc) = self.documents.get(self.active_doc_idx) {
                self.resize9_modal.open_for(&doc.project);
            }
        }
        if top_actions.open_about_modal {
            self.show_about_modal = true;
        }
        if top_actions.open_hotkeys_modal {
            self.show_hotkeys_modal = true;
        }
        if top_actions.open_demo_project {
            match IoManager::load_demo_cat_project() {
                Ok(demo) => {
                    self.open_project_doc(demo);
                    self.status_key = "app.status.demo_loaded";
                }
                Err(e) => {
                    log::error!("Failed to load demo project: {}", e);
                }
            }
        }

        if let Some(lang) = top_actions.language_changed {
            self.settings.language = lang;
            let _ = self.settings.save();
            crate::web::check_and_fetch_web_fonts(lang);
            ctx.request_repaint();
        }
        if let Some(th) = top_actions.theme_changed {
            self.settings.theme = th;
            self.app_theme = th;
            let _ = self.settings.save();
        }

        if self.documents.is_empty() {
            if self.logo_texture.is_none() {
                self.logo_texture = crate::theme::load_app_logo_texture(ctx);
            }
            egui::CentralPanel::default().show(ctx, |ui| {
                let start_action = show_start_screen(
                    ui,
                    &self.i18n,
                    &self.settings.recent_projects,
                    &AppSettings::settings_file_path(),
                    self.logo_texture.as_ref(),
                );
                match start_action {
                    StartScreenAction::None => {}
                    StartScreenAction::NewProject => {
                        self.wizard.open = true;
                    }
                    StartScreenAction::OpenProject => {
                        self.io.trigger_open_dialog(ctx.clone());
                    }
                    StartScreenAction::OpenSlicer => {
                        self.io.trigger_load_slicer_sheet_dialog(ctx.clone());
                    }
                    StartScreenAction::OpenDemo => {
                        match IoManager::load_demo_cat_project() {
                            Ok(demo) => {
                                self.open_project_doc(demo);
                                self.status_key = "app.status.demo_loaded";
                            }
                            Err(e) => {
                                log::error!("Failed to load demo project: {}", e);
                            }
                        }
                    }
                    StartScreenAction::OpenRecent(_path) => {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            match IoManager::load_from_native_path(&_path) {
                                Ok(loaded) => {
                                    self.open_project_doc(loaded);
                                    self.status_key = "app.status.project_loaded";
                                }
                                Err(_) => {
                                    self.settings.remove_recent(&_path);
                                    self.status_key = "app.status.load_error";
                                }
                            }
                        }
                    }
                    StartScreenAction::RemoveRecent(path) => {
                        self.pending_delete_recent = Some(path);
                    }
                }
            });
        } else {
            let old_cols = self.tools.two_columns;
            show_left_dock(ctx, &self.i18n, &mut self.tools, Some(&mut self.settings));
            if self.tools.two_columns != old_cols {
                self.settings.tools_two_columns = self.tools.two_columns;
                let _ = self.settings.save();
            }

            let replace_color_pick_target = if self.replace_color_modal.open {
                self.replace_color_modal.picking
            } else {
                None
            };
            let prev_fg_for_picking = self.tools.fg_color;
            let prev_bg_for_picking = self.tools.bg_color;
            let saved_tool_for_picking = self.tools.active_tool;
            if replace_color_pick_target.is_some() {
                self.tools.active_tool = crate::tools::ToolKind::Eyedropper;
            }

            if self.show_right_dock {
                if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
                    let right_actions = show_right_dock(
                        ctx,
                        &self.i18n,
                        &mut doc.project,
                        &mut self.tools,
                        &mut doc.preview,
                        &doc.renderer,
                        Some(&mut self.settings),
                    );
                    if right_actions.trigger_load_sketch {
                        self.io.trigger_load_sketch_dialog(ctx.clone());
                    }
                    if right_actions.trigger_clear_sketch {
                        doc.project.set_sketch_image(None, None, None);
                    }
                    if right_actions.trigger_bake_sketch {
                        let snap = doc.current_snapshot();
                        let lock_alpha = self.tools.lock_alpha;
                        let batch = self.tools.batch_mode;
                        let sel = doc.renderer.selection.clone();
                        let frames: Vec<usize> = if batch {
                            (0..doc.project.metadata.frame_count as usize).collect()
                        } else {
                            vec![doc.project.active_frame]
                        };
                        let mut any_baked = false;
                        for f_idx in frames {
                            if doc.project.bake_sketch_to_frame(f_idx, lock_alpha, &sel) {
                                any_baked = true;
                            }
                        }
                        if any_baked {
                            doc.history.push(snap);
                            self.status_key = "app.status.sketch_baked";
                        }
                    }
                    if right_actions.trigger_import_palette {
                        self.io.trigger_import_palette_dialog(ctx.clone());
                    }
                    if right_actions.trigger_export_palette_png {
                        self.io.trigger_export_palette_dialog(
                            &doc.project.metadata.palette.colors,
                            ctx.clone(),
                        );
                    }
                    if right_actions.trigger_export_palette_hex {
                        self.io.trigger_export_palette_hex_dialog(
                            &doc.project.metadata.palette.colors,
                            ctx.clone(),
                        );
                    }
                    if right_actions.trigger_export_palette_gpl {
                        self.io.trigger_export_palette_gpl_dialog(
                            &doc.project.metadata.project_name,
                            &doc.project.metadata.palette.colors,
                            ctx.clone(),
                        );
                    }
                    if let Some(criterion) = right_actions.trigger_sort_palette {
                        doc.record_history();
                        sort_palette_colors(&mut doc.project.metadata.palette.colors, criterion);
                        let active_s = doc.project.metadata.palette.active_swap_index;
                        if let Some(swap) = doc.project.metadata.palette.swaps.get_mut(active_s) {
                            swap.colors = doc.project.metadata.palette.colors.clone();
                        }
                        doc.project.has_unsaved_changes = true;
                        self.status_key = "app.status.palette_sorted";
                    }
                    if right_actions.trigger_remap_palette_modal {
                        self.remap_palette_modal
                            .open_with(!doc.renderer.selection.is_empty(), doc.project.metadata.palette.colors.len());
                    }
                    if let Some(del_skin) = right_actions.request_delete_skin {
                        self.pending_delete_skin = Some(del_skin);
                    }
                    if let Some(slot_info) = right_actions.request_delete_palette_slot {
                        self.pending_delete_palette_slot = Some(slot_info);
                    }
                    if let Some(pt_info) = right_actions.request_delete_pivot_point {
                        self.pending_delete_pivot_point = Some(pt_info);
                    }
                }
            }

            if self.show_bottom_dock {
                if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
                    let bottom_actions = show_bottom_dock(
                        ctx,
                        &self.i18n,
                        &mut doc.project,
                        &mut doc.history,
                        &mut doc.preview,
                        &doc.renderer,
                    );
                    if let Some(f_idx) = bottom_actions.trigger_import_single_frame {
                        self.io.trigger_import_single_frame_dialog(f_idx, ctx.clone());
                    }
                    if let Some(f_idx) = bottom_actions.trigger_export_single_frame {
                        let frame_img = doc.project.export_single_frame_rgba(f_idx);
                        let default_name = format!(
                            "{}{}.png",
                            doc.project.metadata.project_name,
                            f_idx + 1
                        );
                        self.io.trigger_export_single_frame_dialog(
                            &default_name,
                            frame_img,
                            ctx.clone(),
                        );
                    }
                    if let Some(del_idx) = bottom_actions.request_delete_frame {
                        self.pending_delete_frame = Some(del_idx);
                    }
                    if let Some(tag_info) = bottom_actions.request_delete_tag {
                        self.pending_delete_tag = Some(tag_info);
                    }
                    if let Some(layer_info) = bottom_actions.request_delete_layer {
                        self.pending_delete_layer = Some(layer_info);
                    }
                }
            }

            if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
                show_center_workspace(
                    ctx,
                    &self.i18n,
                    &mut doc.project,
                    &mut doc.viewport,
                    &mut self.tools,
                    &mut doc.history,
                    &mut doc.renderer,
                );
            }

            if let Some(target) = replace_color_pick_target {
                self.tools.active_tool = saved_tool_for_picking;
                let mut picked_color = None;

                if let Some(doc) = self.documents.get(self.active_doc_idx) {
                    if let Some((lx, ly)) = doc.viewport.hover_pixel {
                        if ctx.input(|i| i.pointer.primary_clicked()) {
                            let (ox, oy) = doc.project.metadata.frame_origin(doc.project.active_frame);
                            let ax = ox + lx as u32;
                            let ay = oy + ly as u32;
                            let src_rgba = if let Some((b_idx, ref orig_rgba)) = self.modal_preview_backup {
                                if b_idx == self.active_doc_idx {
                                    orig_rgba
                                } else {
                                    &doc.project.atlas_rgba
                                }
                            } else {
                                &doc.project.atlas_rgba
                            };
                            if ax < src_rgba.width() && ay < src_rgba.height() {
                                let p = *src_rgba.get_pixel(ax, ay);
                                picked_color = Some([p[0], p[1], p[2], p[3]]);
                            }
                        }
                    }
                }

                if picked_color.is_none() {
                    if self.tools.fg_color != prev_fg_for_picking {
                        picked_color = Some(self.tools.fg_color);
                    } else if self.tools.bg_color != prev_bg_for_picking {
                        picked_color = Some(self.tools.bg_color);
                    }
                }

                if let Some(col) = picked_color {
                    match target {
                        crate::ui::modals::ReplaceColorPickTarget::From => {
                            self.replace_color_modal.from_color = col;
                        }
                        crate::ui::modals::ReplaceColorPickTarget::To => {
                            self.replace_color_modal.to_color = col;
                        }
                    }
                    self.replace_color_modal.picking = None;
                }
            }
        }

        // Modals
        if let Some(new_meta) = self.wizard.show(ctx, &self.i18n) {
            self.open_project_doc(Project::new(new_meta));
            self.status_key = "app.status.project_created";
        }

        let regrid_res = if let Some(doc) = self.documents.get(self.active_doc_idx) {
            self.regrid_modal.show(ctx, &self.i18n, &doc.project)
        } else {
            None
        };
        if let Some(new_cols) = regrid_res {
            if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
                doc.record_history();
                doc.project.regrid_atlas(new_cols);
                doc.viewport.needs_fit_to_view = true;
            }
        }

        if let Some((nw, nh, acol, arow)) = self.resize9_modal.show(ctx, &self.i18n) {
            if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
                doc.record_history();
                doc.project.resize_frames_9grid(nw, nh, acol, arow);
                doc.renderer.selection.clear();
                doc.viewport.needs_fit_to_view = true;
            }
        }

        let export_png_res = if let Some(doc) = self.documents.get(self.active_doc_idx) {
            self.export_png_modal.show(ctx, &self.i18n, &doc.project)
        } else {
            None
        };
        if let Some((scale, extrude)) = export_png_res {
            if let Some(doc) = self.documents.get(self.active_doc_idx) {
                if let Ok(png_bytes) = encode_scaled_png_bytes(&doc.project, scale, extrude) {
                    let default_name =
                        format!("{}_{}x.png", doc.project.metadata.project_name, scale);
                    self.io
                        .trigger_save_exported_png(&default_name, png_bytes, ctx.clone());
                }
            }
        }

        let export_gif_res = if let Some(doc) = self.documents.get(self.active_doc_idx) {
            self.export_gif_modal.show(ctx, &self.i18n, &doc.project)
        } else {
            None
        };
        if let Some((scale, start_f, end_f)) = export_gif_res {
            if let Some(doc) = self.documents.get(self.active_doc_idx) {
                if let Ok(gif_bytes) =
                    encode_animated_gif_bytes(&doc.project, scale, start_f, end_f)
                {
                    let default_name = format!("{}.gif", doc.project.metadata.project_name);
                    self.io
                        .trigger_save_exported_gif(&default_name, gif_bytes, ctx.clone());
                }
            }
        }

        let active_proj = self.documents.get(self.active_doc_idx).map(|d| &d.project);
        let export_packed_res = self.pack_atlas_modal.show(ctx, &self.i18n, active_proj);
        if let Some(action) = export_packed_res {
            if let Some(doc) = self.documents.get(self.active_doc_idx) {
                if let Ok(result) = crate::io::packer::pack_project_atlas(&doc.project, &action.settings) {
                    let default_name = doc.project.metadata.project_name.clone();
                    let json_text = crate::io::packer::encode_packed_atlas_json(
                        &result,
                        &doc.project,
                        &format!("{}_packed.png", default_name),
                    );
                    if let Ok(png_bytes) = IoManager::encode_png_bytes(&result.atlas_image) {
                        self.io.trigger_export_packed_atlas_dialog(
                            &default_name,
                            png_bytes,
                            json_text,
                            ctx.clone(),
                        );
                        self.status_key = "app.status.atlas_packed";
                    }
                }
            }
        }

        let any_fx_modal_open_before = self.replace_color_modal.open
            || self.color_adjust_modal.open
            || self.outline_fx_modal.open
            || self.convolution_modal.open
            || self.remap_palette_modal.open
            || self.rotate_modal.open;
        if any_fx_modal_open_before {
            if let Some(doc) = self.documents.get(self.active_doc_idx) {
                let needs_backup = match &self.modal_preview_backup {
                    Some((b_idx, _)) => *b_idx != self.active_doc_idx,
                    None => true,
                };
                if needs_backup {
                    self.modal_preview_backup =
                        Some((self.active_doc_idx, doc.project.atlas_rgba.clone()));
                }
            }
        }

        if self.rotate_modal.open || self.modify_selection_modal.open {
            if let Some(doc) = self.documents.get(self.active_doc_idx) {
                let needs_sel_backup = match &self.modal_selection_preview_backup {
                    Some((b_idx, _)) => *b_idx != self.active_doc_idx,
                    None => true,
                };
                if needs_sel_backup {
                    self.modal_selection_preview_backup =
                        Some((self.active_doc_idx, doc.renderer.selection.clone()));
                }
            }
        }

        let pal_slice = self
            .documents
            .get(self.active_doc_idx)
            .map(|d| d.project.metadata.palette.colors.as_slice());

        if let Some(action) = self.replace_color_modal.show(
            ctx,
            &self.i18n,
            self.tools.fg_color,
            self.tools.bg_color,
            Some(&mut self.settings),
            pal_slice,
        ) {
            if let Some((b_idx, orig_rgba)) = self.modal_preview_backup.take() {
                if let Some(doc) = self.documents.get_mut(b_idx) {
                    doc.project.atlas_rgba = orig_rgba;
                    doc.project.sync_active_layer();
                }
            }
            if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
                let snap = doc.current_snapshot();
                let fw = doc.project.metadata.frame_width;
                let fh = doc.project.metadata.frame_height;
                let cols = doc.project.metadata.columns.max(1);
                let sel_pixels = if action.in_selection_only && !doc.renderer.selection.is_empty() {
                    Some(&doc.renderer.selection.pixels)
                } else {
                    None
                };
                let mut changed = false;
                if action.all_frames {
                    let total = doc.project.metadata.frame_count;
                    for f in 0..total {
                        let col = f % cols;
                        let row = f / cols;
                        let ox = col * fw;
                        let oy = row * fh;
                        if crate::tools::fill::replace_color_in_region(
                            &mut doc.project.atlas_rgba,
                            ox,
                            oy,
                            fw,
                            fh,
                            action.from_color,
                            action.to_color,
                            sel_pixels,
                            action.tolerance,
                        ) {
                            changed = true;
                        }
                    }
                } else {
                    let f = doc.project.active_frame as u32;
                    let col = f % cols;
                    let row = f / cols;
                    let ox = col * fw;
                    let oy = row * fh;
                    if crate::tools::fill::replace_color_in_region(
                        &mut doc.project.atlas_rgba,
                        ox,
                        oy,
                        fw,
                        fh,
                        action.from_color,
                        action.to_color,
                        sel_pixels,
                        action.tolerance,
                    ) {
                        changed = true;
                    }
                }
                if changed {
                    doc.history.push(snap);
                    doc.project.has_unsaved_changes = true;
                    doc.project.canvas_dirty = true;
                    self.status_key = "app.status.color_replaced";
                }
            }
        }

        if let Some(action) = self.color_adjust_modal.show(ctx, &self.i18n) {
            if let Some((b_idx, orig_rgba)) = self.modal_preview_backup.take() {
                if let Some(doc) = self.documents.get_mut(b_idx) {
                    doc.project.atlas_rgba = orig_rgba;
                    doc.project.sync_active_layer();
                }
            }
            if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
                let snap = doc.current_snapshot();
                let fw = doc.project.metadata.frame_width;
                let fh = doc.project.metadata.frame_height;
                let cols = doc.project.metadata.columns.max(1);
                let sel_pixels = if action.in_selection_only && !doc.renderer.selection.is_empty() {
                    Some(&doc.renderer.selection.pixels)
                } else {
                    None
                };
                let mut changed = false;
                if action.all_frames {
                    let total = doc.project.metadata.frame_count;
                    for f in 0..total {
                        let col = f % cols;
                        let row = f / cols;
                        let ox = col * fw;
                        let oy = row * fh;
                        if crate::tools::fill::adjust_hsv_in_region(
                            &mut doc.project.atlas_rgba,
                            ox,
                            oy,
                            fw,
                            fh,
                            action.hue_shift,
                            action.sat_shift,
                            action.val_shift,
                            action.contrast,
                            sel_pixels,
                        ) {
                            changed = true;
                        }
                    }
                } else {
                    let f = doc.project.active_frame as u32;
                    let col = f % cols;
                    let row = f / cols;
                    let ox = col * fw;
                    let oy = row * fh;
                    if crate::tools::fill::adjust_hsv_in_region(
                        &mut doc.project.atlas_rgba,
                        ox,
                        oy,
                        fw,
                        fh,
                        action.hue_shift,
                        action.sat_shift,
                        action.val_shift,
                        action.contrast,
                        sel_pixels,
                    ) {
                        changed = true;
                    }
                }
                if changed {
                    doc.history.push(snap);
                    doc.project.has_unsaved_changes = true;
                    doc.project.canvas_dirty = true;
                    self.status_key = "app.status.colors_adjusted";
                }
            }
        }

        if let Some((mode, shape, amount)) = self.modify_selection_modal.show(ctx, &self.i18n) {
            if let Some((b_idx, orig_sel)) = self.modal_selection_preview_backup.take() {
                if let Some(doc) = self.documents.get_mut(b_idx) {
                    doc.renderer.selection = orig_sel;
                }
            }
            match mode {
                ModifySelectionMode::Expand => self.expand_selection(amount, shape),
                ModifySelectionMode::Contract => self.contract_selection(amount, shape),
                ModifySelectionMode::Border => self.border_selection(amount, shape),
            }
        } else if self.modify_selection_modal.open {
            if let Some((b_idx, ref orig_sel)) = self.modal_selection_preview_backup {
                if let Some(doc) = self.documents.get_mut(b_idx) {
                    doc.renderer.selection = orig_sel.clone();
                    if self.modify_selection_modal.preview && !doc.renderer.selection.is_empty() {
                        let fw = doc.project.metadata.frame_width;
                        let fh = doc.project.metadata.frame_height;
                        let amount = self.modify_selection_modal.amount.clamp(1, 32);
                        let shape = self.modify_selection_modal.shape;
                        match self.modify_selection_modal.mode {
                            ModifySelectionMode::Expand => {
                                doc.renderer.selection.expand(amount, shape, fw, fh);
                            }
                            ModifySelectionMode::Contract => {
                                doc.renderer.selection.contract(amount, shape);
                            }
                            ModifySelectionMode::Border => {
                                doc.renderer.selection.border(amount, shape);
                            }
                        }
                    }
                }
            }
        } else if !self.rotate_modal.open {
            if let Some((b_idx, orig_sel)) = self.modal_selection_preview_backup.take() {
                if let Some(doc) = self.documents.get_mut(b_idx) {
                    doc.renderer.selection = orig_sel;
                }
            }
        }

        if let Some(action) = self.outline_fx_modal.show(
            ctx,
            &self.i18n,
            self.tools.fg_color,
            self.tools.bg_color,
            Some(&mut self.settings),
        ) {
            if let Some((b_idx, orig_rgba)) = self.modal_preview_backup.take() {
                if let Some(doc) = self.documents.get_mut(b_idx) {
                    doc.project.atlas_rgba = orig_rgba;
                    doc.project.sync_active_layer();
                }
            }
            self.apply_outline_fx(action);
        }

        if let Some(action) = self.convolution_modal.show(ctx, &self.i18n) {
            if let Some((b_idx, orig_rgba)) = self.modal_preview_backup.take() {
                if let Some(doc) = self.documents.get_mut(b_idx) {
                    doc.project.atlas_rgba = orig_rgba;
                    doc.project.sync_active_layer();
                }
            }
            self.apply_convolution(action);
        }

        if let Some(action) = self.remap_palette_modal.show(ctx, &self.i18n) {
            if let Some((b_idx, orig_rgba)) = self.modal_preview_backup.take() {
                if let Some(doc) = self.documents.get_mut(b_idx) {
                    doc.project.atlas_rgba = orig_rgba;
                    doc.project.sync_active_layer();
                }
            }
            self.apply_palette_remap(action);
        }

        if let Some(action) = self.rotate_modal.show(ctx, &self.i18n) {
            if let Some((b_idx, orig_rgba)) = self.modal_preview_backup.take() {
                if let Some(doc) = self.documents.get_mut(b_idx) {
                    doc.project.atlas_rgba = orig_rgba;
                    doc.project.sync_active_layer();
                }
            }
            if let Some((b_idx, orig_sel)) = self.modal_selection_preview_backup.take() {
                if let Some(doc) = self.documents.get_mut(b_idx) {
                    doc.renderer.selection = orig_sel;
                }
            }
            self.apply_rotate_arbitrary(action);
        }

        // Handle live preview or cancel restoration for the 6 FX modals
        let any_fx_modal_open_after = self.replace_color_modal.open
            || self.color_adjust_modal.open
            || self.outline_fx_modal.open
            || self.convolution_modal.open
            || self.remap_palette_modal.open
            || self.rotate_modal.open;

        if any_fx_modal_open_after {
            if let Some((b_idx, ref orig_rgba)) = self.modal_preview_backup {
                if let Some(doc) = self.documents.get_mut(b_idx) {
                    doc.project.atlas_rgba = orig_rgba.clone();
                    let fw = doc.project.metadata.frame_width;
                    let fh = doc.project.metadata.frame_height;

                    if self.replace_color_modal.open
                        && self.replace_color_modal.preview
                        && self.replace_color_modal.picking.is_none()
                    {
                        let sel_pixels = if self.replace_color_modal.in_selection_only
                            && !doc.renderer.selection.is_empty()
                        {
                            Some(&doc.renderer.selection.pixels)
                        } else {
                            None
                        };
                        let frames: Vec<usize> = if self.replace_color_modal.all_frames {
                            (0..doc.project.metadata.frame_count as usize).collect()
                        } else {
                            vec![doc.project.active_frame]
                        };
                        for f_idx in frames {
                            let (ox, oy) = doc.project.metadata.frame_origin(f_idx);
                            let _ = crate::tools::fill::replace_color_in_region(
                                &mut doc.project.atlas_rgba,
                                ox,
                                oy,
                                fw,
                                fh,
                                self.replace_color_modal.from_color,
                                self.replace_color_modal.to_color,
                                sel_pixels,
                                self.replace_color_modal.tolerance,
                            );
                        }
                    } else if self.color_adjust_modal.open && self.color_adjust_modal.preview {
                        let sel_pixels = if self.color_adjust_modal.in_selection_only
                            && !doc.renderer.selection.is_empty()
                        {
                            Some(&doc.renderer.selection.pixels)
                        } else {
                            None
                        };
                        let frames: Vec<usize> = if self.color_adjust_modal.all_frames {
                            (0..doc.project.metadata.frame_count as usize).collect()
                        } else {
                            vec![doc.project.active_frame]
                        };
                        for f_idx in frames {
                            let (ox, oy) = doc.project.metadata.frame_origin(f_idx);
                            let _ = crate::tools::fill::adjust_hsv_in_region(
                                &mut doc.project.atlas_rgba,
                                ox,
                                oy,
                                fw,
                                fh,
                                self.color_adjust_modal.hue_shift,
                                self.color_adjust_modal.sat_shift,
                                self.color_adjust_modal.val_shift,
                                self.color_adjust_modal.contrast,
                                sel_pixels,
                            );
                        }
                    } else if self.outline_fx_modal.open && self.outline_fx_modal.preview {
                        let sel_pixels = if self.outline_fx_modal.in_selection_only
                            && !doc.renderer.selection.is_empty()
                        {
                            Some(&doc.renderer.selection.pixels)
                        } else {
                            None
                        };
                        let frames: Vec<usize> = if self.outline_fx_modal.all_frames {
                            (0..doc.project.metadata.frame_count as usize).collect()
                        } else {
                            vec![doc.project.active_frame]
                        };
                        for f_idx in frames {
                            let (ox, oy) = doc.project.metadata.frame_origin(f_idx);
                            let _ = crate::tools::fill::generate_outline_in_region(
                                &mut doc.project.atlas_rgba,
                                ox,
                                oy,
                                fw,
                                fh,
                                self.outline_fx_modal.color,
                                self.outline_fx_modal.thickness.clamp(1, 8),
                                self.outline_fx_modal.corner_style,
                                self.outline_fx_modal.placement,
                                sel_pixels,
                            );
                        }
                    } else if self.convolution_modal.open && self.convolution_modal.preview {
                        let sel_pixels = if self.convolution_modal.in_selection_only
                            && !doc.renderer.selection.is_empty()
                        {
                            Some(&doc.renderer.selection.pixels)
                        } else {
                            None
                        };
                        let frames: Vec<usize> = if self.convolution_modal.all_frames {
                            (0..doc.project.metadata.frame_count as usize).collect()
                        } else {
                            vec![doc.project.active_frame]
                        };
                        for f_idx in frames {
                            let (ox, oy) = doc.project.metadata.frame_origin(f_idx);
                            let _ = crate::tools::filters::apply_convolution_matrix(
                                &mut doc.project.atlas_rgba,
                                ox,
                                oy,
                                fw,
                                fh,
                                &self.convolution_modal.kernel,
                                self.convolution_modal.divisor,
                                self.convolution_modal.bias,
                                self.convolution_modal.preserve_alpha,
                                sel_pixels,
                            );
                        }
                    } else if self.remap_palette_modal.open && self.remap_palette_modal.preview {
                        let sel_pixels = if self.remap_palette_modal.in_selection_only
                            && !doc.renderer.selection.is_empty()
                        {
                            Some(&doc.renderer.selection.pixels)
                        } else {
                            None
                        };
                        let frames: Vec<usize> = if self.remap_palette_modal.all_frames {
                            (0..doc.project.metadata.frame_count as usize).collect()
                        } else {
                            vec![doc.project.active_frame]
                        };
                        let palette = doc.project.metadata.palette.colors.clone();
                        if !palette.is_empty() {
                            for f_idx in frames {
                                let (ox, oy) = doc.project.metadata.frame_origin(f_idx);
                                let _ = remap_pixels_to_palette(
                                    &mut doc.project.atlas_rgba,
                                    ox,
                                    oy,
                                    fw,
                                    fh,
                                    &palette,
                                    self.remap_palette_modal.dither,
                                    self.remap_palette_modal.dither_strength,
                                    sel_pixels,
                                );
                            }
                        }
                    } else if self.rotate_modal.open {
                        if let Some((_, ref orig_sel)) = self.modal_selection_preview_backup {
                            doc.renderer.selection = orig_sel.clone();
                        }
                        if self.rotate_modal.preview && self.rotate_modal.angle_deg.abs() > 0.001 {
                            let frames: Vec<usize> = if self.rotate_modal.all_frames {
                                (0..doc.project.metadata.frame_count as usize).collect()
                            } else {
                                vec![doc.project.active_frame]
                            };
                            if !doc.renderer.selection.is_empty() && self.rotate_modal.in_selection_only {
                                let mut last_new = None;
                                for f_idx in frames {
                                    let (ox, oy) = doc.project.metadata.frame_origin(f_idx);
                                    let np = crate::tools::transforms::rotate_selection_arbitrary(
                                        &mut doc.project.atlas_rgba,
                                        ox,
                                        oy,
                                        fw,
                                        fh,
                                        self.rotate_modal.angle_deg,
                                        self.rotate_modal.algorithm,
                                        &doc.renderer.selection.pixels,
                                    );
                                    last_new = Some(np);
                                }
                                if let Some(np) = last_new {
                                    doc.renderer.selection.pixels = np;
                                }
                            } else {
                                for f_idx in frames {
                                    let (ox, oy) = doc.project.metadata.frame_origin(f_idx);
                                    crate::tools::transforms::rotate_region_arbitrary(
                                        &mut doc.project.atlas_rgba,
                                        ox,
                                        oy,
                                        fw,
                                        fh,
                                        self.rotate_modal.angle_deg,
                                        self.rotate_modal.algorithm,
                                    );
                                }
                            }
                        }
                    }

                    doc.project.canvas_dirty = true;
                    let _ = doc.renderer.ensure_texture(ctx, &mut doc.project);
                }
            }
        } else if let Some((b_idx, orig_rgba)) = self.modal_preview_backup.take() {
            if let Some(doc) = self.documents.get_mut(b_idx) {
                doc.project.atlas_rgba = orig_rgba;
                doc.project.sync_active_layer();
                doc.project.canvas_dirty = true;
                let _ = doc.renderer.ensure_texture(ctx, &mut doc.project);
            }
            if !self.modify_selection_modal.open {
                if let Some((b_idx, orig_sel)) = self.modal_selection_preview_backup.take() {
                    if let Some(doc) = self.documents.get_mut(b_idx) {
                        doc.renderer.selection = orig_sel;
                    }
                }
            }
        }

        if let Some(sliced_proj) = self.slicer_modal.show(ctx, &self.i18n) {
            self.open_project_doc(sliced_proj);
            self.status_key = "app.status.project_loaded";
        }

        if let Some(recovery_action) =
            show_recovery_modal(ctx, &self.i18n, self.autosave.recovery_candidate.is_some())
        {
            match recovery_action {
                RecoveryAction::Restore => {
                    if let Some(recovered) = self.autosave.recovery_candidate.take() {
                        self.open_project_doc(recovered);
                        self.status_key = "app.status.autosave_restored";
                    }
                }
                RecoveryAction::Discard => {
                    self.autosave.recovery_candidate = None;
                    AutosaveService::clear_recovery(frame.storage_mut());
                }
            }
        }

        if let Some(confirmed_del_idx) =
            show_confirm_delete_frame_modal(ctx, &self.i18n, &mut self.pending_delete_frame)
        {
            if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
                let snap = doc.current_snapshot();
                if doc.project.delete_frame_at(confirmed_del_idx) {
                    doc.history.push(snap);
                }
            }
        }

        if let Some(confirmed_del_idx) =
            show_confirm_delete_layer_modal(ctx, &self.i18n, &mut self.pending_delete_layer)
        {
            if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
                doc.project.sync_active_layer();
                let snap = doc.current_snapshot();
                if doc.project.delete_layer(confirmed_del_idx) {
                    doc.history.push(snap);
                    doc.project.has_unsaved_changes = true;
                    doc.project.canvas_dirty = true;
                }
            }
        }

        if let Some(confirmed_del_idx) =
            show_confirm_delete_skin_modal(ctx, &self.i18n, &mut self.pending_delete_skin)
        {
            if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
                if doc.project.metadata.palette.swaps.len() > 1
                    && confirmed_del_idx < doc.project.metadata.palette.swaps.len()
                {
                    let snap = doc.current_snapshot();
                    let old_colors = doc.project.metadata.palette.colors.clone();
                    doc.project.metadata.palette.swaps.remove(confirmed_del_idx);
                    let next_active = doc
                        .project
                        .metadata
                        .palette
                        .active_swap_index
                        .min(doc.project.metadata.palette.swaps.len() - 1);
                    doc.project.metadata.palette.active_swap_index = next_active;
                    if let Some(target_swap) =
                        doc.project.metadata.palette.swaps.get(next_active)
                    {
                        let new_colors = target_swap.colors.clone();
                        if crate::model::palette::apply_palette_swap_to_atlas(
                            &mut doc.project.atlas_rgba,
                            &old_colors,
                            &new_colors,
                        ) {
                            doc.project.canvas_dirty = true;
                        }
                        doc.project.metadata.palette.colors = new_colors;
                    }
                    doc.history.push(snap);
                    self.status_key = "app.status.skin_deleted";
                }
            }
        }

        if let Some(slot_idx) = show_confirm_delete_palette_slot_modal(
            ctx,
            &self.i18n,
            &mut self.pending_delete_palette_slot,
        ) {
            if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
                if doc.project.metadata.palette.colors.len() > 1
                    && slot_idx < doc.project.metadata.palette.colors.len()
                {
                    let snap = doc.current_snapshot();
                    doc.project.metadata.palette.colors.remove(slot_idx);
                    for swap in &mut doc.project.metadata.palette.swaps {
                        if slot_idx < swap.colors.len() {
                            swap.colors.remove(slot_idx);
                        }
                    }
                    doc.history.push(snap);
                }
            }
        }

        if let Some(pt_idx) = show_confirm_delete_pivot_point_modal(
            ctx,
            &self.i18n,
            &mut self.pending_delete_pivot_point,
        ) {
            if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
                if pt_idx < doc.project.metadata.pivot.action_points.len() {
                    let snap = doc.current_snapshot();
                    doc.project.metadata.pivot.action_points.remove(pt_idx);
                    if doc.preview.active_pivot_target == Some(pt_idx) {
                        doc.preview.active_pivot_target = None;
                    }
                    doc.history.push(snap);
                }
            }
        }

        if let Some(t_idx) = show_confirm_delete_tag_modal(
            ctx,
            &self.i18n,
            &mut self.pending_delete_tag,
        ) {
            if let Some(doc) = self.documents.get_mut(self.active_doc_idx) {
                if t_idx < doc.project.metadata.tags.len() {
                    let snap = doc.current_snapshot();
                    doc.project.metadata.tags.remove(t_idx);
                    if doc.preview.active_tag_index == Some(t_idx) {
                        doc.preview.active_tag_index = None;
                    }
                    doc.history.push(snap);
                }
            }
        }

        if let Some(path) = show_confirm_delete_recent_modal(
            ctx,
            &self.i18n,
            &mut self.pending_delete_recent,
        ) {
            self.settings.remove_recent(&path);
        }

        let pending_idx = self.pending_close_doc_idx;
        if let Some(idx) = pending_idx {
            let doc_name = self
                .documents
                .get(idx)
                .map(|d| d.project.metadata.project_name.clone())
                .unwrap_or_else(|| "project".to_string());

            if let Some(action) = show_confirm_close_doc_modal(
                ctx,
                &self.i18n,
                &doc_name,
                &mut self.pending_close_doc_idx,
            ) {
                match action {
                    ConfirmCloseDocAction::SaveAndClose => {
                        if let Some(doc) = self.documents.get(idx) {
                            self.io.trigger_save(&doc.project, false, ctx.clone());
                            self.pending_after_save = Some(PendingAfterSave::CloseDoc(idx));
                        }
                    }
                    ConfirmCloseDocAction::DiscardAndClose => {
                        self.close_doc_now(idx);
                        if self.pending_after_save == Some(PendingAfterSave::CloseApp) {
                            if let Some(next_dirty) = self
                                .documents
                                .iter()
                                .position(|d| d.project.has_unsaved_changes)
                            {
                                self.active_doc_idx = next_dirty;
                                self.pending_close_doc_idx = Some(next_dirty);
                            } else {
                                self.confirmed_exit = true;
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                        }
                    }
                    ConfirmCloseDocAction::Cancel => {
                        self.pending_after_save = None;
                        self.confirmed_exit = false;
                    }
                }
            }
        }

        if show_confirm_exit_app_modal(ctx, &self.i18n, &mut self.show_confirm_exit_modal) {
            self.confirmed_exit = true;
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        show_about_modal(ctx, &self.i18n, &mut self.show_about_modal);
        show_hotkeys_modal(ctx, &mut self.show_hotkeys_modal, &self.i18n);

        let recent_id = egui::Id::new("onianvil_recent_colors");
        let ctx_recents: Option<Vec<[u8; 4]>> = ctx.data(|d| d.get_temp(recent_id));
        if let Some(recents) = ctx_recents {
            if recents != self.settings.recent_colors {
                self.settings.recent_colors = recents;
                let _ = self.settings.save();
            }
        } else {
            ctx.data_mut(|d| d.insert_temp(recent_id, self.settings.recent_colors.clone()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::metadata::ProjectMetadata;
    use crate::model::project::Project;

    #[test]
    fn test_pixel_clipboard_copy_and_paste() {
        let meta = ProjectMetadata::new_config("test_clip".to_string(), 8, 8, 1, 1, None);
        let mut proj = Project::new(meta);
        proj.atlas_rgba.put_pixel(0, 0, Rgba([255, 0, 0, 255]));
        proj.atlas_rgba.put_pixel(1, 0, Rgba([0, 255, 0, 255]));

        let mut app = OniAnvilApp {
            i18n: I18n::new(crate::i18n::Language::English),
            clipboard: None,
            documents: vec![crate::model::document::ProjectDocument::new(proj)],
            active_doc_idx: 0,
            pending_close_doc_idx: None,
            settings: AppSettings::default(),
            tools: ToolSettings::default(),
            io: IoManager::new(),
            autosave: AutosaveService::default(),
            update_checker: crate::io::updater::UpdateChecker::default(),
            wizard: NewProjectWizardState::default(),
            regrid_modal: ReGridModalState::default(),
            resize9_modal: Resize9GridModalState::default(),
            slicer_modal: SheetSlicerModalState::default(),
            export_png_modal: ExportScaledPngModalState::default(),
            export_gif_modal: ExportGifModalState::default(),
            replace_color_modal: ReplaceColorModalState::default(),
            color_adjust_modal: ColorAdjustModalState::default(),
            modify_selection_modal: ModifySelectionModalState::default(),
            outline_fx_modal: OutlineFxModalState::default(),
            convolution_modal: ConvolutionModalState::default(),
            remap_palette_modal: RemapPaletteModalState::default(),
            rotate_modal: RotateModalState::default(),
            pack_atlas_modal: ExportPackedAtlasModalState::default(),
            modal_preview_backup: None,
            modal_selection_preview_backup: None,
            pending_after_save: None,
            pending_delete_frame: None,
            pending_delete_layer: None,
            pending_delete_skin: None,
            pending_delete_palette_slot: None,
            pending_delete_pivot_point: None,
            pending_delete_tag: None,
            pending_delete_recent: None,
            show_right_dock: true,
            show_bottom_dock: true,
            show_about_modal: false,
            show_hotkeys_modal: false,
            show_confirm_exit_modal: false,
            confirmed_exit: false,
            app_theme: crate::theme::AppTheme::StudioDark,
            status_key: "app.status.ready",
            logo_texture: None,
        };

        // Copy entire frame (since no selection)
        app.copy_selection(None);
        assert!(app.clipboard.is_some());
        let clip = app.clipboard.as_ref().unwrap();
        assert_eq!(clip.pixels.len(), 2);

        // Clear atlas
        let doc = &mut app.documents[0];
        doc.project.atlas_rgba.put_pixel(0, 0, Rgba([0, 0, 0, 0]));
        doc.project.atlas_rgba.put_pixel(1, 0, Rgba([0, 0, 0, 0]));

        // Paste clipboard back
        app.paste_clipboard(None);
        let doc = &app.documents[0];
        assert_eq!(*doc.project.atlas_rgba.get_pixel(0, 0), Rgba([255, 0, 0, 255]));
        assert_eq!(*doc.project.atlas_rgba.get_pixel(1, 0), Rgba([0, 255, 0, 255]));
        assert!(doc.renderer.selection.contains(0, 0));
        assert!(doc.renderer.selection.contains(1, 0));
    }

    #[test]
    fn test_cross_frame_copy_and_paste() {
        let meta = ProjectMetadata::new_config("test_cross_frame".to_string(), 16, 16, 2, 2, None);
        let mut proj = Project::new(meta);
        let (ox0, oy0) = proj.metadata.frame_origin(0);
        let (ox1, oy1) = proj.metadata.frame_origin(1);
        proj.atlas_rgba.put_pixel(ox0 + 3, oy0 + 4, Rgba([120, 200, 50, 255]));
        proj.atlas_rgba.put_pixel(ox0 + 7, oy0 + 8, Rgba([200, 50, 120, 255]));

        let mut app = OniAnvilApp {
            i18n: I18n::new(crate::i18n::Language::English),
            clipboard: None,
            documents: vec![crate::model::document::ProjectDocument::new(proj)],
            active_doc_idx: 0,
            pending_close_doc_idx: None,
            settings: AppSettings::default(),
            tools: ToolSettings::default(),
            io: IoManager::new(),
            autosave: AutosaveService::default(),
            update_checker: crate::io::updater::UpdateChecker::default(),
            wizard: NewProjectWizardState::default(),
            regrid_modal: ReGridModalState::default(),
            resize9_modal: Resize9GridModalState::default(),
            slicer_modal: SheetSlicerModalState::default(),
            export_png_modal: ExportScaledPngModalState::default(),
            export_gif_modal: ExportGifModalState::default(),
            replace_color_modal: ReplaceColorModalState::default(),
            color_adjust_modal: ColorAdjustModalState::default(),
            modify_selection_modal: ModifySelectionModalState::default(),
            outline_fx_modal: OutlineFxModalState::default(),
            convolution_modal: ConvolutionModalState::default(),
            remap_palette_modal: RemapPaletteModalState::default(),
            rotate_modal: RotateModalState::default(),
            pack_atlas_modal: ExportPackedAtlasModalState::default(),
            modal_preview_backup: None,
            modal_selection_preview_backup: None,
            pending_after_save: None,
            pending_delete_frame: None,
            pending_delete_layer: None,
            pending_delete_skin: None,
            pending_delete_palette_slot: None,
            pending_delete_pivot_point: None,
            pending_delete_tag: None,
            pending_delete_recent: None,
            show_right_dock: true,
            show_bottom_dock: true,
            show_about_modal: false,
            show_hotkeys_modal: false,
            show_confirm_exit_modal: false,
            confirmed_exit: false,
            app_theme: crate::theme::AppTheme::StudioDark,
            status_key: "app.status.ready",
            logo_texture: None,
        };

        // Frame 0 is active
        assert_eq!(app.documents[0].project.active_frame, 0);

        // 1. Copy entire frame 0 (no selection active)
        app.copy_selection(None);
        assert!(app.clipboard.is_some());
        assert_eq!(app.clipboard.as_ref().unwrap().pixels.len(), 2);

        // 2. Switch to Frame 1
        app.documents[0].project.active_frame = 1;
        assert_eq!(*app.documents[0].project.atlas_rgba.get_pixel(ox1 + 3, oy1 + 4), Rgba([0, 0, 0, 0]));
        assert_eq!(*app.documents[0].project.atlas_rgba.get_pixel(ox1 + 7, oy1 + 8), Rgba([0, 0, 0, 0]));

        // 3. Paste onto Frame 1
        app.paste_clipboard(None);
        let doc = &app.documents[0];
        assert_eq!(*doc.project.atlas_rgba.get_pixel(ox1 + 3, oy1 + 4), Rgba([120, 200, 50, 255]));
        assert_eq!(*doc.project.atlas_rgba.get_pixel(ox1 + 7, oy1 + 8), Rgba([200, 50, 120, 255]));
        assert!(doc.renderer.selection.contains(3, 4));
        assert!(doc.renderer.selection.contains(7, 8));
        assert_eq!(doc.project.metadata.layers[0].layer_rgba.as_ref().unwrap().get_pixel(ox1 + 3, oy1 + 4), &Rgba([120, 200, 50, 255]));

        // 4. Test serialization round-trip via CLIPBOARD_PREFIX
        let payload = format!("{}{}", CLIPBOARD_PREFIX, serde_json::to_string(&app.clipboard.as_ref().unwrap().pixels).unwrap());
        let rest = payload.strip_prefix(CLIPBOARD_PREFIX).unwrap();
        let deserialized: Vec<((i32, i32), [u8; 4])> = serde_json::from_str(rest).unwrap();
        assert_eq!(deserialized.len(), 2);
    }

    #[test]
    fn test_create_custom_brush_from_selection_and_stamp() {
        let meta = ProjectMetadata::new_config("test_brush".to_string(), 16, 16, 1, 1, None);
        let mut proj = Project::new(meta);
        proj.atlas_rgba.put_pixel(2, 2, Rgba([100, 150, 200, 255]));
        proj.atlas_rgba.put_pixel(3, 2, Rgba([110, 160, 210, 255]));

        let mut app = OniAnvilApp {
            i18n: I18n::new(crate::i18n::Language::English),
            clipboard: None,
            documents: vec![crate::model::document::ProjectDocument::new(proj)],
            active_doc_idx: 0,
            pending_close_doc_idx: None,
            settings: AppSettings::default(),
            tools: ToolSettings::default(),
            io: IoManager::new(),
            autosave: AutosaveService::default(),
            update_checker: crate::io::updater::UpdateChecker::default(),
            wizard: NewProjectWizardState::default(),
            regrid_modal: ReGridModalState::default(),
            resize9_modal: Resize9GridModalState::default(),
            slicer_modal: SheetSlicerModalState::default(),
            export_png_modal: ExportScaledPngModalState::default(),
            export_gif_modal: ExportGifModalState::default(),
            replace_color_modal: ReplaceColorModalState::default(),
            color_adjust_modal: ColorAdjustModalState::default(),
            modify_selection_modal: ModifySelectionModalState::default(),
            outline_fx_modal: OutlineFxModalState::default(),
            convolution_modal: ConvolutionModalState::default(),
            remap_palette_modal: RemapPaletteModalState::default(),
            rotate_modal: RotateModalState::default(),
            pack_atlas_modal: ExportPackedAtlasModalState::default(),
            modal_preview_backup: None,
            modal_selection_preview_backup: None,
            pending_after_save: None,
            pending_delete_frame: None,
            pending_delete_layer: None,
            pending_delete_skin: None,
            pending_delete_palette_slot: None,
            pending_delete_pivot_point: None,
            pending_delete_tag: None,
            pending_delete_recent: None,
            show_right_dock: true,
            show_bottom_dock: true,
            show_about_modal: false,
            show_hotkeys_modal: false,
            show_confirm_exit_modal: false,
            confirmed_exit: false,
            app_theme: crate::theme::AppTheme::StudioDark,
            status_key: "app.status.ready",
            logo_texture: None,
        };

        // Select the two pixels at (2, 2) and (3, 2)
        {
            let doc = &mut app.documents[0];
            doc.renderer.selection.pixels.insert((2, 2));
            doc.renderer.selection.pixels.insert((3, 2));
        }

        app.create_custom_brush_from_selection();

        assert!(app.tools.custom_brush.is_some());
        let cb = app.tools.custom_brush.as_ref().unwrap();
        assert_eq!(cb.width, 2);
        assert_eq!(cb.height, 1);
        assert_eq!(cb.pixels.len(), 2);
        assert_eq!(app.tools.active_tool, ToolKind::Pencil);
        assert!(app.documents[0].renderer.selection.is_empty());
    }
}
