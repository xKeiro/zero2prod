watch:
    cargo install --locked bunyan
    cargo watch -x "test | bunyan" -x "run | bunyan"
unused:
    cargo +nightly install cargo-udeps
    cargo +nightly udeps
fix:
    cargo clippy --fix --allow-dirty --allow-staged && cargo fmt --all