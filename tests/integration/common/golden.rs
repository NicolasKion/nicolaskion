//! The golden fixtures recorded from the Laravel reference
//! (scripts/laravel-reference/record.py) and the normalizations every
//! comparison applies. The only accepted differences are listed here and
//! in FIXES.md.

use serde_json::Value;

use super::repo_path;

/// One recorded `GET /` of the Inertia page.
pub struct GoldenPage {
    pub status: u16,
    /// The Inertia component, when the page rendered.
    pub component: Option<String>,
    /// The page props after `normalize`, when the page rendered.
    pub props: Option<Value>,
}

pub fn read_json(relative: &str) -> Value {
    let path = repo_path(&format!("tests/fixtures/golden/{relative}"));
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

pub fn read_text(relative: &str) -> String {
    let path = repo_path(&format!("tests/fixtures/golden/{relative}"));
    std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

pub fn page(name: &str) -> GoldenPage {
    let entry = read_json(&format!("pages/{name}.json"));
    let page = entry.get("page");
    GoldenPage {
        status: entry["status"].as_u64().expect("status") as u16,
        component: page.map(|page| page["component"].as_str().expect("component").to_owned()),
        props: page.map(|page| {
            let mut props = page["props"].clone();
            normalize_recorded(&mut props);
            props
        }),
    }
}

/// Every recorded page, by name.
pub fn page_names() -> Vec<String> {
    let dir = repo_path("tests/fixtures/golden/pages");
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .expect("golden pages")
        .map(|entry| {
            let name = entry.expect("golden entry").file_name();
            name.to_string_lossy().trim_end_matches(".json").to_owned()
        })
        .collect();
    names.sort();
    names
}

/// The requests Laravel sent the mock upstream for a page, after N3.
pub fn requests(name: &str) -> Vec<Value> {
    let mut requests = read_json(&format!("requests/{name}.json"))
        .as_array()
        .expect("requests")
        .clone();
    for request in &mut requests {
        normalize_request(request);
    }
    requests
}

/// A recorded non-Inertia answer (`http/<name>.json`).
pub fn http(name: &str) -> Value {
    read_json(&format!("http/{name}.json"))
}

/// The body of a recorded non-Inertia answer (`http/<name>.html`).
pub fn http_body(name: &str) -> String {
    read_text(&format!("http/{name}.html"))
}

/// The documented normalizations of a recorded page:
/// - N1: Inertia's shared `errors` bag is dropped. Nothing on the page
///   posts a form, so the new props do not carry it.
pub fn normalize_recorded(props: &mut Value) {
    if let Value::Object(map) = props {
        map.shift_remove("errors");
    }
}

/// - N2: the new `githubConfigured` prop is dropped from the api's props
///   before the comparison; its own tests pin it (FIXES.md "A GitHub
///   failure with nothing cached was a 500").
pub fn normalize_actual(props: &mut Value) {
    if let Value::Object(map) = props {
        map.shift_remove("githubConfigured");
    }
}

/// - N3: ESI paths lose Laravel's legacy `/latest` prefix and trailing
///   slash (FIXES.md "ESI requests"), so a recorded
///   `/esi/latest/characters/1/` compares as `/esi/characters/1`.
pub fn normalize_request(request: &mut Value) {
    if let Some(path) = request["path"].as_str()
        && let Some(route) = path.strip_prefix("/esi/latest")
    {
        request["path"] = Value::String(format!("/esi{}", route.trim_end_matches('/')));
    }
}
