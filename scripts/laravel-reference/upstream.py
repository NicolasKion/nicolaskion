#!/usr/bin/env python3
"""A mock of the GitHub and ESI endpoints the site reads, serving
tests/fixtures/upstream. Used by the Laravel reference (run.sh start) and
by the composed stack in the end-to-end suite (compose.e2e.yml); the Rust
suites have their own mock over the same files
(tests/integration/common/upstream.rs).

    python3 -I scripts/laravel-reference/upstream.py --port 8399

Routes, under /github and /esi:

    GET /github/user, GET /github/user/repos?page=N
    GET /esi/characters/{id}, /esi/corporations/{id}, /esi/alliances/{id}
        (also with Laravel's /latest prefix and trailing slash)

Control routes:

    GET  /__scenario            the current scenario
    POST /__scenario/<name>     switch: normal, paginated, github-down,
                                esi-down, no-alliance
    GET  /__requests            every request since the last reset
    POST /__requests/reset

Only the standard library; run with `python3 -I`.
"""

import argparse
import json
import os
import re
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from urllib.parse import parse_qs, urlsplit

HERE = os.path.dirname(os.path.abspath(__file__))
FIXTURES = os.environ.get(
    "UPSTREAM_FIXTURES",
    os.path.join(os.path.dirname(os.path.dirname(HERE)), "tests/fixtures/upstream"),
)
SCENARIOS = {"normal", "paginated", "github-down", "esi-down", "no-alliance"}

state = {"scenario": os.environ.get("UPSTREAM_SCENARIO", "normal"), "requests": []}
lock = threading.Lock()


def fixture(name):
    with open(os.path.join(FIXTURES, name), encoding="utf-8") as handle:
        return json.load(handle)


def paginated_repositories():
    """101 repositories, as in ProjectsPageTest "fetches repositories across
    multiple pages": the first fixture repository with ids 1 to 101."""
    template = fixture("github/repos.json")[0]
    repositories = []
    for index in range(1, 102):
        repository = dict(template)
        repository["id"] = index
        repository["full_name"] = f"NicolasKion/repo-{index}"
        repositories.append(repository)
    return repositories


def github(path, query, scenario):
    if scenario == "github-down":
        return 500, {"message": "Server Error"}
    if path == "/user":
        return 200, fixture("github/user.json")
    if path == "/user/repos":
        page = int(query.get("page", ["1"])[0])
        per_page = int(query.get("per_page", ["30"])[0])
        repositories = (
            paginated_repositories() if scenario == "paginated" else fixture("github/repos.json")
        )
        return 200, repositories[(page - 1) * per_page : page * per_page]
    return 404, {"message": "Not Found"}


def esi(path, scenario):
    if scenario == "esi-down":
        return 503, {"error": "The service is unavailable"}
    path = re.sub(r"^/latest", "", path).rstrip("/")
    match = re.fullmatch(r"/(characters|corporations|alliances)/(\d+)", path)
    if not match:
        return 404, {"error": "Not found"}
    kind = match.group(1)
    if kind == "characters":
        character = fixture("esi/character.json")
        if scenario == "no-alliance":
            character.pop("alliance_id")
            character.pop("security_status")
            character["birthday"] = "2008-12-31T23:59:59Z"
        return 200, character
    return 200, fixture({"corporations": "esi/corporation.json", "alliances": "esi/alliance.json"}[kind])


class Handler(BaseHTTPRequestHandler):
    def log_message(self, *args):
        pass

    def answer(self, status, body):
        data = json.dumps(body).encode()
        self.send_response(status)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(data)))
        self.end_headers()
        self.wfile.write(data)

    def do_POST(self):
        path = urlsplit(self.path).path
        if path.startswith("/__scenario/"):
            name = path.rsplit("/", 1)[1]
            if name not in SCENARIOS:
                return self.answer(400, {"error": f"unknown scenario {name}"})
            with lock:
                state["scenario"] = name
            return self.answer(200, {"scenario": name})
        if path == "/__requests/reset":
            with lock:
                state["requests"].clear()
            return self.answer(200, {})
        return self.answer(404, {})

    def do_GET(self):
        url = urlsplit(self.path)
        if url.path == "/__scenario":
            return self.answer(200, {"scenario": state["scenario"]})
        if url.path == "/__requests":
            with lock:
                return self.answer(200, list(state["requests"]))
        query = parse_qs(url.query)
        with lock:
            scenario = state["scenario"]
            state["requests"].append(
                {
                    "path": url.path,
                    "query": {key: values[0] for key, values in sorted(query.items())},
                    "accept": self.headers.get("Accept"),
                    "authorization": self.headers.get("Authorization"),
                }
            )
        if url.path.startswith("/github/"):
            return self.answer(*github(url.path[len("/github") :], query, scenario))
        if url.path.startswith("/esi/"):
            return self.answer(*esi(url.path[len("/esi") :], scenario))
        return self.answer(404, {"message": "Not Found"})


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--host", default="127.0.0.1")
    parser.add_argument("--port", type=int, default=8399)
    args = parser.parse_args()
    ThreadingHTTPServer((args.host, args.port), Handler).serve_forever()


if __name__ == "__main__":
    main()
