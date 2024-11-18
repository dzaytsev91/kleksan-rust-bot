FROM rust:1-bullseye
COPY ./ ./
RUN cargo build --release
CMD ["./target/release/kleksan-rust-bot"]
