# Build the compiler, then ship only the static binary.
#   docker build -t lingua .
#   docker run --rm -v "$PWD":/src -w /src lingua check money.lf
FROM rust:1.99.1-bookworm AS build
WORKDIR /src
COPY . .
RUN cargo build --release --locked

FROM scratch
COPY --from=build /src/target/release/lingua /lingua
ENTRYPOINT ["/lingua"]
