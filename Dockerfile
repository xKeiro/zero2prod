FROM clux/muslrust:stable AS planner
RUN cargo install cargo-chef
COPY Cargo.* .
RUN cargo chef prepare --recipe-path recipe.json


FROM clux/muslrust:stable AS cacher
RUN cargo install cargo-chef
COPY --from=planner /volume/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json


FROM clux/muslrust:stable AS builder
COPY . .
COPY --from=cacher /volume/target target
COPY --from=cacher /root/.cargo /root/.cargo
ENV SQLX_OFFLINE true
RUN cargo build --bin zero2prod --release --target x86_64-unknown-linux-musl


# Need cacerts
FROM gcr.io/distroless/static:nonroot
COPY --from=builder --chown=nonroot:nonroot /volume/target/x86_64-unknown-linux-musl/release/zero2prod /app/zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["/app/zero2prod"]