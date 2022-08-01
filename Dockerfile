FROM rust:1.62-alpine

WORKDIR /usr/src/plasma-simulation

COPY ./Cargo.toml .
COPY ./Cargo.lock .

RUN mkdir ./src \
    && echo 'fn main() { println!("Build for dependency caching."); }' > src/main.rs \
    && cargo build --release

RUN rm -rf ./src ./target

COPY . .

RUN cargo install --path .

CMD ["plasma-simulation"]
