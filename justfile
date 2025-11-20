build-dashboard:
    pushd dashboard && pnpm install && pnpm run build && popd

build:
    cargo build

check-code: build-dashboard
    cargo check
    cargo fmt --check --all
    cargo clippy --all
    cargo audit
    cargo deny check

e2e: build-dashboard build
    ./scripts/run-e2e.sh

run: build-dashboard
    cargo run --bin nodescope-cli