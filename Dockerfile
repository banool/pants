FROM rust:1.43 as build

WORKDIR /app

COPY . .

RUN rustup default nightly

RUN cargo build --release

FROM alpine:latest

WORKDIR /app

COPY --from=build /app/target/release/pants /app/pants

CMD ["pants"]
