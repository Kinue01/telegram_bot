FROM rust
COPY . /app/
WORKDIR /app/
RUN cargo build --release
ENTRYPOINT /app/target/release/telegram_bot