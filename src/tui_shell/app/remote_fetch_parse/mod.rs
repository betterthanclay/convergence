mod parse;
mod validate;

#[derive(Debug, Default)]
pub(super) struct FetchSpec {
    pub(super) snap_id: Option<String>,
    pub(super) bundle_id: Option<String>,
    pub(super) release: Option<String>,
    pub(super) lane: Option<String>,
    pub(super) user: Option<String>,
    pub(super) restore: bool,
    pub(super) into: Option<String>,
    pub(super) force: bool,
}

pub(super) fn parse_fetch_spec(args: &[String]) -> Result<FetchSpec, String> {
    let mut parsed = parse::parse_tokens(args)?;
    parse::apply_shorthands(&mut parsed)?;
    validate::validate_target_selection(&parsed.spec)?;
    Ok(parsed.spec)
}

#[cfg(test)]
mod tests;
