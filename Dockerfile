FROM rust:latest

WORKDIR /usr/src/vim_bot

COPY . .

RUN cargo build --release

RUN cargo install --path .

CMD ["/usr/local/cargo/bin/vim_bot"]
