# Start with a Rust alpine image.
FROM rust:1.54-alpine3.14 as build

# Copy in the code.
WORKDIR /build
COPY . .

# Add necessary dependencies.
RUN apk add --no-cache openssl openssl-dev musl-dev

# Set compiler to nightly.
RUN rustup default nightly

# Build the binary, strip it to make it smaller.
RUN cargo build --release
RUN strip target/release/pants

# Make a new build stage.
FROM alpine:3.14

# Copy in the binary from the previous build stage.
COPY --from=build /build/static static
COPY --from=build /build/templates templates
COPY --from=build /build/target/release/pants .

# Run it.
ENTRYPOINT ["/pants"]
