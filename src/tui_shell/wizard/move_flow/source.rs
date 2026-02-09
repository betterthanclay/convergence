use super::super::super::App;
use super::glob_search;
use super::prompts;

pub(super) fn move_wizard_from(app: &mut App, value: String) {
    let Some(ws) = app.require_workspace() else {
        return;
    };
    let Some(w) = app.move_wizard.as_mut() else {
        app.push_error("move wizard not active".to_string());
        return;
    };

    let raw = value.trim().to_string();
    if raw.is_empty() {
        app.push_error("missing from glob".to_string());
        return;
    }

    if !w.candidates.is_empty()
        && raw.chars().all(|c| c.is_ascii_digit())
        && let Ok(n) = raw.parse::<usize>()
        && n >= 1
        && n <= w.candidates.len()
    {
        let from = w.candidates[n - 1].clone();
        w.candidates.clear();
        w.query = Some(raw);
        w.from = Some(from.clone());
        prompts::open_to_prompt(app, from);
        return;
    }

    w.query = Some(raw.clone());
    let matches = match glob_search(&ws.root, &raw) {
        Ok(m) => m,
        Err(err) => {
            w.candidates.clear();
            prompts::open_from_error_prompt(app, raw, format!("{:#}", err));
            return;
        }
    };

    if matches.is_empty() {
        w.candidates.clear();
        prompts::open_no_matches_prompt(app, raw);
        return;
    }

    if matches.len() == 1 {
        let from = matches[0].clone();
        w.candidates.clear();
        w.from = Some(from.clone());
        prompts::open_to_prompt(app, from);
        return;
    }

    w.candidates = matches.clone();
    prompts::open_candidates_prompt(app, raw, &matches);
}
