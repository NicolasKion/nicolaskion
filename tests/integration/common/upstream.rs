//! A mock of the GitHub and ESI endpoints on an ephemeral port, serving
//! tests/fixtures/upstream like scripts/upstream-mock.py does for the
//! composed stack. Records every request and fails on demand.

use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use axum::Router;
use axum::extract::{Query, State};
use axum::http::{HeaderMap, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use serde_json::{Value, json};

use super::repo_path;

/// One request the mock received.
#[derive(Debug, Clone, PartialEq)]
pub struct RecordedRequest {
    pub path: String,
    pub query: BTreeMap<String, String>,
    pub accept: Option<String>,
    pub authorization: Option<String>,
    pub user_agent: Option<String>,
    pub compatibility_date: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Service {
    GitHub,
    Esi,
}

/// What the mock answers. Starts as the fixture files.
#[derive(Debug, Clone)]
pub struct Data {
    pub user: Value,
    pub repositories: Vec<Value>,
    pub character: Value,
    pub corporation: Value,
    pub alliance: Value,
}

#[derive(Debug, Default)]
struct Failure {
    status: Option<u16>,
    /// Fail only this many more requests, then answer normally.
    remaining: Option<usize>,
}

struct Inner {
    data: Data,
    github: Failure,
    esi: Failure,
    /// Raw bodies for other paths (`/files/...`), for the logo sync.
    files: BTreeMap<String, (u16, Vec<u8>)>,
    requests: Vec<RecordedRequest>,
}

#[derive(Clone)]
pub struct MockUpstream {
    addr: SocketAddr,
    inner: Arc<Mutex<Inner>>,
}

pub fn fixture(name: &str) -> Value {
    let path = repo_path(&format!("tests/fixtures/upstream/{name}"));
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

impl Data {
    pub fn fixtures() -> Self {
        Self {
            user: fixture("github/user.json"),
            repositories: fixture("github/repos.json")
                .as_array()
                .expect("repositories")
                .clone(),
            character: fixture("esi/character.json"),
            corporation: fixture("esi/corporation.json"),
            alliance: fixture("esi/alliance.json"),
        }
    }
}

impl MockUpstream {
    pub async fn start() -> Self {
        let inner = Arc::new(Mutex::new(Inner {
            data: Data::fixtures(),
            github: Failure::default(),
            esi: Failure::default(),
            files: BTreeMap::new(),
            requests: Vec::new(),
        }));
        let app = Router::new().fallback(answer).with_state(inner.clone());
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("mock binds");
        let addr = listener.local_addr().expect("mock address");
        tokio::spawn(async move {
            axum::serve(listener, app).await.expect("mock serves");
        });
        Self { addr, inner }
    }

    pub fn github_url(&self) -> String {
        format!("http://{}/github", self.addr)
    }

    pub fn esi_url(&self) -> String {
        format!("http://{}/esi", self.addr)
    }

    /// A URL the mock answers with a file set by `serve_file`.
    pub fn file_url(&self, name: &str) -> String {
        format!("http://{}/files/{name}", self.addr)
    }

    pub fn serve_file(&self, name: &str, status: u16, body: &[u8]) {
        self.inner
            .lock()
            .expect("mock lock")
            .files
            .insert(format!("/files/{name}"), (status, body.to_vec()));
    }

    pub fn change(&self, change: impl FnOnce(&mut Data)) {
        change(&mut self.inner.lock().expect("mock lock").data);
    }

    /// Answer every request to `service` with `status` from now on.
    pub fn fail_with(&self, service: Service, status: u16) {
        self.fail_times(service, status, None);
    }

    /// Answer the next `times` requests to `service` with `status`.
    pub fn fail_next(&self, service: Service, status: u16, times: usize) {
        self.fail_times(service, status, Some(times));
    }

    fn fail_times(&self, service: Service, status: u16, remaining: Option<usize>) {
        let mut inner = self.inner.lock().expect("mock lock");
        let failure = match service {
            Service::GitHub => &mut inner.github,
            Service::Esi => &mut inner.esi,
        };
        *failure = Failure {
            status: Some(status),
            remaining,
        };
    }

    pub fn recover(&self, service: Service) {
        let mut inner = self.inner.lock().expect("mock lock");
        match service {
            Service::GitHub => inner.github = Failure::default(),
            Service::Esi => inner.esi = Failure::default(),
        }
    }

    pub fn requests(&self) -> Vec<RecordedRequest> {
        self.inner.lock().expect("mock lock").requests.clone()
    }

    pub fn reset_requests(&self) {
        self.inner.lock().expect("mock lock").requests.clear();
    }

    /// The requests to one service.
    pub fn requests_to(&self, service: Service) -> Vec<RecordedRequest> {
        let prefix = match service {
            Service::GitHub => "/github/",
            Service::Esi => "/esi/",
        };
        self.requests()
            .into_iter()
            .filter(|request| request.path.starts_with(prefix))
            .collect()
    }

    /// The scenarios scripts/upstream-mock.py offers too.
    pub fn scenario(&self, name: &str) {
        match name {
            "normal" => {}
            "paginated" => self.change(|data| {
                let template = data.repositories[0].clone();
                data.repositories = (1..=101)
                    .map(|index| {
                        let mut repository = template.clone();
                        repository["id"] = json!(index);
                        repository["full_name"] = json!(format!("NicolasKion/repo-{index}"));
                        repository
                    })
                    .collect();
            }),
            "github-down" => self.fail_with(Service::GitHub, 500),
            "esi-down" => self.fail_with(Service::Esi, 503),
            "no-alliance" => self.change(|data| {
                let character = data.character.as_object_mut().expect("character");
                character.remove("alliance_id");
                character.remove("security_status");
                character.insert("birthday".to_owned(), json!("2008-12-31T23:59:59Z"));
            }),
            other => panic!("unknown scenario {other}"),
        }
    }
}

fn header(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned)
}

async fn answer(
    State(inner): State<Arc<Mutex<Inner>>>,
    uri: Uri,
    Query(query): Query<BTreeMap<String, String>>,
    headers: HeaderMap,
) -> Response {
    let mut inner = inner.lock().expect("mock lock");
    let path = uri.path().to_owned();
    inner.requests.push(RecordedRequest {
        path: path.clone(),
        query: query.clone(),
        accept: header(&headers, "accept"),
        authorization: header(&headers, "authorization"),
        user_agent: header(&headers, "user-agent"),
        compatibility_date: header(&headers, "x-compatibility-date"),
    });

    if let Some((status, body)) = inner.files.get(&path) {
        return (StatusCode::from_u16(*status).expect("status"), body.clone()).into_response();
    }

    let service = if path.starts_with("/github/") {
        Service::GitHub
    } else if path.starts_with("/esi/") {
        Service::Esi
    } else {
        return (StatusCode::NOT_FOUND, "no such mock path").into_response();
    };
    let failure = match service {
        Service::GitHub => &mut inner.github,
        Service::Esi => &mut inner.esi,
    };
    if let Some(status) = failure.status {
        let fail = match &mut failure.remaining {
            None => true,
            Some(0) => false,
            Some(remaining) => {
                *remaining -= 1;
                true
            }
        };
        if fail {
            return (
                StatusCode::from_u16(status).expect("status"),
                axum::Json(json!({ "message": "mock failure" })),
            )
                .into_response();
        }
    }

    let data = &inner.data;
    let body = match service {
        Service::GitHub => match &path["/github".len()..] {
            "/user" => data.user.clone(),
            "/user/repos" => {
                let page: usize = query.get("page").and_then(|v| v.parse().ok()).unwrap_or(1);
                let per_page: usize = query
                    .get("per_page")
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(30);
                Value::Array(
                    data.repositories
                        .iter()
                        .skip((page - 1) * per_page)
                        .take(per_page)
                        .cloned()
                        .collect(),
                )
            }
            _ => return (StatusCode::NOT_FOUND, "no such GitHub route").into_response(),
        },
        Service::Esi => {
            let route = path["/esi".len()..]
                .trim_start_matches("/latest")
                .trim_end_matches('/');
            let mut parts = route.trim_start_matches('/').split('/');
            match (
                parts.next(),
                parts.next().and_then(|id| id.parse::<i64>().ok()),
            ) {
                (Some("characters"), Some(_)) => data.character.clone(),
                (Some("corporations"), Some(_)) => data.corporation.clone(),
                (Some("alliances"), Some(_)) => data.alliance.clone(),
                _ => return (StatusCode::NOT_FOUND, "no such ESI route").into_response(),
            }
        }
    };
    axum::Json(body).into_response()
}
