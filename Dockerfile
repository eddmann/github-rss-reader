FROM clux/muslrust as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM alpine/git
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/github-rss-reader /
COPY entrypoint.sh /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
