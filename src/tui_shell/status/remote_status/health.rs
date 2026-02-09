pub(super) fn fetch_healthz(base_url: &str) -> String {
    let url = format!("{}/healthz", base_url.trim_end_matches('/'));
    let start = std::time::Instant::now();
    match reqwest::blocking::get(&url) {
        Ok(response) => {
            let ms = start.elapsed().as_millis();
            format!("{} {}ms", response.status(), ms)
        }
        Err(err) => format!("error {:#}", err),
    }
}
