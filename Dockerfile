FROM rust:1.43 as build

WORKDIR /build

COPY . .

RUN rustup default nightly

RUN cargo install --path . --root .

CMD ["/build/bin/pants"]
