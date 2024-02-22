FROM rust:1

WORKDIR /app
COPY . .
RUN cargo build --release
CMD ["target/release/cozy"]