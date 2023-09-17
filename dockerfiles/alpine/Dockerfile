FROM rust:1.72 as backend

ENV HOME=/home/root
WORKDIR $HOME/app
# rust toolchain
ARG RUST_TC_VER=stable
ARG BUILD_TARGET=x86_64-unknown-linux-musl
ARG BUILD_PROFILE=release-lto

RUN apt-get update \
  && apt-get install -y --no-install-recommends musl-tools \
  && rustup install $RUST_TC_VER \
  && rustup default $RUST_TC_VER \
  && rustup target add $BUILD_TARGET


# RUN USER=root cargo new --bin bob-management
COPY ./Cargo.toml ./Cargo.toml
COPY ./build.rs ./build.rs
COPY ./cli ./cli
COPY ./backend ./backend
COPY ./.cargo ./.cargo
RUN cargo backend --profile=$BUILD_PROFILE --target=$BUILD_TARGET

ADD . ./

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/home/root/app/target \
    cargo backend --profile=$BUILD_PROFILE --target=$BUILD_TARGET \
    && mkdir /build_output \ 
    && cp -f target/$BUILD_TARGET/$BUILD_PROFILE/backend /build_output/backend 

FROM node:20.6 as frontend

COPY ./frontend ./frontend

RUN cd frontend && yarn && yarn build

FROM alpine:3.18
ARG APP=/usr/src/app
ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN addgroup -S $APP_USER \
    && adduser -S -g $APP_USER $APP_USER \
    && apk update \
    && apk add --no-cache ca-certificates tzdata \
    && rm -rf /var/cache/apk/* 

EXPOSE 9000

COPY --from=backend --chown=$APP_USER:$APP_USER /build_output/backend ${APP}/backend/backend
COPY --from=frontend --chown=$APP_USER:$APP_USER /frontend/dist ${APP}/frontend

USER $APP_USER
WORKDIR ${APP}

CMD ["./backend/backend"]

