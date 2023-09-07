FROM rust:1.72 as builder

ENV HOME=/home/root
WORKDIR $HOME/app
# rust toolchain
ARG RUST_TC_VER=stable
ARG BUILD_TARGET=x86_64-unknown-linux-musl
ARG BUILD_PROFILE=release-lto

# import the Nodesource GPG key
RUN apt-get update \
    && apt-get install -y ca-certificates curl gnupg \
    && mkdir -p /etc/apt/keyrings \
    && curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key | gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg

# Create deb repository
# FIXME: Replace it with Nodejs 20.x when it will be fixed
ENV NODE_MAJOR=18
RUN echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_$NODE_MAJOR.x nodistro main" | tee /etc/apt/sources.list.d/nodesource.list

# Install yarn keys
RUN curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add - \ 
    && echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list

RUN apt-get update \
  && apt-get install -y --no-install-recommends musl-tools yarn nodejs \
  && rustup install $RUST_TC_VER \
  && rustup default $RUST_TC_VER \
  && rustup target add $BUILD_TARGET


# RUN USER=root cargo new --bin bob-management
COPY ./Cargo.toml ./Cargo.toml
COPY ./build.rs ./build.rs
COPY ./cli ./cli
COPY ./backend ./backend
COPY ./frontend ./frontend
RUN cargo build --profile=$BUILD_PROFILE --target=$BUILD_TARGET --bin bob_management
RUN find . -name '*.rs' -delete

ADD . ./

RUN rm ./target/$BUILD_TARGET/$BUILD_PROFILE/deps/bob_management* 

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/home/root/app/target \
    cargo build --profile=$BUILD_PROFILE --target=$BUILD_TARGET --bin bob_management \
    && mkdir /build_output \ 
    && mkdir /build_output/backend \ 
    && mkdir /build_output/frontend  \
    && cp -f target/$BUILD_TARGET/$BUILD_PROFILE/bob_management /build_output/backend/bob_management \
    && cp -r -f target/frontend/dist /build_output/frontend/dist

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

COPY --from=builder --chown=$APP_USER:$APP_USER /build_output/backend ${APP}/backend
COPY --from=builder --chown=$APP_USER:$APP_USER /build_output/frontend ${APP}/frontend

USER $APP_USER
WORKDIR ${APP}

CMD ["./backend/bob_management", "--default"]

