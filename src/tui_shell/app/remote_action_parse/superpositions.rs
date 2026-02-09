use super::SuperpositionsArgs;

pub(in crate::tui_shell::app) fn parse_superpositions_args(
    args: &[String],
) -> Result<SuperpositionsArgs, String> {
    let mut out = SuperpositionsArgs::default();
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
            "--filter" | "filter" => {
                i += 1;
                if i >= args.len() {
                    return Err("missing value for --filter".to_string());
                }
                out.filter = Some(args[i].clone());
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
