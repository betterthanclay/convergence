pub(super) fn temp_restore_path(kind: &str, id_prefix: &str) -> std::path::PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    std::env::temp_dir().join(format!("converge-grab-{}-{}-{}", kind, id_prefix, nanos))
}
