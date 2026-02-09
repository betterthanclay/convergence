use super::SnapsView;

impl SnapsView {
    pub(super) fn has_pending_row(&self) -> bool {
        self.pending_changes.is_some_and(|s| s.total() > 0)
    }

    pub(super) fn has_clean_row(&self) -> bool {
        self.pending_changes.is_none() && self.head_id.is_some() && !self.all_items.is_empty()
    }

    pub(super) fn has_header_row(&self) -> bool {
        self.has_pending_row() || self.has_clean_row()
    }

    pub(super) fn rows_len(&self) -> usize {
        let mut n = self.items.len();
        if self.has_header_row() {
            n += 1;
        }
        if self.items.is_empty() {
            n += 1;
        }
        n
    }

    pub(in crate::tui_shell) fn selected_is_pending(&self) -> bool {
        self.has_pending_row() && self.selected_row.min(self.rows_len().saturating_sub(1)) == 0
    }

    pub(in crate::tui_shell) fn selected_is_clean(&self) -> bool {
        self.has_clean_row() && self.selected_row.min(self.rows_len().saturating_sub(1)) == 0
    }

    pub(in crate::tui_shell) fn selected_snap_index(&self) -> Option<usize> {
        if self.items.is_empty() {
            return None;
        }
        let row = self.selected_row.min(self.rows_len().saturating_sub(1));
        let idx = if self.has_header_row() {
            if row == 0 {
                return None;
            }
            row - 1
        } else {
            row
        };
        if idx < self.items.len() {
            Some(idx)
        } else {
            None
        }
    }
}
