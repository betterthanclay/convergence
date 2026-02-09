use super::ReleaseArgs;

pub(in crate::tui_shell::app) fn parse_release_args(
    args: &[String],
) -> Result<ReleaseArgs, String> {
    let mut out = ReleaseArgs::default();

    if !args.iter().any(|arg| arg.starts_with("--")) && args.len() >= 2 {
        out.channel = Some(args[0].clone());
        out.bundle_id = Some(args[1].clone());
        if args.len() > 2 {
            out.notes = Some(args[2..].join(" "));
        }
    }

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--channel" | "channel" => {
                i += 1;
                if i >= args.len() {
                    return Err("missing value for --channel".to_string());
                }
                out.channel = Some(args[i].clone());
            }
            "--bundle-id" | "bundle" => {
                i += 1;
                if i >= args.len() {
                    return Err("missing value for --bundle-id".to_string());
                }
                out.bundle_id = Some(args[i].clone());
            }
            "--notes" | "notes" => {
                i += 1;
                if i >= args.len() {
                    return Err("missing value for --notes".to_string());
                }
                out.notes = Some(args[i].clone());
            }
            arg => {
                if arg.starts_with("--") {
                    return Err(format!("unknown arg: {}", arg));
                }
            }
        }
        i += 1;
    }

    Ok(out)
}
