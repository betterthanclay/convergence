pub(in crate::tui_shell::status) fn min_blob_rename_score(max_len: usize) -> f64 {
    // Adaptive threshold: small files should still rename-match after small edits.
    // Keep it conservative to avoid spurious matches.
    if max_len <= 512 {
        0.65
    } else if max_len <= 4 * 1024 {
        0.70
    } else if max_len <= 16 * 1024 {
        0.78
    } else {
        0.85
    }
}

pub(in crate::tui_shell::status) fn min_blob_rename_matched_bytes(max_len: usize) -> usize {
    // Guardrail for tiny files where many candidates might otherwise tie.
    if max_len <= 128 {
        max_len / 2
    } else if max_len <= 4 * 1024 {
        32
    } else {
        0
    }
}

pub(in crate::tui_shell::status) fn default_chunk_size_bytes() -> usize {
    // Keep in sync with workspace defaults.
    4 * 1024 * 1024
}

pub(in crate::tui_shell::status) fn min_recipe_rename_score(max_chunks: usize) -> f64 {
    if max_chunks <= 8 {
        0.60
    } else if max_chunks <= 32 {
        0.75
    } else {
        0.90
    }
}

pub(in crate::tui_shell::status) fn min_recipe_rename_matched_chunks(max_chunks: usize) -> usize {
    if max_chunks <= 8 {
        2
    } else if max_chunks <= 32 {
        4
    } else {
        0
    }
}
