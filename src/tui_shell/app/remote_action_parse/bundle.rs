use super::BundleArgs;

pub(in crate::tui_shell::app) fn parse_bundle_args(args: &[String]) -> Result<BundleArgs, String> {
    let mut out = BundleArgs::default();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--scope" => {
                i += 1;
                if i >= args.len() {
                    return Err("missing value for --scope".to_string());
                }
                out.scope = Some(args[i].clone());
            }
            "--gate" => {
                i += 1;
                if i >= args.len() {
                    return Err("missing value for --gate".to_string());
                }
                out.gate = Some(args[i].clone());
            }
            "--publication" => {
                i += 1;
                if i >= args.len() {
                    return Err("missing value for --publication".to_string());
                }
                out.publications.push(args[i].clone());
            }
            arg => return Err(format!("unknown arg: {}", arg)),
        }
        i += 1;
    }
    Ok(out)
}
