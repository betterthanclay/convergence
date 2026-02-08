#[derive(Debug, Default)]
pub(super) struct ScopeQueryArgs {
    pub(super) scope: Option<String>,
    pub(super) gate: Option<String>,
    pub(super) limit: Option<usize>,
    pub(super) filter: Option<String>,
}

pub(super) fn parse_scope_query_args(args: &[String]) -> Result<ScopeQueryArgs, String> {
    let mut out = ScopeQueryArgs::default();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--scope" | "scope" => {
                i += 1;
                if i >= args.len() {
                    return Err("missing value for --scope".to_string());
                }
                out.scope = Some(args[i].clone());
            }
            "--gate" | "gate" => {
                i += 1;
                if i >= args.len() {
                    return Err("missing value for --gate".to_string());
                }
                out.gate = Some(args[i].clone());
            }
            "--limit" | "limit" => {
                i += 1;
                if i >= args.len() {
                    return Err("missing value for --limit".to_string());
                }
                out.limit = match args[i].parse::<usize>() {
                    Ok(n) => Some(n),
                    Err(_) => return Err("invalid --limit".to_string()),
                };
            }
            "--filter" | "filter" => {
                i += 1;
                if i >= args.len() {
                    return Err("missing value for --filter".to_string());
                }
                out.filter = Some(args[i].clone());
            }
            a => {
                return Err(format!("unknown arg: {}", a));
            }
        }
        i += 1;
    }

    Ok(out)
}
