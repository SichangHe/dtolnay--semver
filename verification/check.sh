#!/bin/sh
set -eu

if [ -z "${VERUS:-}" ]; then
    echo "VERUS must name Verus 0.2025.11.30.840fa61" >&2
    exit 2
fi

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo_dir=$(CDPATH= cd -- "$script_dir/.." && pwd)
cd "$repo_dir"
evidence_dir=${EVIDENCE_DIR:-target/prerelease-verification}
mkdir -p "$evidence_dir"

run_check() {
    name=$1
    shift
    if "$@" >"$evidence_dir/$name.stdout" 2>"$evidence_dir/$name.stderr"; then
        return
    fi
    sed -n '1,240p' "$evidence_dir/$name.stdout" >&2
    sed -n '1,240p' "$evidence_dir/$name.stderr" >&2
    return 1
}

run_check adversarial cargo test --test prerelease_adversarial_search
run_check verus_version sh -c '"$1" --version | grep -F "0.2025.11.30.840fa61"' sh "$VERUS"
run_check verus timeout 120s "$VERUS" --crate-type lib verification/prerelease.rs
run_check assumptions assumls check .
run_check proof_format rustfmt --check verification/prerelease.rs
run_check fmt cargo fmt --check
run_check clippy cargo clippy --tests --benches -- -Dclippy::all -Dclippy::pedantic
run_check no_std cargo check --no-default-features
run_check serde cargo check --features serde
run_check no_std_serde cargo check --no-default-features --features serde
run_check tests cargo test --all-features
