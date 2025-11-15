check-code:
    cargo check
    cargo fmt --check --all
    cargo clippy --all
    cargo audit
    cargo deny check
