FROM rust:latest AS builder
RUN adduser \

  --disabled-password \
  --gecos "" \
  --home "/nonexistent" \
  --shell "/sbin/nologin" \
  --no-create-home \
  --uid 65532 \
  small-user

WORKDIR /app
COPY ./ ./
RUN cargo build --release
RUN strip target/release/kleksan-rust-bot
##########################
FROM scratch

COPY --from=builder /usr/share/zoneinfo /usr/share/zoneinfo
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
COPY --from=builder /app/target/release/kleksan-rust-bot .

USER small-user:small-user
CMD ["./kleksan-rust-bot"]