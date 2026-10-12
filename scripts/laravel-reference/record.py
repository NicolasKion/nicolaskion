#!/usr/bin/env python3
"""Records golden fixtures from the running Laravel reference.

    scripts/laravel-reference/run.sh start
    python3 -I scripts/laravel-reference/record.py

Writes tests/fixtures/golden/:

- pages/<scenario>.json: the status and the Inertia page object of `GET /`
  (`X-Inertia: true` with the asset version) for each upstream scenario
  and configuration (token and character set or not).
- requests/<scenario>.json: what Laravel asked the mock upstream for while
  rendering that page (path, query, Accept, Authorization).
- http/<name>.json and .html: the non-Inertia answers (the health page,
  404, the dropped storage routes) and the full HTML document of `/`.
- php-functions.json: PHP's answers for the functions the port depends on
  (php-functions.php).

The Laravel cache store is `array` (env), so every request asks the mock
afresh. The reference restarts for each configuration. Only the standard library; run with `python3 -I`.
"""

import json
import os
import re
import subprocess
import sys
import time
import urllib.error
import urllib.request

BASE = "http://127.0.0.1:8314"
UPSTREAM = "http://127.0.0.1:8399"
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.dirname(os.path.dirname(HERE))
OUT = os.path.join(REPO, "tests/fixtures/golden")
REF_DIR = os.environ.get("LARAVEL_REF_DIR", "/tmp/nicolaskion-laravel-ref")
ENV_FILE = os.path.join(REF_DIR, "app/.env")

TOKEN = "golden-token"
CHARACTER = "95569887"

# name: (GITHUB_TOKEN, EVE_CHARACTER_ID, upstream scenario)
SCENARIOS = {
    "unconfigured": ("", "", "normal"),
    "blank-config": ("'   '", "' '", "normal"),
    "full": (TOKEN, CHARACTER, "normal"),
    "no-character": (TOKEN, "", "normal"),
    "no-alliance": (TOKEN, CHARACTER, "no-alliance"),
    "esi-down": (TOKEN, CHARACTER, "esi-down"),
    "paginated": (TOKEN, "", "paginated"),
    "github-down": (TOKEN, CHARACTER, "github-down"),
}


class NoRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, *args, **kwargs):
        return None


def request(url, method="GET", headers=None, body=None):
    req = urllib.request.Request(url, method=method, headers=headers or {}, data=body)
    try:
        with urllib.request.build_opener(NoRedirect()).open(req) as response:
            return response.status, response.headers, response.read()
    except urllib.error.HTTPError as error:
        return error.code, error.headers, error.read()


def upstream(path, method="GET"):
    _, _, body = request(UPSTREAM + path, method=method, body=b"" if method == "POST" else None)
    return json.loads(body)


def write(relative, data):
    path = os.path.join(OUT, relative)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        if isinstance(data, str):
            handle.write(data)
        else:
            # Key order is kept: the props are compared in Laravel's order.
            json.dump(data, handle, indent=2, ensure_ascii=False)
            handle.write("\n")


def configure(token, character):
    with open(ENV_FILE, encoding="utf-8") as handle:
        env = handle.read()
    env = re.sub(r"^GITHUB_TOKEN=.*$", f"GITHUB_TOKEN={token}", env, flags=re.M)
    env = re.sub(r"^EVE_CHARACTER_ID=.*$", f"EVE_CHARACTER_ID={character}", env, flags=re.M)
    with open(ENV_FILE, "w", encoding="utf-8") as handle:
        handle.write(env)
    # `artisan serve` hands the environment it read at start to its
    # workers, so a changed .env needs a restart.
    subprocess.run(["docker", "restart", "nicolaskion-laravel-ref"], check=True, capture_output=True)
    for _ in range(100):
        try:
            if request(BASE + "/up")[0] == 200:
                return
        except OSError:
            pass
        time.sleep(0.2)
    raise SystemExit("the reference did not come back after the restart")


def inertia_version():
    status, headers, _ = request(BASE + "/", headers={"X-Inertia": "true"})
    assert status == 409, status
    return headers["X-Inertia-Version"]


def record_page(name, version):
    headers = {"X-Inertia": "true", "X-Inertia-Version": version, "X-Requested-With": "XMLHttpRequest"}
    upstream("/__requests/reset", "POST")
    status, response_headers, body = request(BASE + "/", headers=headers)
    entry = {"path": "/", "status": status}
    if status == 200:
        entry["page"] = json.loads(body)
    else:
        entry["content_type"] = response_headers.get("Content-Type")
    write(f"pages/{name}.json", entry)
    write(f"requests/{name}.json", upstream("/__requests"))
    print(f"page {name}: {status}")


def record_http(name, path, method="GET"):
    status, headers, body = request(BASE + path, method=method, body=b"" if method == "PUT" else None)
    entry = {
        "path": path,
        "method": method,
        "status": status,
        "content_type": headers.get("Content-Type"),
        "location": headers.get("Location"),
        "cache_control": headers.get("Cache-Control"),
        "link": headers.get("Link"),
        "cookies": sorted(cookie.split("=", 1)[0] for cookie in headers.get_all("Set-Cookie") or []),
    }
    write(f"http/{name}.json", entry)
    if body:
        write(f"http/{name}.html", body.decode(errors="replace"))
    print(f"http {name}: {status}")


def record_php_functions():
    output = subprocess.run(
        [os.path.join(HERE, "run.sh"), "php", "php-functions.php"],
        check=True,
        capture_output=True,
        text=True,
    ).stdout
    write("php-functions.json", json.loads(output))
    print("php-functions")


def main():
    version = inertia_version()
    try:
        for name, (token, character, scenario) in SCENARIOS.items():
            configure(token, character)
            upstream(f"/__scenario/{scenario}", "POST")
            record_page(name, version)

        configure("", "")
        upstream("/__scenario/normal", "POST")
        record_http("home", "/")
        record_http("up", "/up")
        record_http("unknown-path", "/nope")
        record_http("storage-get", "/storage/tt-8d20886697466b8297bb6d6e1735fa53/missing.json")
        record_http("storage-put", "/storage/upload.txt", "PUT")
        record_http("trailing-slash", "/up/")
    finally:
        configure("", "")
        upstream("/__scenario/normal", "POST")
    record_php_functions()


if __name__ == "__main__":
    sys.exit(main())
