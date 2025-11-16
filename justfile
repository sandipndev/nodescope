check-code:
    cargo check
    cargo fmt --check --all
    cargo clippy --all
    cargo audit
    cargo deny check

build:
    pushd dashboard && pnpm install && pnpm run build && popd
    cargo build

e2e: build
    ./scripts/run-e2e.sh
