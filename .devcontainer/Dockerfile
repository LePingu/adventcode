FROM --platform=linux/arm64 rust:1.64-slim-bullseye

RUN apt-get update -yq \
    && apt-get -yq install curl gnupg ca-certificates libssl-dev wget pkg-config \
    && rustup component add clippy && rustup component add rustfmt