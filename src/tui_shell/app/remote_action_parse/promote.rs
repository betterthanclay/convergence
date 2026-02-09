use super::PromoteArgs;

pub(in crate::tui_shell::app) fn parse_promote_args(
    args: &[String],
) -> Result<PromoteArgs, String> {
    let mut out = PromoteArgs::default();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--bundle-id" | "bundle" => {
                i += 1;
                if i >= args.len() {
                    return Err("missing value for --bundle-id".to_string());
                }
                out.bundle_id = Some(args[i].clone());
            }
            "--to-gate" | "to" => {
                i += 1;
                if i >= args.len() {
                    return Err("missing value for --to-gate".to_string());
                }
                out.to_gate = Some(args[i].clone());
            }
            arg => {
                if !arg.starts_with("--") {
                    if out.bundle_id.is_none() {
                        out.bundle_id = Some(arg.to_string());
                    } else if out.to_gate.is_none() {
                        out.to_gate = Some(arg.to_string());
                    } else {
                        return Err(format!("unknown arg: {}", arg));
                    }
                } else {
                    return Err(format!("unknown arg: {}", arg));
                }
            }
        }
        i += 1;
    }
    Ok(out)
}
