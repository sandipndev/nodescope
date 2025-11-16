check-code: build-dashboard
    cargo check
    cargo fmt --check --all
    cargo clippy --all
    cargo audit
    cargo deny check

build-dashboard:
    pushd dashboard && pnpm install && pnpm run build && popd

build:
    cargo build

e2e: build
    ./scripts/run-e2e.sh
