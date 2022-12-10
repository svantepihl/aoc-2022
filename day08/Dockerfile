FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc
WORKDIR /app
COPY ./input.txt ./
COPY --from=builder /app/target/release/aoc ./
CMD ["./aoc"]