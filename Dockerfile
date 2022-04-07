FROM ekidd/rust-musl-builder:stable as builder

WORKDIR /app

COPY . /app
RUN cargo build --release

CMD cargo run
