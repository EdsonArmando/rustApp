FROM rust:1.54.0
ENV DATABASE_URL         = mongodb://mongoadmin:Arqui2022_2022@localhost:27017
ENV DATABASE_NAME        = Test
ENV USER_COLLECTION_NAME = Juego
ENV PORT                 = 2000
ENV HOST                 = 0.0.0.0

WORKDIR /app

COPY . .

RUN cargo build --release

EXPOSE 2000

ENTRYPOINT ["./target/release/rust-web-mongodb-fase2"]