#!/bin/sh
# Container entrypoint of Breadator.
#
# 1. Fly mounts the volume root-owned: when started as root, give the data directory to the
#    unprivileged user (uid 10001) and re-run this script as that user. Nothing else runs as root.
# 2. With Litestream configured (bucket, endpoint and keys in the environment): restore the
#    database from the replica when the volume is empty (new machine, lost volume), then run the
#    server under `litestream replicate`, which streams every change to the bucket and stops the
#    server on SIGTERM. Without it, start the server alone.
set -eu

DATABASE_PATH="${DATABASE_PATH:-/data/breadator.sqlite3}"
export DATABASE_PATH

if [ "$(id -u)" = 0 ]; then
    mkdir -p "$(dirname "$DATABASE_PATH")"
    chown -R 10001:10001 "$(dirname "$DATABASE_PATH")"
    exec setpriv --reuid=10001 --regid=10001 --init-groups "$0" "$@"
fi

if [ -n "${LITESTREAM_BUCKET:-}" ] && [ -n "${LITESTREAM_ENDPOINT:-}" ] \
    && [ -n "${LITESTREAM_ACCESS_KEY_ID:-}" ] && [ -n "${LITESTREAM_SECRET_ACCESS_KEY:-}" ]; then
    export LITESTREAM_PATH="${LITESTREAM_PATH:-breadator}"
    echo "litestream: replicating $DATABASE_PATH to bucket $LITESTREAM_BUCKET/$LITESTREAM_PATH" >&2
    # Only when the database file is missing and the bucket holds a replica: never overwrites.
    litestream restore -if-db-not-exists -if-replica-exists "$DATABASE_PATH"
    exec litestream replicate -exec "breadator $*"
fi

echo "litestream: not configured (LITESTREAM_BUCKET, LITESTREAM_ENDPOINT, LITESTREAM_ACCESS_KEY_ID, LITESTREAM_SECRET_ACCESS_KEY): the database is not replicated" >&2
exec breadator "$@"
