FROM rust:1.61 as build

RUN USER=root cargo new --bin w2g_discord_bot
WORKDIR /w2g_discord_bot

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./.env ./.env

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/w2g_discord_bot*
RUN cargo build --release

FROM rust:1.61-slim-buster

COPY --from=build /w2g_discord_bot/target/release/w2g_discord_bot .
COPY --from=build /w2g_discord_bot/.env .

CMD ["./w2g_discord_bot"]