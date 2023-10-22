FROM rust:1.68-bookworm as build

RUN apt-get update \
    && apt-get install -y cmake protobuf-compiler
RUN USER=root cargo new --bin nostr-relay-auth-server

WORKDIR /nostr-relay-auth-server
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
COPY ./proto ./proto
COPY ./build.rs ./build.rs

# RUN rm ./target/release/deps/nostr-relay-auth-server*
RUN cargo build --release

FROM debian:bookworm-slim as run

ARG APP=/usr/src/app

EXPOSE 50051

ENV APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=build /nostr-relay-auth-server/target/release/nostr-relay-auth-server ${APP}/nostr-relay-auth-server

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

ENTRYPOINT [ "./nostr-relay-auth-server" ]