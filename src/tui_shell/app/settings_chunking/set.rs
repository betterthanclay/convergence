use super::*;

pub(super) fn handle_set_chunking(app: &mut App, ws: &Workspace, args: &[String]) {
    let mut chunk_size_mib: Option<u64> = None;
    let mut threshold_mib: Option<u64> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--chunk-size-mib" => {
                i += 1;
                let Some(v) = args.get(i) else {
                    app.push_error("missing value for --chunk-size-mib".to_string());
                    return;
                };
                chunk_size_mib = v.parse::<u64>().ok();
            }
            "--threshold-mib" => {
                i += 1;
                let Some(v) = args.get(i) else {
                    app.push_error("missing value for --threshold-mib".to_string());
                    return;
                };
                threshold_mib = v.parse::<u64>().ok();
            }
            _ => {
                app.push_error(
                    "usage: settings chunking set --chunk-size-mib N --threshold-mib N".to_string(),
                );
                return;
            }
        }
        i += 1;
    }

    let Some(chunk_size_mib) = chunk_size_mib else {
        app.push_error("missing --chunk-size-mib".to_string());
        return;
    };
    let Some(threshold_mib) = threshold_mib else {
        app.push_error("missing --threshold-mib".to_string());
        return;
    };

    let chunk_size = chunk_size_mib * 1024 * 1024;
    let threshold = threshold_mib * 1024 * 1024;

    let mut cfg = match ws.store.read_config() {
        Ok(c) => c,
        Err(err) => {
            app.push_error(format!("read config: {:#}", err));
            return;
        }
    };
    cfg.chunking = Some(ChunkingConfig {
        chunk_size,
        threshold,
    });
    if let Err(err) = ws.store.write_config(&cfg) {
        app.push_error(format!("write config: {:#}", err));
        return;
    }

    app.refresh_root_view();
    app.push_output(vec!["updated chunking config".to_string()]);
}
