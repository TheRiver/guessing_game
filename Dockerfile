FROM rust:1.54 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM ubuntu:21.04
COPY --from=builder /usr/local/cargo/bin/guessing_game /usr/local/bin/guessing_game
CMD ["guessing_game"]