# BUILD
FROM rust:1.89.0 AS builder
WORKDIR /app
ADD . /app
RUN cargo build --release

# PROD
EXPOSE 1946
FROM gcr.io/distroless/cc
COPY --from=builder /app/static /
COPY --from=builder /app/.env /
COPY --from=builder /app/target/release/fang_rs /
CMD ["./fang_rs"]
