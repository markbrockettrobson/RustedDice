FROM rust:1.61.0

COPY . /usr/src/RustedDice
WORKDIR /usr/src/RustedDice/rusted_dice

RUN rustup component add clippy
RUN cargo install --path .
RUN cargo build 
RUN cargo clippy --all-targets --all-features -- -D warnings
RUN cargo test