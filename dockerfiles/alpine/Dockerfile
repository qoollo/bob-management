FROM rust:1.72 as backend

ARG GIT_HASH_VAR
ENV BOBGUI_GIT_HASH $GIT_HASH_VAR
ARG BRANCH_TAG_VAR
ENV BOBGUI_BUILD_BRANCH_TAG $BRANCH_TAG_VAR

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

# estimate build directory
RUN echo "$(case "$BUILD_PROFILE" in\
  ("dev") echo "debug";;\
  ("test") echo "debug";;\
  ("bench") echo "release";;\
  (*) echo "$BUILD_PROFILE";;\
  esac)" >> ./build_profile_dir

RUN mkdir -p backend/src frontend cli/src utils/src
RUN mkdir target
COPY Cargo.toml Cargo.toml
COPY cli/Cargo.toml cli/Cargo.toml
COPY backend/Cargo.toml backend/Cargo.toml
COPY utils/Cargo.toml utils/Cargo.toml
COPY frontend/Cargo.toml frontend/Cargo.toml
COPY .cargo .cargo
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > backend/src/lib.rs \
  && echo "fn main() {println!(\"if you see this, the build broke\")}" > backend/src/main.rs \
  && echo "fn main() {println!(\"if you see this, the build broke\")}" > frontend/build.rs \
  && echo "fn main() {println!(\"if you see this, the build broke\")}" > cli/src/lib.rs \
  && echo "fn main() {println!(\"if you see this, the build broke\")}" > utils/src/lib.rs \
  && echo "fn main() {println!(\"if you see this, the build broke\")}" > build.rs \
  && cargo build-backend --profile=$BUILD_PROFILE --target=$BUILD_TARGET

COPY . ./

RUN cargo build-backend --profile=$BUILD_PROFILE --target=$BUILD_TARGET \
    && mkdir /build_output \ 
    && cp -f target/$BUILD_TARGET/$(cat ./build_profile_dir)/bob-management /build_output/bob-management

FROM node:20.6 as frontend

COPY ./frontend ./frontend

RUN cd frontend && yarn && yarn build && mkdir /build_output && cp -r ./frontend /build_output/frontend

FROM alpine:3.18
ARG APP=/home/bob-management
ENV TZ=Etc/UTC \
    APP_USER=bobm
ENV PATH="$PATH:${APP}"
RUN addgroup -S $APP_USER \
    && adduser -S -g $APP_USER $APP_USER \
    && apk update \
    && apk add --no-cache ca-certificates tzdata \
    && rm -rf /var/cache/apk/* 

EXPOSE 9000

COPY --from=backend --chown=$APP_USER:$APP_USER /build_output/bob-management ${APP}/bob-management
COPY --from=frontend --chown=$APP_USER:$APP_USER /build_output/frontend ${APP}/frontend

USER $APP_USER
WORKDIR ${APP}

ENTRYPOINT ["./bob-management"]
CMD ["--default"]

