FROM rust:1.92-alpine

WORKDIR /usr/src/plasma-simulation

COPY ./Cargo.toml .
COPY ./Cargo.lock .

COPY . .

RUN cargo install --path .

ENTRYPOINT ["plasma-simulation"]
CMD ["bash"]
