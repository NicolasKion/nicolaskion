#!/usr/bin/env bash
# Runs the Laravel reference (the `main` branch) on http://127.0.0.1:8314
# for recording golden fixtures (record.py) and the browser captures
# (capture.mjs), next to the mock upstream (upstream.py) on 127.0.0.1:8399.
#
#   scripts/laravel-reference/run.sh setup   export main, patch, install, build assets
#   scripts/laravel-reference/run.sh start   serve the reference and the mock upstream
#   scripts/laravel-reference/run.sh stop
#
# The checkout lives in $LARAVEL_REF_DIR (default /tmp/nicolaskion-laravel-ref),
# never inside this repository. The only change to main: the GitHub and ESI
# base URLs come from GITHUB_API_URL and ESI_URL, so the reference talks to
# the mock instead of the real APIs. Both containers use the host network so
# they reach each other on 127.0.0.1. Containers are named
# nicolaskion-laravel-* and nicolaskion-upstream-mock.
set -euo pipefail

repo=$(cd "$(dirname "$0")/../.." && pwd)
dir=${LARAVEL_REF_DIR:-/tmp/nicolaskion-laravel-ref}
image=nicolaskion-laravel-ref
port=8314

run() {
  docker run --rm -u "$(id -u):$(id -g)" -e HOME=/tmp -e COMPOSER_HOME=/tmp/composer \
    -v "$dir/app:/app" "$@"
}

case "${1:-}" in
  setup)
    docker build -t "$image" "$repo/scripts/laravel-reference"
    rm -rf "$dir" && mkdir -p "$dir/app"
    git -C "$repo" archive main | tar -x -C "$dir/app"
    sed -i "s#Http::baseUrl('https://api.github.com')#Http::baseUrl(env('GITHUB_API_URL', 'https://api.github.com'))#" \
      "$dir/app/app/Services/GitHub.php"
    sed -i "s#Http::baseUrl('https://esi.evetech.net/latest')#Http::baseUrl(env('ESI_URL', 'https://esi.evetech.net').'/latest')#" \
      "$dir/app/app/Services/Esi.php"
    grep -q GITHUB_API_URL "$dir/app/app/Services/GitHub.php"
    grep -q ESI_URL "$dir/app/app/Services/Esi.php"
    cp "$repo/scripts/laravel-reference/env" "$dir/app/.env"
    run --name nicolaskion-laravel-setup "$image" sh -c '
      composer install --no-interaction --no-progress
      touch database/database.sqlite
      php artisan migrate --force
      npm ci --no-audit --no-fund
      npm run build'
    ;;
  start)
    "$0" stop
    cp "$repo/scripts/laravel-reference/env" "$dir/app/.env"
    rm -f "$dir/app/storage/logs/laravel.log"
    docker run -d --name nicolaskion-upstream-mock --network host \
      -v "$repo:/repo:ro" python:3.14-slim \
      python3 -I /repo/scripts/laravel-reference/upstream.py --port 8399 >/dev/null
    docker run -d --name nicolaskion-laravel-ref -u "$(id -u):$(id -g)" -e HOME=/tmp \
      -e PHP_CLI_SERVER_WORKERS=4 --network host \
      -v "$dir/app:/app" "$image" \
      php artisan serve --host=127.0.0.1 --port="$port" --no-reload >/dev/null
    until curl -fs "http://127.0.0.1:$port/up" >/dev/null; do sleep 0.5; done
    until curl -fs "http://127.0.0.1:8399/__scenario" >/dev/null; do sleep 0.5; done
    echo "Laravel reference on http://127.0.0.1:$port, mock upstream on http://127.0.0.1:8399"
    ;;
  php)
    # Runs a PHP file from this directory inside the reference image.
    shift
    docker run --rm -v "$repo/scripts/laravel-reference:/scripts:ro" "$image" php "/scripts/$1"
    ;;
  stop)
    docker rm -f nicolaskion-laravel-ref nicolaskion-upstream-mock >/dev/null 2>&1 || true
    ;;
  *)
    sed -n '2,15p' "$0"
    exit 1
    ;;
esac
