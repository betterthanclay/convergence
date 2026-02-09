use super::PinArgs;

pub(in crate::tui_shell::app) fn parse_pin_args(args: &[String]) -> Result<PinArgs, String> {
    let mut out = PinArgs::default();
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
            "--unpin" | "unpin" => {
                out.unpin = true;
            }
            arg => {
                if !arg.starts_with("--") && out.bundle_id.is_none() {
                    out.bundle_id = Some(arg.to_string());
                } else {
                    return Err(format!("unknown arg: {}", arg));
                }
            }
        }
        i += 1;
    }
    Ok(out)
}
