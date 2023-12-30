FROM rust:1.74 as build

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

FROM debian:bookworm-slim
ENV DEBIAN_FRONTEND=noninteractive
RUN apt update -y
RUN apt install -y libssl3 ca-certificates
COPY --from=build /w2g_discord_bot/target/release/w2g_discord_bot .
COPY --from=build /w2g_discord_bot/.env .
CMD ["./w2g_discord_bot"]