FROM rust:1.43

WORKDIR /app

COPY . .

RUN rustup default nightly

RUN cargo install --path .

CMD cargo run

