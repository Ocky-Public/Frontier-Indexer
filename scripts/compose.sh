#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
ENV_FILE="$ROOT_DIR/.env"
ENV_SAMPLE_FILE="$ROOT_DIR/.env.sample"

usage() {
    cat <<'EOF'
Usage: scripts/compose.sh <command> [args...]

Commands:
  config       Validate the compose configuration
  up           Start the stack with build support
  down         Stop the stack and remove orphans
  logs         Follow compose logs
  smoke-test   Start the stack, verify database and metrics, then tear down

Environment:
  CONTAINER_RUNTIME   docker | podman | podman-compose
  KEEP_STACK_RUNNING  Set to 1 to skip teardown after smoke-test
EOF
}

detect_runtime() {
    if command -v docker >/dev/null 2>&1 && docker compose version >/dev/null 2>&1; then
        printf '%s\n' docker
        return
    fi

    if command -v podman >/dev/null 2>&1 && podman compose version >/dev/null 2>&1; then
        printf '%s\n' podman
        return
    fi

    if command -v podman-compose >/dev/null 2>&1; then
        printf '%s\n' podman-compose
        return
    fi

    printf '%s\n' "No supported compose runtime found. Install docker compose, podman compose, or podman-compose." >&2
    exit 1
}

RUNTIME=${CONTAINER_RUNTIME:-$(detect_runtime)}

case "$RUNTIME" in
    docker)
        COMPOSE_CMD=(docker compose)
        ;;
    podman)
        COMPOSE_CMD=(podman compose)
        ;;
    podman-compose)
        COMPOSE_CMD=(podman-compose)
        ;;
    *)
        printf '%s\n' "Unsupported CONTAINER_RUNTIME: $RUNTIME" >&2
        exit 1
        ;;
esac

run_compose() {
    local compose_env_file="$ENV_FILE"

    if [[ ! -f "$compose_env_file" ]]; then
        compose_env_file="$ENV_SAMPLE_FILE"
    fi

    (cd "$ROOT_DIR" && INDEXER_ENV_FILE="$compose_env_file" "${COMPOSE_CMD[@]}" "$@")
}

ensure_env_file() {
    if [[ -f "$ENV_FILE" ]]; then
        return
    fi

    if [[ ! -f "$ENV_SAMPLE_FILE" ]]; then
        printf '%s\n' "Missing $ENV_FILE and $ENV_SAMPLE_FILE; cannot run compose." >&2
        exit 1
    fi

    cp "$ENV_SAMPLE_FILE" "$ENV_FILE"
    printf '%s\n' "Created .env from .env.sample for compose." >&2
}

wait_for_db() {
    local attempt=0

    until run_compose exec -T db pg_isready -U postgres -d postgres >/dev/null 2>&1; do
        attempt=$((attempt + 1))
        if (( attempt > 60 )); then
            printf '%s\n' "Database did not become ready in time." >&2
            return 1
        fi
        sleep 1
    done
}

wait_for_metrics() {
    local attempt=0

    until curl -fsS http://127.0.0.1:9184/metrics >/dev/null 2>&1; do
        attempt=$((attempt + 1))
        if (( attempt > 60 )); then
            printf '%s\n' "Metrics endpoint did not become ready in time." >&2
            return 1
        fi
        sleep 1
    done
}

verify_schema() {
    local schema_present
    local migrations_present

    schema_present=$(run_compose exec -T db psql -U postgres -d postgres -Atc "SELECT schema_name FROM information_schema.schemata WHERE schema_name = 'indexer'")
    migrations_present=$(run_compose exec -T db psql -U postgres -d postgres -Atc "SELECT to_regclass('indexer.__diesel_schema_migrations') IS NOT NULL")

    [[ "$schema_present" == "indexer" ]]
    [[ "$migrations_present" == "t" ]]
}

print_failure_logs() {
    printf '%s\n' "Smoke test failed. Recent compose logs:" >&2
    run_compose logs --tail=100 >&2 || true
}

smoke_test() {
    ensure_env_file
    run_compose config >/dev/null

    if [[ "${KEEP_STACK_RUNNING:-0}" != "1" ]]; then
        trap 'run_compose down --remove-orphans --volumes' EXIT
    fi

    run_compose down --remove-orphans --volumes >/dev/null 2>&1 || true
    run_compose up --build -d
    wait_for_db
    wait_for_metrics
    verify_schema
}

if [[ $# -lt 1 ]]; then
    usage
    exit 1
fi

COMMAND=$1
shift

case "$COMMAND" in
    config)
        ensure_env_file
        run_compose config "$@"
        ;;
    up)
        ensure_env_file
        run_compose up --build "$@"
        ;;
    down)
        run_compose down --remove-orphans "$@"
        ;;
    logs)
        run_compose logs -f "$@"
        ;;
    smoke-test)
        trap print_failure_logs ERR
        smoke_test
        ;;
    *)
        usage
        exit 1
        ;;
esac