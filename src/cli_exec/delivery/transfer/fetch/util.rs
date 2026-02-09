pub(super) fn default_temp_destination(prefix: &str, id: &str) -> std::path::PathBuf {
    let short = id.chars().take(8).collect::<String>();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    std::env::temp_dir().join(format!("{}-{}-{}", prefix, short, nanos))
}
