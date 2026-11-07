use crate::canvas::renderer::CanvasRenderer;
use crate::canvas::viewport::ViewportState;
use crate::history::command::{HistorySnapshot, HistoryStack};
use crate::model::project::Project;
use crate::ui::right_dock::PreviewState;

pub struct ProjectDocument {
    pub project: Project,
    pub viewport: ViewportState,
    pub history: HistoryStack,
    pub renderer: CanvasRenderer,
    pub preview: PreviewState,
    pub renaming_tab: bool,
}

impl ProjectDocument {
    pub fn new(project: Project) -> Self {
        Self {
            project,
            viewport: ViewportState::default(),
            history: HistoryStack::new(),
            renderer: CanvasRenderer::default(),
            preview: PreviewState::default(),
            renaming_tab: false,
        }
    }

    pub fn record_history(&mut self) {
        let snap = self.current_snapshot();
        self.history.push(snap);
        self.project.has_unsaved_changes = true;
    }

    pub fn current_snapshot(&self) -> HistorySnapshot {
        self.project.current_snapshot()
    }

    pub fn restore_snapshot(&mut self, snap: HistorySnapshot) {
        self.project.atlas_rgba = snap.atlas_rgba;
        self.project.metadata = snap.metadata;
        self.project.ensure_layers_initialized();
        self.project.sync_active_layer();
        self.project.active_frame = snap
            .active_frame
            .min((self.project.metadata.frame_count as usize).saturating_sub(1));
        self.project.canvas_dirty = true;
        self.project.layers_dirty = true;
    }

    pub fn perform_undo(&mut self) {
        let current = self.current_snapshot();
        if let Some(prev) = self.history.undo(current) {
            self.restore_snapshot(prev);
        }
    }

    pub fn perform_redo(&mut self) {
        let current = self.current_snapshot();
        if let Some(next) = self.history.redo(current) {
            self.restore_snapshot(next);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_history_undo_redo() {
        let mut doc = ProjectDocument::new(Project::default());
        assert!(!doc.history.can_undo());

        doc.record_history();
        doc.project.metadata.project_name = "RenamedProject".to_string();

        assert!(doc.history.can_undo());
        doc.perform_undo();
        assert_eq!(doc.project.metadata.project_name, "sprite");

        assert!(doc.history.can_redo());
        doc.perform_redo();
        assert_eq!(doc.project.metadata.project_name, "RenamedProject");
    }
}
