FROM rust:1.62-alpine

WORKDIR /usr/src/plasma-simulation
COPY . .

RUN cargo install --path .

CMD ["plasma-simulation"]
